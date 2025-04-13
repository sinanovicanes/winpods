use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};

#[derive(Debug, Default, Clone)]
pub struct GlobalMediaController {
    paused_sessions: Vec<GlobalSystemMediaTransportControlsSession>,
}

impl GlobalMediaController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resume(&mut self) -> windows::core::Result<()> {
        for session in self.paused_sessions.iter() {
            session.TryPlayAsync()?;
        }

        self.paused_sessions.clear();

        Ok(())
    }

    pub fn reset(&mut self) {
        self.paused_sessions.clear();
    }

    pub fn pause(&mut self) -> windows::core::Result<()> {
        let session_manager: GlobalSystemMediaTransportControlsSessionManager =
            GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?;
        let sessions = session_manager.GetSessions()?;

        for session in sessions.into_iter() {
            // Only pause sessions that are currently playing
            if session.GetPlaybackInfo()?.PlaybackStatus()?
                != GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing
            {
                continue;
            }

            if session.TryPauseAsync().is_ok() {
                self.paused_sessions.push(session)
            }
        }

        Ok(())
    }
}
