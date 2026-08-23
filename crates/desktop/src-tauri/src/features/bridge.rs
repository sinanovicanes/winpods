//! Forwards internal events to the webviews.
//!
//! One task owns this, so the rest of the backend never has to know that a UI exists.

use std::sync::Arc;

use tauri::AppHandle;
use tokio::sync::broadcast::error::RecvError;

use crate::state::AppState;

pub fn start(app: &AppHandle, state: &Arc<AppState>) {
    let app = app.clone();
    let mut events = state.subscribe();

    tauri::async_runtime::spawn(async move {
        loop {
            match events.recv().await {
                Ok(event) => event.emit(&app),
                // Advertisements can outpace a stalled bridge. Skipping to the newest event is
                // correct here: every event carries the full current value, so the next one
                // supersedes whatever was missed.
                Err(RecvError::Lagged(skipped)) => {
                    tracing::warn!("UI event bridge fell behind, skipped {skipped} event(s)");
                }
                Err(RecvError::Closed) => break,
            }
        }
    });
}
