use utils::EventDispatcher;
use windows::{
    Devices::Bluetooth::Advertisement::{
        BluetoothLEAdvertisementFilter, BluetoothLEAdvertisementReceivedEventArgs,
        BluetoothLEAdvertisementWatcher, BluetoothLEScanningMode,
    },
    Foundation::TypedEventHandler,
};

use crate::advertisement_received_data::AdvertisementReceivedData;

struct AdvertisementReceivedEvent(AdvertisementReceivedData);

pub struct AdvertisementWatcher {
    watcher: BluetoothLEAdvertisementWatcher,
    dispatcher: EventDispatcher,
}

impl AdvertisementWatcher {
    pub fn new() -> windows::core::Result<Self> {
        let watcher = Self {
            watcher: BluetoothLEAdvertisementWatcher::new()?,
            dispatcher: EventDispatcher::new(),
        };

        watcher.init()?;

        Ok(watcher)
    }

    fn init(&self) -> windows::core::Result<()> {
        let dispatcher = self.dispatcher.clone();
        let _ = self.watcher.Received(&TypedEventHandler::<
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
        }))?;

        Ok(())
    }

    pub fn start(&mut self) -> windows::core::Result<()> {
        self.watcher.Start()?;
        self.watcher
            .SetScanningMode(BluetoothLEScanningMode::Active)?;

        Ok(())
    }

    pub fn stop(&mut self) -> windows::core::Result<()> {
        self.watcher.Stop()?;
        self.watcher
            .SetScanningMode(BluetoothLEScanningMode::None)?;

        Ok(())
    }

    pub fn filter(&mut self, filter: &BluetoothLEAdvertisementFilter) -> windows::core::Result<()> {
        self.watcher.SetAdvertisementFilter(filter)?;

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
}
