use tauri::{
    App, AppHandle, Wry,
    menu::{MenuEvent, MenuItem},
};

pub const AUDIO_PRESETS_MENU_ID: &str = "audio-presets";
pub const MUSIC_MODE_MENU_ID: &str = "audio-preset-music";
pub const GAMING_MODE_MENU_ID: &str = "audio-preset-gaming";
pub const MEETING_MODE_MENU_ID: &str = "audio-preset-meeting";
pub const AIRPODS_OUTPUT_LAPTOP_MIC_MENU_ID: &str = "audio-preset-airpods-output-laptop-mic";

pub fn create_audio_presets_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(
        app,
        AUDIO_PRESETS_MENU_ID,
        "Audio Presets",
        false,
        None::<&str>,
    )
    .unwrap()
}

pub fn create_music_mode_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, MUSIC_MODE_MENU_ID, "Music Mode", true, None::<&str>).unwrap()
}

pub fn create_gaming_mode_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, GAMING_MODE_MENU_ID, "Gaming Mode", true, None::<&str>).unwrap()
}

pub fn create_meeting_mode_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(
        app,
        MEETING_MODE_MENU_ID,
        "Meeting Mode",
        true,
        None::<&str>,
    )
    .unwrap()
}

pub fn create_airpods_output_laptop_mic_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(
        app,
        AIRPODS_OUTPUT_LAPTOP_MIC_MENU_ID,
        "Use AirPods output + laptop mic",
        true,
        None::<&str>,
    )
    .unwrap()
}

pub fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        MUSIC_MODE_MENU_ID => {
            tracing::info!("Music Mode selected: prefer AirPods output and system default mic");
            super::windows_settings::open_sound_settings(app);
        }
        GAMING_MODE_MENU_ID => {
            tracing::info!("Gaming Mode selected: prefer AirPods output and laptop/external mic");
            super::windows_settings::open_sound_settings(app);
        }
        MEETING_MODE_MENU_ID => {
            tracing::info!("Meeting Mode selected: use meeting-friendly input/output settings");
            super::windows_settings::open_sound_settings(app);
        }
        AIRPODS_OUTPUT_LAPTOP_MIC_MENU_ID => {
            tracing::info!("AirPods output + laptop mic selected");
            super::windows_settings::open_sound_settings(app);
        }
        _ => {}
    }
}
