use std::{fmt::Debug, sync::RwLock};

use serde::{Deserialize, Serialize};
use tauri::{App, Emitter, Manager};
use tauri_plugin_store::{Store, StoreExt};
use utils::EventDispatcher;

mod ear_detection;
mod listeners;
mod low_battery_notification;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsState {
    pub auto_update: bool,
    pub low_battery_threshold: u8,
    pub ear_detection: bool,
    #[serde(skip)]
    dispatcher: EventDispatcher,
}

struct OnAutoUpdateChangeEvent(pub bool);
struct OnLowBatteryThresholdChangeEvent(pub u8);
struct OnEarDetectionChangeEvent(pub bool);

impl SettingsState {
    pub fn load_from_store<R: tauri::Runtime>(store: &Store<R>) -> Self {
        let auto_update = store
            .get("auto_update")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let low_battery_threshold = store
            .get("low_battery_threshold")
            .and_then(|v| v.as_u64())
            .unwrap_or(20) as u8;
        let ear_detection = store
            .get("ear_detection")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        Self {
            auto_update,
            low_battery_threshold,
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

    pub fn set_low_battery_threshold(&mut self, new_state: u8) {
        if self.low_battery_threshold == new_state {
            return;
        }
        self.low_battery_threshold = new_state;
        self.dispatcher
            .dispatch(OnLowBatteryThresholdChangeEvent(new_state));
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

    pub fn on_low_battery_threshold_changed(&self, callback: impl Fn(&u8) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<OnLowBatteryThresholdChangeEvent, _>(move |event| {
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
            low_battery_threshold: 20,
            ear_detection: true,
            dispatcher: EventDispatcher::new(),
        }
    }
}

impl Debug for SettingsState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsState")
            .field("auto_update", &self.auto_update)
            .field("low_battery_threshold", &self.low_battery_threshold)
            .field("ear_detection", &self.ear_detection)
            .finish()
    }
}

pub fn init(app: &mut App) {
    let store = app
        .store("settings.json")
        .expect("Failed to get settings store");
    let settings_state = SettingsState::load_from_store(&store);

    let app_handle = app.app_handle().clone();
    settings_state.on_auto_update_changed(move |state| {
        app_handle
            .emit("settings:update:auto_update", state)
            .unwrap();
        let store = app_handle.store("settings.json").unwrap();
        store.set("auto_update", state.clone());
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
    settings_state.on_low_battery_threshold_changed(move |state| {
        app_handle
            .emit("settings:update:low_battery_threshold", state)
            .unwrap();
        let store = app_handle.store("settings.json").unwrap();
        store.set("low_battery_threshold", state.clone());
    });

    app.manage(RwLock::new(settings_state));

    listeners::ui_listeners::init(app);
    ear_detection::init(app);
    low_battery_notification::init(app);
}
