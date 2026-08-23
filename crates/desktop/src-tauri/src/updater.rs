//! Update ownership.
//!
//! Rust is the single owner, for the same reason it owns settings: two owners of one piece of
//! state drift apart. Previously the backend auto-installed on startup while the UI independently
//! offered a manual "update available" button, so with automatic updates enabled both could
//! download and install the *same* update at once and then race to restart the app.
//!
//! Now the UI only reads [`UpdateStatus`] and asks for an install; this service does the work
//! behind a lock that permits one install at a time.

use std::sync::{Mutex, MutexGuard, PoisonError};

use anyhow::{Context, Result};
use serde::Serialize;
use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;
use tokio::sync::broadcast;

use crate::events::AppEvent;

/// Emit at most one progress event per this many percent, so a fast download does not flood the
/// event bus with a hundred updates.
const PROGRESS_STEP: u8 = 5;

/// Everything the UI needs to render the update state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatus {
    /// The running version.
    pub current: String,
    /// The newer version, when one is available.
    pub available: Option<String>,
    /// Whether an install is running right now.
    pub installing: bool,
    /// Download progress as a percentage, when the total size is known.
    pub progress: Option<u8>,
    /// Why the last check or install failed. Cleared by the next successful check.
    pub error: Option<String>,
}

impl UpdateStatus {
    fn new(current: String) -> Self {
        Self {
            current,
            available: None,
            installing: false,
            progress: None,
            error: None,
        }
    }

    /// Whether a newer version is available and not already being installed.
    pub fn is_installable(&self) -> bool {
        self.available.is_some() && !self.installing
    }
}

/// Owns the update state and performs the install.
pub struct UpdateService {
    status: Mutex<UpdateStatus>,
    /// Held for the duration of an install, so a second request is rejected rather than queued.
    install_lock: tokio::sync::Mutex<()>,
    events: broadcast::Sender<AppEvent>,
}

impl UpdateService {
    pub fn new(current_version: String, events: broadcast::Sender<AppEvent>) -> Self {
        Self {
            status: Mutex::new(UpdateStatus::new(current_version)),
            install_lock: tokio::sync::Mutex::new(()),
            events,
        }
    }

    /// The current status.
    pub fn status(&self) -> UpdateStatus {
        lock(&self.status).clone()
    }

    /// Asks the update endpoint whether a newer version exists.
    pub async fn check(&self, app: &AppHandle) -> Result<UpdateStatus> {
        let result = app
            .updater()
            .context("the updater is not configured")?
            .check()
            .await;

        match result {
            Ok(update) => {
                let version = update.map(|update| update.version);

                self.mutate(|status| {
                    status.available = version.clone();
                    status.error = None;
                });

                match &version {
                    Some(version) => tracing::info!("Update {version} is available"),
                    None => tracing::info!("winpods is up to date"),
                }
            }
            Err(e) => {
                // Expected when offline. Recorded so the UI can say so, never fatal.
                tracing::warn!("Update check failed: {e}");
                self.mutate(|status| status.error = Some(e.to_string()));
            }
        }

        Ok(self.status())
    }

    /// Downloads and installs the available update, then restarts.
    ///
    /// Returns without doing anything if an install is already running, which is what makes the
    /// automatic path and a user pressing the button safe to happen at the same time.
    pub async fn install(&self, app: &AppHandle) -> Result<()> {
        let Ok(_guard) = self.install_lock.try_lock() else {
            tracing::info!("An update install is already running, ignoring this request");
            return Ok(());
        };

        self.mutate(|status| {
            status.installing = true;
            status.progress = None;
            status.error = None;
        });

        match self.download_and_install(app).await {
            Ok(()) => Ok(()),
            Err(e) => {
                tracing::error!("Update install failed: {e:#}");
                self.mutate(|status| {
                    status.installing = false;
                    status.progress = None;
                    status.error = Some(format!("{e:#}"));
                });
                Err(e)
            }
        }
    }

    async fn download_and_install(&self, app: &AppHandle) -> Result<()> {
        let update = app
            .updater()
            .context("the updater is not configured")?
            .check()
            .await
            .context("failed to check for an update")?
            .context("no update is available")?;

        tracing::info!("Installing update {}", update.version);

        let mut downloaded = 0usize;
        let mut reported = 0u8;

        update
            .download_and_install(
                |chunk, total| {
                    downloaded += chunk;

                    let Some(total) = total.filter(|total| *total > 0) else {
                        return;
                    };

                    let percent = ((downloaded as u64 * 100) / total).min(100) as u8;

                    if percent < reported.saturating_add(PROGRESS_STEP) && percent != 100 {
                        return;
                    }

                    reported = percent;
                    self.mutate(|status| status.progress = Some(percent));
                },
                || tracing::info!("Download finished, installing"),
            )
            .await
            .context("failed to download and install the update")?;

        tracing::info!("Update installed, restarting");
        app.restart();
    }

    /// Applies a change and broadcasts the new status if it actually changed.
    ///
    /// The lock is a `std::sync::Mutex` on purpose: nothing awaits while holding it, and the
    /// download progress callback is synchronous so it could not await anyway.
    fn mutate(&self, apply: impl FnOnce(&mut UpdateStatus)) {
        let updated = {
            let mut status = lock(&self.status);
            let before = status.clone();
            apply(&mut status);

            if *status == before {
                return;
            }

            status.clone()
        };

        let _ = self.events.send(AppEvent::UpdateStatusChanged(updated));
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
