use utils::EventDispatcher;
use windows::{
    Devices::Bluetooth::Advertisement::{
        BluetoothLEAdvertisementFilter, BluetoothLEAdvertisementReceivedEventArgs,
        BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStatus,
        BluetoothLEAdvertisementWatcherStoppedEventArgs, BluetoothLEScanningMode,
    },
    Foundation::TypedEventHandler,
};

use crate::{Error, Result, advertisement_received_data::AdvertisementReceivedData};

struct AdvertisementStoppedEvent;
struct AdvertisementReceivedEvent(AdvertisementReceivedData);

pub enum AdvertisementWatcherStatus {
    Started,
    Stopped,
}

pub struct AdvertisementWatcher {
    watcher: BluetoothLEAdvertisementWatcher,
    dispatcher: EventDispatcher,
}

impl AdvertisementWatcher {
    pub fn new() -> Result<Self> {
        let watcher = Self {
            watcher: BluetoothLEAdvertisementWatcher::new().map_err(|_| Error::WindowsError)?,
            dispatcher: EventDispatcher::new(),
        };

        watcher.init()?;

        Ok(watcher)
    }

    fn init(&self) -> Result<()> {
        let dispatcher = self.dispatcher.clone();
        let _ = self
            .watcher
            .Received(&TypedEventHandler::<
                BluetoothLEAdvertisementWatcher,
                BluetoothLEAdvertisementReceivedEventArgs,
            >::new(move |_watcher, args| {
                let Some(args) = args.as_ref() else {
                    return Ok(());
                };

                let Ok(data) = AdvertisementReceivedData::try_from(args.clone()) else {
                    return Ok(());
                };

                dispatcher.dispatch(AdvertisementReceivedEvent(data));

                Ok(())
            }))
            .map_err(|_| Error::WindowsError)?;

        let dispatcher = self.dispatcher.clone();
        let _ = self
            .watcher
            .Stopped(&TypedEventHandler::<
                BluetoothLEAdvertisementWatcher,
                BluetoothLEAdvertisementWatcherStoppedEventArgs,
            >::new(move |_watcher, _args| {
                dispatcher.dispatch(AdvertisementStoppedEvent);
                Ok(())
            }))
            .map_err(|_| Error::WindowsError)?;

        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        self.watcher.Start().map_err(|_| Error::WindowsError)?;
        self.watcher
            .SetScanningMode(BluetoothLEScanningMode::Active)
            .map_err(|_| Error::WindowsError)?;

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        self.watcher.Stop().map_err(|_| Error::WindowsError)?;
        self.watcher
            .SetScanningMode(BluetoothLEScanningMode::None)
            .map_err(|_| Error::WindowsError)?;

        Ok(())
    }

    pub fn status(&self) -> AdvertisementWatcherStatus {
        let status = self
            .watcher
            .Status()
            .map_err(|_| Error::WindowsError)
            .unwrap_or_default();

        match status {
            BluetoothLEAdvertisementWatcherStatus::Started => AdvertisementWatcherStatus::Started,
            _ => AdvertisementWatcherStatus::Stopped,
        }
    }

    pub fn filter(&self, filter: &BluetoothLEAdvertisementFilter) -> Result<()> {
        self.watcher
            .SetAdvertisementFilter(filter)
            .map_err(|_| Error::WindowsError)?;

        Ok(())
    }

    pub fn on_received(
        &self,
        callback: impl Fn(&AdvertisementReceivedData) + Send + Sync + 'static,
    ) {
        self.dispatcher
            .add_listener::<AdvertisementReceivedEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_stopped(&self, callback: impl Fn() + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<AdvertisementStoppedEvent, _>(move |_| {
                callback();
            });
    }
}
