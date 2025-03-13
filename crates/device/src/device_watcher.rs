use utils::EventDispatcher;
use windows::{
    Devices::Enumeration::{DeviceInformation, DeviceInformationUpdate},
    Foundation::TypedEventHandler,
};

struct DeviceAddedEvent(windows::Devices::Enumeration::DeviceInformation);
struct DeviceRemovedEvent(windows::Devices::Enumeration::DeviceInformationUpdate);

pub struct DeviceWatcher {
    watcher: windows::Devices::Enumeration::DeviceWatcher,
    dispatcher: EventDispatcher,
}

impl DeviceWatcher {
    pub fn new() -> windows::core::Result<Self> {
        let watcher = DeviceInformation::CreateWatcher()?;
        let device_watcher = Self {
            watcher,
            dispatcher: EventDispatcher::new(),
        };

        device_watcher.init();

        Ok(device_watcher)
    }

    fn init(&self) {
        let dispatcher = self.dispatcher.clone();
        let _ = self.watcher.Added(&TypedEventHandler::<
            windows::Devices::Enumeration::DeviceWatcher,
            DeviceInformation,
        >::new(move |_watcher, device_info| {
            if let Some(device_info) = device_info.as_ref() {
                dispatcher.dispatch(DeviceAddedEvent(device_info.clone()));
            }
            Ok(())
        }));

        let dispatcher = self.dispatcher.clone();
        let _ = self.watcher.Removed(&TypedEventHandler::<
            windows::Devices::Enumeration::DeviceWatcher,
            DeviceInformationUpdate,
        >::new(move |_watcher, device_info| {
            if let Some(device_info) = device_info.as_ref() {
                dispatcher.dispatch(DeviceRemovedEvent(device_info.clone()));
            }
            Ok(())
        }));

        let dispatcher = self.dispatcher.clone();
        let _ = self.watcher.Updated(&TypedEventHandler::<
            windows::Devices::Enumeration::DeviceWatcher,
            DeviceInformationUpdate,
        >::new(move |_watcher, device_info| {
            if let Some(device_info) = device_info.as_ref() {
                dispatcher.dispatch(DeviceRemovedEvent(device_info.clone()));
            }
            Ok(())
        }));
    }

    pub fn start(&self) -> windows::core::Result<()> {
        self.watcher.Start()?;
        Ok(())
    }

    pub fn stop(&self) -> windows::core::Result<()> {
        self.watcher.Stop()?;
        Ok(())
    }

    pub fn on_device_added(&self, callback: impl Fn(&DeviceInformation) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<DeviceAddedEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_device_removed(
        &self,
        callback: impl Fn(&DeviceInformationUpdate) + Send + Sync + 'static,
    ) {
        self.dispatcher
            .add_listener::<DeviceRemovedEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_device_updated(
        &self,
        callback: impl Fn(&DeviceInformationUpdate) + Send + Sync + 'static,
    ) {
        self.dispatcher
            .add_listener::<DeviceRemovedEvent, _>(move |event| {
                callback(&event.0);
            });
    }
}
