use std::sync::{Arc, Mutex};

use windows::{
    Devices::Bluetooth::Advertisement::{
        BluetoothLEAdvertisementFilter, BluetoothLEAdvertisementReceivedEventArgs,
        BluetoothLEAdvertisementWatcher, BluetoothLEScanningMode,
    },
    Foundation::TypedEventHandler,
};

use crate::advertisement_received_data::AdvertisementReceivedData;

enum AdvertisementWatcherState {
    Stopped,
    Scanning,
}

pub struct AdvertisementWatcher {
    state: AdvertisementWatcherState,
    watcher: BluetoothLEAdvertisementWatcher,
    received_token: Option<i64>,
    on_received_callbacks: Arc<Mutex<Vec<Box<dyn Fn(AdvertisementReceivedData) + Send + Sync>>>>,
}

impl AdvertisementWatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        if matches!(self.state, AdvertisementWatcherState::Scanning) {
            return;
        }

        self.state = AdvertisementWatcherState::Scanning;

        let callbacks = Arc::clone(&self.on_received_callbacks);

        let received_token = self
            .watcher
            .Received(&TypedEventHandler::<
                BluetoothLEAdvertisementWatcher,
                BluetoothLEAdvertisementReceivedEventArgs,
            >::new(move |_watcher, args| {
                let args = args.as_ref().unwrap();
                let data = AdvertisementReceivedData::try_from(args.clone()).unwrap();

                let callbacks = callbacks.lock().unwrap();
                for callback in callbacks.iter() {
                    callback(data.clone());
                }

                Ok(())
            }))
            .unwrap();

        self.received_token = Some(received_token);
        self.watcher.Start().unwrap();
        self.watcher
            .SetScanningMode(BluetoothLEScanningMode::Active)
            .unwrap();
    }

    pub fn stop(&mut self) {
        self.state = AdvertisementWatcherState::Stopped;
        self.watcher.Stop().unwrap();
        self.watcher
            .SetScanningMode(BluetoothLEScanningMode::None)
            .unwrap();
        if let Some(token) = self.received_token {
            let _ = self.watcher.RemoveReceived(token);
            self.received_token = None;
        }
    }

    pub fn filter(&mut self, filter: &BluetoothLEAdvertisementFilter) {
        self.watcher.SetAdvertisementFilter(filter).unwrap();
    }

    pub fn on_received<F>(&self, f: F)
    where
        F: Fn(AdvertisementReceivedData) + Send + Sync + 'static,
    {
        self.on_received_callbacks.lock().unwrap().push(Box::new(f));
    }
}

impl Default for AdvertisementWatcher {
    fn default() -> Self {
        Self {
            state: AdvertisementWatcherState::Stopped,
            watcher: BluetoothLEAdvertisementWatcher::new().unwrap(),
            on_received_callbacks: Arc::new(Mutex::new(Vec::new())),
            received_token: None,
        }
    }
}
