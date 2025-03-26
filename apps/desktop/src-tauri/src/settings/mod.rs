use std::{fmt::Debug, sync::Mutex};

use serde::{Deserialize, Serialize};
use tauri::{App, Emitter, Listener, Manager};
use tauri_plugin_store::{Store, StoreExt};
use utils::EventDispatcher;

mod ear_detection;
mod listeners;
mod low_battery_notification;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsState {
    pub auto_update: bool,
    pub notification: bool,
    pub low_battery_notification: bool,
    pub ear_detection: bool,
    #[serde(skip)]
    dispatcher: EventDispatcher,
}

struct OnAutoUpdateChangeEvent(pub bool);
struct OnNotificationChangeEvent(pub bool);
struct OnLowBatteryNotificationChangeEvent(pub bool);
struct OnEarDetectionChangeEvent(pub bool);

impl SettingsState {
    pub fn load_from_store<R: tauri::Runtime>(store: &Store<R>) -> Self {
        let auto_update = store
            .get("auto_update")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let notification = store
            .get("notification")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let low_battery_notification = store
            .get("low_battery_notification")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let ear_detection = store
            .get("ear_detection")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        Self {
            auto_update,
            notification,
            low_battery_notification,
            ear_detection,
            dispatcher: EventDispatcher::new(),
        }
    }
}

impl SettingsState {
    pub fn set_auto_update(&mut self, new_state: bool) {
        if self.auto_update == new_state {
            return;
        }
        self.auto_update = new_state;
        self.dispatcher.dispatch(OnAutoUpdateChangeEvent(new_state));
    }

    pub fn set_notification(&mut self, new_state: bool) {
        if self.notification == new_state {
            return;
        }
        self.notification = new_state;
        self.dispatcher
            .dispatch(OnNotificationChangeEvent(new_state));
    }

    pub fn set_low_battery_notification(&mut self, new_state: bool) {
        if self.low_battery_notification == new_state {
            return;
        }
        self.low_battery_notification = new_state;
        self.dispatcher
            .dispatch(OnLowBatteryNotificationChangeEvent(new_state));
    }

    pub fn set_ear_detection(&mut self, new_state: bool) {
        if self.ear_detection == new_state {
            return;
        }
        self.ear_detection = new_state;
        self.dispatcher
            .dispatch(OnEarDetectionChangeEvent(new_state));
    }

    pub fn on_auto_update_changed(&self, callback: impl Fn(&bool) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<OnAutoUpdateChangeEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_notification_changed(&self, callback: impl Fn(&bool) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<OnNotificationChangeEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_low_battery_notification_changed(
        &self,
        callback: impl Fn(&bool) + Send + Sync + 'static,
    ) {
        self.dispatcher
            .add_listener::<OnLowBatteryNotificationChangeEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_ear_detection_changed(&self, callback: impl Fn(&bool) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<OnEarDetectionChangeEvent, _>(move |event| {
                callback(&event.0);
            });
    }
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            auto_update: true,
            notification: true,
            low_battery_notification: true,
            ear_detection: true,
            dispatcher: EventDispatcher::new(),
        }
    }
}

impl Debug for SettingsState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsState")
            .field("auto_update", &self.auto_update)
            .field("notification", &self.notification)
            .field("low_battery_notification", &self.low_battery_notification)
            .field("ear_detection", &self.ear_detection)
            .finish()
    }
}

pub fn init(app: &mut App) {
    let store = app
        .store("settings.json")
        .expect("Failed to get settings store");
    let settings_state = SettingsState::load_from_store(&store);

    app.listen("store://change", |event| {
        tracing::debug!("{:?}", event);
    });

    let app_handle = app.app_handle().clone();
    settings_state.on_auto_update_changed(move |state| {
        app_handle
            .emit("settings:update:auto_update", state)
            .unwrap();
        let store = app_handle.store("settings.json").unwrap();
        store.set("auto_update", state.clone());
    });

    let app_handle = app.app_handle().clone();
    settings_state.on_notification_changed(move |state| {
        app_handle
            .emit("settings:update:notification", state)
            .unwrap();
        let store = app_handle.store("settings.json").unwrap();
        store.set("notification", state.clone());
    });

    let app_handle = app.app_handle().clone();
    settings_state.on_ear_detection_changed(move |state| {
        app_handle
            .emit("settings:update:ear_detection", state)
            .unwrap();
        let store = app_handle.store("settings.json").unwrap();
        store.set("ear_detection", state.clone());
    });

    let app_handle = app.app_handle().clone();
    settings_state.on_low_battery_notification_changed(move |state| {
        app_handle
            .emit("settings:update:low_battery_notification", state)
            .unwrap();
        let store = app_handle.store("settings.json").unwrap();
        store.set("low_battery_notification", state.clone());
    });

    app.manage(Mutex::new(settings_state));

    listeners::ui_listeners::init(app);
    ear_detection::init(app);
    low_battery_notification::init(app);
}
