use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use utils::EventDispatcher;
use windows::{Devices::Radios::Radio, Foundation::TypedEventHandler};

use super::get_bluetooth_adapter_radio;
use crate::AdapterState;

/// How long to wait before looking for the bluetooth radio again.
/// The delay is doubled on every failed attempt, up to [`RETRY_MAX_INTERVAL`].
const RETRY_INTERVAL: Duration = Duration::from_secs(2);
const RETRY_MAX_INTERVAL: Duration = Duration::from_secs(30);

struct AdapterStateChangedEvent(AdapterState);

pub struct AdapterWatcher {
    dispatcher: EventDispatcher,
    radio: Arc<Mutex<Option<Radio>>>,
    stopped: Arc<AtomicBool>,
}

impl AdapterWatcher {
    pub fn new() -> Self {
        let dispatcher = EventDispatcher::new();

        AdapterWatcher {
            dispatcher,
            radio: Arc::new(Mutex::new(None)),
            stopped: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn state(&self) -> AdapterState {
        match self.radio.lock().unwrap().as_ref() {
            Some(radio) => AdapterState::from(radio),
            // Without a radio we cannot tell the state. The watcher keeps looking for one in the
            // background and dispatches a state changed event as soon as it finds it.
            None => AdapterState::Off,
        }
    }

    pub fn start(&mut self) {
        self.stopped.store(false, Ordering::Relaxed);

        if attach_radio(&self.radio, &self.dispatcher, false) {
            return;
        }

        // The bluetooth stack is not always ready when the app starts (this is common when the app
        // is launched on login), so keep looking for the radio instead of reporting the adapter as
        // off for the rest of the session.
        tracing::warn!("Bluetooth radio is not available yet, retrying in the background");

        let radio = self.radio.clone();
        let dispatcher = self.dispatcher.clone();
        let stopped = self.stopped.clone();

        thread::spawn(move || {
            let mut interval = RETRY_INTERVAL;

            loop {
                thread::sleep(interval);

                // The watcher was dropped or stopped, nothing left to do.
                if stopped.load(Ordering::Relaxed) || Arc::strong_count(&radio) == 1 {
                    return;
                }

                if attach_radio(&radio, &dispatcher, true) {
                    tracing::info!("Bluetooth radio is available again");
                    return;
                }

                interval = (interval * 2).min(RETRY_MAX_INTERVAL);
            }
        });
    }

    pub fn stop(&mut self) {
        self.stopped.store(true, Ordering::Relaxed);
        *self.radio.lock().unwrap() = None;
    }

    pub fn on_state_changed(&self, callback: impl Fn(AdapterState) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<AdapterStateChangedEvent, _>(move |event| {
                callback(event.0);
            });
    }
}

impl Default for AdapterWatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Looks up the bluetooth radio and subscribes to its state changes.
///
/// Returns `false` when no radio could be found. When `dispatch_current_state` is set, the state of
/// the radio is dispatched right away, which is used by the retry loop to let the listeners know the
/// adapter is reachable again.
fn attach_radio(
    slot: &Arc<Mutex<Option<Radio>>>,
    dispatcher: &EventDispatcher,
    dispatch_current_state: bool,
) -> bool {
    let Some(radio) = get_bluetooth_adapter_radio() else {
        return false;
    };

    let state = AdapterState::from(&radio);
    let mut current_state = state;
    let handler_dispatcher = dispatcher.clone();

    if let Err(e) = radio.StateChanged(&TypedEventHandler::<Radio, _>::new(move |radio, _| {
        let Some(radio) = radio.as_ref() else {
            return Ok(());
        };

        let new_state = AdapterState::from(radio);

        if new_state == current_state {
            return Ok(());
        }

        current_state = new_state;
        handler_dispatcher.dispatch(AdapterStateChangedEvent(new_state));
        Ok(())
    })) {
        tracing::error!("Failed to listen for bluetooth radio state changes: {}", e);
    }

    *slot.lock().unwrap() = Some(radio);

    if dispatch_current_state {
        dispatcher.dispatch(AdapterStateChangedEvent(state));
    }

    true
}
