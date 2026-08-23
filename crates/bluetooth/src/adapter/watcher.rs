use std::sync::{
    Arc, Mutex, MutexGuard, PoisonError,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use tokio::{sync::broadcast, task::JoinHandle};
use windows::{Devices::Radios::Radio, Foundation::TypedEventHandler, core::IInspectable};

use super::{AdapterState, bluetooth_radio};

/// How long to wait before looking for the bluetooth radio again. Doubles on every failed
/// attempt, up to [`RETRY_MAX_INTERVAL`].
const RETRY_INTERVAL: Duration = Duration::from_secs(2);
const RETRY_MAX_INTERVAL: Duration = Duration::from_secs(30);

/// Enough room for a subscriber that is briefly busy. State changes are rare, so a slow
/// subscriber lagging past this has bigger problems than a missed event.
const EVENT_CAPACITY: usize = 16;

/// Watches the bluetooth radio and reports every state change.
///
/// The radio is not always present when the app starts -- most visibly when the app is launched
/// on login, where the bluetooth stack can come up seconds later. Rather than reporting
/// "bluetooth is off" for the rest of the session, the watcher keeps looking for the radio in the
/// background and announces it as soon as it appears.
pub struct AdapterWatcher {
    shared: Arc<Shared>,
    retry: Mutex<Option<JoinHandle<()>>>,
}

struct Shared {
    events: broadcast::Sender<AdapterState>,
    /// Cached state, so [`AdapterWatcher::state`] can answer a command without a WinRT round trip.
    powered: AtomicBool,
    subscription: Mutex<Option<RadioSubscription>>,
}

/// A radio with its `StateChanged` handler registered.
///
/// Dropping it unregisters the handler. The previous implementation never did, so every reattach
/// leaked a handler onto the radio.
struct RadioSubscription {
    radio: Radio,
    token: i64,
}

impl Drop for RadioSubscription {
    fn drop(&mut self) {
        if let Err(e) = self.radio.RemoveStateChanged(self.token) {
            tracing::warn!("Failed to remove the radio state changed handler: {e}");
        }
    }
}

impl AdapterWatcher {
    pub fn new() -> Self {
        let (events, _) = broadcast::channel(EVENT_CAPACITY);

        Self {
            shared: Arc::new(Shared {
                events,
                powered: AtomicBool::new(false),
                subscription: Mutex::new(None),
            }),
            retry: Mutex::new(None),
        }
    }

    /// Subscribes to adapter state changes.
    ///
    /// Only *changes* are delivered. Read [`Self::state`] for the current value.
    pub fn subscribe(&self) -> broadcast::Receiver<AdapterState> {
        self.shared.events.subscribe()
    }

    /// The last known adapter state.
    ///
    /// Synchronous and cheap. Before [`Self::start`] has found a radio this reports
    /// [`AdapterState::Off`], because without a radio the state genuinely cannot be determined.
    pub fn state(&self) -> AdapterState {
        AdapterState::from_powered(self.shared.powered.load(Ordering::Relaxed))
    }

    /// Attaches to the radio, retrying in the background for as long as it takes.
    pub async fn start(&self) {
        self.stop();

        if self.shared.attach(false).await {
            return;
        }

        tracing::warn!("Bluetooth radio is not available yet, retrying in the background");

        let shared = Arc::clone(&self.shared);
        let handle = tokio::spawn(async move {
            let mut interval = RETRY_INTERVAL;

            loop {
                tokio::time::sleep(interval).await;

                if shared.attach(true).await {
                    tracing::info!("Bluetooth radio is available again");
                    return;
                }

                interval = (interval * 2).min(RETRY_MAX_INTERVAL);
            }
        });

        *lock(&self.retry) = Some(handle);
    }

    /// Stops watching and releases the radio.
    pub fn stop(&self) {
        if let Some(handle) = lock(&self.retry).take() {
            handle.abort();
        }

        // Dropping the subscription unregisters the WinRT handler.
        *lock(&self.shared.subscription) = None;
        self.shared.powered.store(false, Ordering::Relaxed);
    }
}

impl Shared {
    /// Looks up the radio and subscribes to its state changes.
    ///
    /// Returns `false` when no radio could be found. When `announce` is set the current state is
    /// broadcast right away, which is how the retry loop tells subscribers that the adapter
    /// became reachable again.
    async fn attach(self: &Arc<Self>, announce: bool) -> bool {
        let Ok(radio) = bluetooth_radio().await else {
            return false;
        };

        let state = AdapterState::from(&radio);
        self.powered.store(state.is_on(), Ordering::Relaxed);

        let shared = Arc::clone(self);
        let handler = TypedEventHandler::<Radio, IInspectable>::new(move |radio, _| {
            let Some(radio) = radio.as_ref() else {
                return Ok(());
            };

            let state = AdapterState::from(radio);

            // `swap` both records the new state and tells us whether it actually changed, so
            // repeated notifications for the same state stay off the channel.
            if shared.powered.swap(state.is_on(), Ordering::Relaxed) == state.is_on() {
                return Ok(());
            }

            tracing::info!("Bluetooth adapter state changed: {state:?}");
            let _ = shared.events.send(state);
            Ok(())
        });

        match radio.StateChanged(&handler) {
            Ok(token) => *lock(&self.subscription) = Some(RadioSubscription { radio, token }),
            Err(e) => {
                tracing::error!("Failed to listen for bluetooth radio state changes: {e}");
                return false;
            }
        }

        if announce {
            let _ = self.events.send(state);
        }

        true
    }
}

impl Default for AdapterWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AdapterWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Locks a mutex, recovering from poisoning.
///
/// The only data behind these mutexes is a handle slot. A panic elsewhere while holding the lock
/// cannot leave that in a state worth propagating a panic over.
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
