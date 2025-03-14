use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

pub fn play() -> bool {
    let session_manager: GlobalSystemMediaTransportControlsSessionManager =
        GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .unwrap()
            .get()
            .unwrap();

    // Get the current active session
    if let Some(session) = session_manager.GetCurrentSession().ok() {
        // Start media playback
        session.TryPlayAsync().unwrap();
        return true;
    }

    return false;
}

pub fn pause() -> bool {
    let session_manager: GlobalSystemMediaTransportControlsSessionManager =
        GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .unwrap()
            .get()
            .unwrap();

    // Get the current active session
    if let Some(session) = session_manager.GetCurrentSession().ok() {
        // Stop media playback
        session.TryPauseAsync().unwrap();
        return true;
    }

    return false;
}
