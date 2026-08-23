use std::fmt;

use serde::{Serialize, Serializer};

/// The error type every Tauri command returns.
///
/// Tauri requires a `Serialize` error, and `anyhow::Error` deliberately does not implement it.
/// This wrapper bridges the two: internally the full `anyhow` context chain is preserved for the
/// log, and on the way to the webview it collapses into a single readable string.
#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `{:#}` renders the whole context chain: "failed to select device: no paired device".
        write!(f, "{:#}", self.0)
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Log here rather than at every call site: a command failing is always worth a log line,
        // and this is the one place every failure passes through.
        tracing::error!("Command failed: {:#}", self.0);
        serializer.serialize_str(&self.to_string())
    }
}

pub type CommandResult<T> = Result<T, AppError>;
