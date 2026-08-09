use std::{fs::File, path::PathBuf};

use tracing_subscriber::{EnvFilter, fmt, fmt::writer::BoxMakeWriter, prelude::*};

/// Has to match `identifier` in `tauri.conf.json` so the logs end up next to the app data.
const APP_IDENTIFIER: &str = "com.winpods.app";
const LOG_FILE_NAME: &str = "winpods.log";

/// Initializes the global tracing subscriber.
///
/// Release builds are windowed, so stdout goes nowhere. Without a log file there is no way for
/// users to tell us what went wrong, so we always write the logs next to the app data as well.
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(if cfg!(debug_assertions) {
            "debug"
        } else {
            "info"
        })
    });

    let file_writer = match create_log_file() {
        Some(file) => BoxMakeWriter::new(file),
        None => BoxMakeWriter::new(std::io::sink),
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .with(fmt::layer().with_ansi(false).with_writer(file_writer))
        .init();
}

/// Creates (or truncates) the log file of this run.
fn create_log_file() -> Option<File> {
    let path = log_file_path()?;

    match File::create(&path) {
        Ok(file) => Some(file),
        Err(e) => {
            eprintln!("Failed to create the log file at {}: {}", path.display(), e);
            None
        }
    }
}

/// Returns the path of the log file, creating the directory it lives in if needed.
fn log_file_path() -> Option<PathBuf> {
    let mut path = local_data_dir()?;
    path.push(APP_IDENTIFIER);

    if let Err(e) = std::fs::create_dir_all(&path) {
        eprintln!(
            "Failed to create the log directory at {}: {}",
            path.display(),
            e
        );
        return None;
    }

    path.push(LOG_FILE_NAME);

    Some(path)
}

#[cfg(windows)]
fn local_data_dir() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
}

#[cfg(not(windows))]
fn local_data_dir() -> Option<PathBuf> {
    Some(std::env::temp_dir())
}
