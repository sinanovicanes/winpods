//! Pausing and resuming Windows media playback.
//!
//! Used for AirPods ear detection: when a bud leaves an ear, whatever is playing gets paused, and
//! putting it back resumes exactly the sessions that were paused -- not everything that happens
//! to be paused at that moment.
//!
//! The Windows API behind this (`GlobalSystemMediaTransportControlsSessionManager`) reports the
//! media sessions of every app that registered transport controls: browsers, Spotify, and so on.

use anyhow::{Context, Result};
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};

/// Pauses and resumes the media sessions of this machine.
///
/// Remembers which sessions *it* paused so [`Self::resume`] does not start playback in an app the
/// user had paused themselves.
#[derive(Debug, Default)]
pub struct MediaController {
    paused_sessions: Vec<GlobalSystemMediaTransportControlsSession>,
}

impl MediaController {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether this controller currently holds sessions it paused.
    pub fn has_paused_sessions(&self) -> bool {
        !self.paused_sessions.is_empty()
    }

    /// Pauses every session that is currently playing, remembering each one.
    ///
    /// Sessions already paused by the user are left alone, so resuming will not start them.
    pub async fn pause(&mut self) -> Result<()> {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?
            .await
            .context("failed to get the system media session manager")?;
        // Collect the playing sessions before awaiting anything. WinRT collection iterators
        // (`IIterator<T>`) are not `Send`, so one held across an `await` makes this whole future
        // non-`Send` -- and Tauri requires the futures it spawns to be `Send`.
        let playing: Vec<GlobalSystemMediaTransportControlsSession> = {
            let sessions = manager
                .GetSessions()
                .context("failed to list the system media sessions")?;

            sessions.into_iter().filter(is_playing).collect()
        };

        for session in playing {
            match session.TryPauseAsync() {
                // The operation is awaited so a session that refuses to pause is not recorded as
                // paused -- otherwise resuming would start something that never stopped.
                Ok(operation) => match operation.await {
                    Ok(true) => self.paused_sessions.push(session),
                    Ok(false) => tracing::debug!("A media session declined to pause"),
                    Err(e) => tracing::debug!("Failed to pause a media session: {e}"),
                },
                Err(e) => tracing::debug!("Failed to request a media session pause: {e}"),
            }
        }

        tracing::info!("Paused {} media session(s)", self.paused_sessions.len());

        Ok(())
    }

    /// Resumes the sessions this controller paused, then forgets them.
    pub async fn resume(&mut self) -> Result<()> {
        let sessions = std::mem::take(&mut self.paused_sessions);
        let count = sessions.len();

        for session in sessions {
            match session.TryPlayAsync() {
                Ok(operation) => {
                    if let Err(e) = operation.await {
                        tracing::debug!("Failed to resume a media session: {e}");
                    }
                }
                Err(e) => tracing::debug!("Failed to request a media session resume: {e}"),
            }
        }

        tracing::info!("Resumed {count} media session(s)");

        Ok(())
    }

    /// Forgets the paused sessions without resuming them.
    ///
    /// Used when the device disconnects: the sessions are no longer ours to resume, and holding
    /// the references would keep them alive for nothing.
    pub fn reset(&mut self) {
        self.paused_sessions.clear();
    }
}

fn is_playing(session: &GlobalSystemMediaTransportControlsSession) -> bool {
    session
        .GetPlaybackInfo()
        .and_then(|info| info.PlaybackStatus())
        .is_ok_and(|status| {
            status == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing
        })
}
