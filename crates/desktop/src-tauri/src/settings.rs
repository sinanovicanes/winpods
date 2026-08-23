//! Settings ownership.
//!
//! Rust is the single source of truth. The UI reads with `get_settings` and writes with
//! `update_settings`; every accepted change is persisted and then broadcast, so both windows and
//! every feature see the same value.
//!
//! The previous design split ownership between the `tauri-plugin-store` file and the Rust state,
//! reconciling them through untyped global events (`settings:set:auto_start` carrying a string
//! parsed with `parse::<bool>()`). A rejected or malformed write was indistinguishable from a
//! successful one, because `emit` has no return value.

use std::path::PathBuf;

use anyhow::Result;
use tokio::sync::{RwLock, broadcast};
use winpods_core::{Settings, SettingsPatch, SettingsStore};

use crate::events::AppEvent;

/// Owns the settings and keeps the file in step with them.
pub struct SettingsService {
    store: SettingsStore,
    current: RwLock<Settings>,
    events: broadcast::Sender<AppEvent>,
}

impl SettingsService {
    /// Loads the settings from `path`, falling back to defaults.
    pub fn new(path: PathBuf, events: broadcast::Sender<AppEvent>) -> Self {
        let store = SettingsStore::new(path);
        let current = store.load();

        tracing::info!("Loaded settings: {current:?}");

        Self {
            store,
            current: RwLock::new(current),
            events,
        }
    }

    /// The current settings.
    pub async fn get(&self) -> Settings {
        *self.current.read().await
    }

    /// Applies a partial update, persists it, and broadcasts the result.
    ///
    /// A patch that changes nothing returns early without touching the disk or waking any
    /// subscriber. Persisting is best effort: a settings file that cannot be written should not
    /// stop the change from taking effect for this session, so the failure is logged and the new
    /// value is still returned.
    pub async fn update(&self, patch: SettingsPatch) -> Result<Settings> {
        let mut current = self.current.write().await;

        if !current.apply(patch) {
            return Ok(*current);
        }

        let updated = *current;
        // Release the write lock before broadcasting: subscribers react by reading the settings,
        // and holding the lock across that would deadlock them against this task.
        drop(current);

        tracing::info!("Settings changed: {updated:?}");

        if let Err(e) = self.store.save(&updated) {
            tracing::error!("Settings changed but could not be saved: {e:#}");
        }

        let _ = self.events.send(AppEvent::SettingsChanged(updated));

        Ok(updated)
    }
}
