use tauri::{
    App, AppHandle, Wry,
    menu::{Menu, MenuEvent},
};

pub mod airpods_actions;
mod audio_presets;
mod dashboard;
mod quit;
mod windows_settings;

pub fn create_tray_menu(app: &App) -> Menu<Wry> {
    let status = airpods_actions::create_status_item(app);
    let left_battery = airpods_actions::create_left_battery_item(app);
    let right_battery = airpods_actions::create_right_battery_item(app);
    let case_battery = airpods_actions::create_case_battery_item(app);
    let dashboard = dashboard::create_menu_item(app);
    let refresh_battery = airpods_actions::create_refresh_battery_item(app);
    let switch_to_pc = airpods_actions::create_switch_to_pc_item(app);
    let reconnect = airpods_actions::create_reconnect_item(app);
    let audio_presets = audio_presets::create_audio_presets_item(app);
    let music_mode = audio_presets::create_music_mode_item(app);
    let gaming_mode = audio_presets::create_gaming_mode_item(app);
    let meeting_mode = audio_presets::create_meeting_mode_item(app);
    let airpods_output_laptop_mic = audio_presets::create_airpods_output_laptop_mic_item(app);
    let bluetooth_settings = windows_settings::create_bluetooth_settings_item(app);
    let sound_settings = windows_settings::create_sound_settings_item(app);
    let quit = quit::create_menu_item(app);

    Menu::with_items(
        app,
        &[
            &status,
            &left_battery,
            &right_battery,
            &case_battery,
            &dashboard,
            &refresh_battery,
            &switch_to_pc,
            &reconnect,
            &audio_presets,
            &music_mode,
            &gaming_mode,
            &meeting_mode,
            &airpods_output_laptop_mic,
            &bluetooth_settings,
            &sound_settings,
            &quit,
        ],
    )
    .expect("Failed to create tray menu")
}

pub fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        quit::MENU_ID => quit::on_menu_event(app, event),
        dashboard::MENU_ID => dashboard::on_menu_event(app, event),
        airpods_actions::REFRESH_BATTERY_MENU_ID
        | airpods_actions::SWITCH_TO_PC_MENU_ID
        | airpods_actions::RECONNECT_MENU_ID => airpods_actions::on_menu_event(app, event),
        audio_presets::MUSIC_MODE_MENU_ID
        | audio_presets::GAMING_MODE_MENU_ID
        | audio_presets::MEETING_MODE_MENU_ID
        | audio_presets::AIRPODS_OUTPUT_LAPTOP_MIC_MENU_ID => {
            audio_presets::on_menu_event(app, event)
        }
        windows_settings::BLUETOOTH_SETTINGS_MENU_ID | windows_settings::SOUND_SETTINGS_MENU_ID => {
            windows_settings::on_menu_event(app, event)
        }
        _ => {
            tracing::warn!("Unknown menu item: {:?}", event.id);
        }
    }
}
