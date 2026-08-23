//! Background behaviour built on top of the event bus.
//!
//! Each feature is a task that subscribes to [`crate::events::AppEvent`] and acts on what it
//! cares about. They are started once during setup and run for the lifetime of the app.

pub mod autostart;
pub mod bridge;
pub mod ear_detection;
pub mod low_battery;
pub mod updater;

use std::sync::Arc;

use tauri::AppHandle;

use crate::state::AppState;

/// Starts every background feature.
pub fn start_all(app: &AppHandle, state: &Arc<AppState>) {
    bridge::start(app, state);
    autostart::start(app, state);
    ear_detection::start(state);
    low_battery::start(app, state);
    updater::start(app, state);
}
