use std::sync::{
    Arc, Mutex, MutexGuard, PoisonError,
    atomic::{AtomicBool, Ordering},
};

use anyhow::{Context, Result};
use tokio::sync::broadcast;
use windows::{
    Devices::Bluetooth::Advertisement::{
        BluetoothLEAdvertisementReceivedEventArgs, BluetoothLEAdvertisementWatcher,
        BluetoothLEAdvertisementWatcherStatus, BluetoothLEAdvertisementWatcherStoppedEventArgs,
    },
    Foundation::TypedEventHandler,
};

use super::Advertisement;

/// Advertisements arrive continuously, so a lagging subscriber is expected and harmless -- it
/// simply picks up at the newest reading. The capacity only needs to cover a brief stall.
const EVENT_CAPACITY: usize = 64;

/// Watches BLE advertisements and broadcasts them.
///
/// Windows delivers advertisements on its own threadpool. The handler does the minimum amount of
/// work -- optionally filter by company id, copy the payload, send -- and never blocks.
pub struct AdvertisementWatcher {
    watcher: BluetoothLEAdvertisementWatcher,
    events: broadcast::Sender<Advertisement>,
    tokens: Mutex<Option<Tokens>>,
    running: Arc<AtomicBool>,
}

struct Tokens {
    received: i64,
    stopped: i64,
}

impl AdvertisementWatcher {
    /// Watches every advertisement in range.
    pub fn new() -> Result<Self> {
        Self::build(None)
    }

    /// Watches only advertisements carrying manufacturer data for `company_id`.
    ///
    /// Prefer this over [`Self::new`] when a single vendor is of interest; see
    /// [`Advertisement::from_args`] for why.
    pub fn for_company(company_id: u16) -> Result<Self> {
        Self::build(Some(company_id))
    }

    fn build(company_filter: Option<u16>) -> Result<Self> {
        crate::com::ensure_mta();

        let watcher = BluetoothLEAdvertisementWatcher::new()
            .context("failed to create the BLE advertisement watcher")?;
        let (events, _) = broadcast::channel(EVENT_CAPACITY);
        let running = Arc::new(AtomicBool::new(false));

        let handler_events = events.clone();
        let received = watcher
            .Received(&TypedEventHandler::<
                BluetoothLEAdvertisementWatcher,
                BluetoothLEAdvertisementReceivedEventArgs,
            >::new(move |_watcher, args| {
                let Some(args) = args.as_ref() else {
                    return Ok(());
                };

                // Nothing is subscribed yet, so skip the parse entirely.
                if handler_events.receiver_count() == 0 {
                    return Ok(());
                }

                match Advertisement::from_args(args, company_filter) {
                    Ok(advertisement) => {
                        let _ = handler_events.send(advertisement);
                    }
                    Err(e) => tracing::debug!("Skipped an unreadable advertisement: {e:#}"),
                }

                Ok(())
            }))
            .context("failed to listen for BLE advertisements")?;

        let handler_running = Arc::clone(&running);
        let stopped = watcher
            .Stopped(&TypedEventHandler::<
                BluetoothLEAdvertisementWatcher,
                BluetoothLEAdvertisementWatcherStoppedEventArgs,
            >::new(move |_watcher, args| {
                // Windows stops the watcher on its own when the radio goes away. Recording that
                // keeps `is_running` honest so the app does not think it is still scanning.
                handler_running.store(false, Ordering::Relaxed);

                let error = args.as_ref().and_then(|args| args.Error().ok());
                tracing::info!("BLE advertisement watcher stopped (error: {error:?})");
                Ok(())
            }))
            .context("failed to listen for BLE advertisement watcher shutdown")?;

        Ok(Self {
            watcher,
            events,
            tokens: Mutex::new(Some(Tokens { received, stopped })),
            running,
        })
    }

    /// Subscribes to received advertisements.
    pub fn subscribe(&self) -> broadcast::Receiver<Advertisement> {
        self.events.subscribe()
    }

    /// Starts scanning. Starting an already running watcher is a no-op.
    pub fn start(&self) -> Result<()> {
        if self.is_running() {
            return Ok(());
        }

        // Windows raises `E_INVALIDARG` when a watcher is started twice, and the status can have
        // moved on since `is_running` above, so check it against WinRT as well.
        if self.status() == BluetoothLEAdvertisementWatcherStatus::Started {
            self.running.store(true, Ordering::Relaxed);
            return Ok(());
        }

        self.watcher
            .Start()
            .context("failed to start the BLE advertisement watcher")?;
        self.running.store(true, Ordering::Relaxed);
        tracing::info!("BLE advertisement watcher started");

        Ok(())
    }

    /// Stops scanning. Stopping an already stopped watcher is a no-op.
    pub fn stop(&self) -> Result<()> {
        if self.status() == BluetoothLEAdvertisementWatcherStatus::Stopped {
            self.running.store(false, Ordering::Relaxed);
            return Ok(());
        }

        self.watcher
            .Stop()
            .context("failed to stop the BLE advertisement watcher")?;
        self.running.store(false, Ordering::Relaxed);

        Ok(())
    }

    /// Whether the watcher is currently scanning.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    fn status(&self) -> BluetoothLEAdvertisementWatcherStatus {
        self.watcher
            .Status()
            .unwrap_or(BluetoothLEAdvertisementWatcherStatus::Stopped)
    }
}

impl Drop for AdvertisementWatcher {
    fn drop(&mut self) {
        let _ = self.stop();

        // Unregister the handlers, otherwise they outlive the watcher and keep the captured
        // channel sender alive.
        if let Some(tokens) = lock(&self.tokens).take() {
            let _ = self.watcher.RemoveReceived(tokens.received);
            let _ = self.watcher.RemoveStopped(tokens.stopped);
        }
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
