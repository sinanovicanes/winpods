use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
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

    pub fn pause(&mut self) -> windows::core::Result<()> {
        let session_manager: GlobalSystemMediaTransportControlsSessionManager =
            GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?;
        let sessions = session_manager.GetSessions()?;

        println!("Size of sessions: {:?}", sessions.Size());

        for session in sessions.into_iter() {
            match session.TryPauseAsync() {
                Ok(_) => self.paused_sessions.push(session),
                Err(_) => (),
            }
        }

        Ok(())
    }
}
