//! Tracing setup.
//!
//! Release builds are windowed, so stdout goes nowhere and a log file is the only way a user can
//! tell us what went wrong. Logs are written next to the app data with daily rotation, which
//! replaces the previous behaviour of truncating a single file on every launch -- that discarded
//! the evidence whenever the app was restarted before the problem could be reported.

use std::path::PathBuf;

use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Must match `identifier` in `tauri.conf.json` so the logs sit next to the app data.
const APP_IDENTIFIER: &str = "com.winpods.app";
const LOG_DIRECTORY: &str = "logs";
const LOG_FILE_PREFIX: &str = "winpods";

/// Initializes the global subscriber.
///
/// The returned guard flushes the non-blocking file writer when dropped; drop it only at the very
/// end of `main`, otherwise the final log lines never reach disk.
#[must_use = "dropping the guard stops the log file from being flushed"]
pub fn init() -> Option<WorkerGuard> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(if cfg!(debug_assertions) {
            "debug"
        } else {
            // Keep the app's own crates verbose while silencing dependency noise.
            "info,winpods=debug,winpods_bluetooth=debug,winpods_core=debug,winpods_media=debug"
        })
    });

    let console = fmt::layer().with_target(cfg!(debug_assertions));
    let registry = tracing_subscriber::registry().with(filter).with(console);

    let Some(directory) = log_directory() else {
        registry.init();
        tracing::warn!("No log directory available, logging to stdout only");
        return None;
    };

    let (writer, guard) = tracing_appender::non_blocking(rolling::daily(
        &directory,
        format!("{LOG_FILE_PREFIX}.log"),
    ));

    registry
        .with(fmt::layer().with_ansi(false).with_writer(writer))
        .init();

    tracing::info!("Logging to {}", directory.display());

    Some(guard)
}

/// Returns the directory the log files live in, creating it if needed.
fn log_directory() -> Option<PathBuf> {
    let mut path = local_data_dir()?;
    path.push(APP_IDENTIFIER);
    path.push(LOG_DIRECTORY);

    if let Err(e) = std::fs::create_dir_all(&path) {
        eprintln!(
            "Failed to create the log directory at {}: {e}",
            path.display()
        );
        return None;
    }

    Some(path)
}

#[cfg(windows)]
fn local_data_dir() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
}

#[cfg(not(windows))]
fn local_data_dir() -> Option<PathBuf> {
    // Only reached by `cargo check`/clippy on a non-Windows host; the app itself is Windows only.
    Some(std::env::temp_dir())
}
