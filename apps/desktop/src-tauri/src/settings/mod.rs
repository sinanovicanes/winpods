use std::{fmt::Debug, sync::Mutex};

use serde::{Deserialize, Serialize};
use tauri::{App, Emitter, Manager};
use utils::EventDispatcher;

mod ear_detection;
mod listeners;
mod low_battery_notification;

#[derive(Clone, Serialize, Deserialize)]
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
    let settings_state = SettingsState::default();

    let app_handle = app.app_handle().clone();
    settings_state.on_auto_update_changed(move |state| {
        app_handle
            .emit("settings:update:auto_update", state)
            .unwrap();
    });

    let app_handle = app.app_handle().clone();
    settings_state.on_notification_changed(move |state| {
        app_handle
            .emit("settings:update:notification", state)
            .unwrap();
    });

    let app_handle = app.app_handle().clone();
    settings_state.on_ear_detection_changed(move |state| {
        app_handle
            .emit("settings:update:ear_detection", state)
            .unwrap();
    });

    let app_handle = app.app_handle().clone();
    settings_state.on_low_battery_notification_changed(move |state| {
        app_handle
            .emit("settings:update:low_battery_notification", state)
            .unwrap();
    });

    app.manage(Mutex::new(settings_state));

    listeners::ui_listeners::init(app);
    ear_detection::init(app);
    low_battery_notification::init(app);
}
