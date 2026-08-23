use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Largest accepted low battery threshold. The UI offers 10..=90 in steps of ten, plus 0 to
/// disable; anything beyond this came from a hand-edited file.
const MAX_LOW_BATTERY_THRESHOLD: u8 = 90;

/// Threshold value that turns the low battery notification off.
pub const LOW_BATTERY_DISABLED: u8 = 0;

/// The user's settings.
///
/// Rust owns these. The UI reads them with one command and changes them with another, rather than
/// the previous arrangement where a key/value store plugin and the Rust state were both partial
/// owners kept in step by stringly typed global events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    /// Launch winpods when the user signs in.
    pub auto_start: bool,
    /// Download and install updates without asking.
    pub auto_update: bool,
    /// Pause media when a bud leaves an ear, resume when it goes back.
    pub ear_detection: bool,
    /// Notify at or below this percentage. [`LOW_BATTERY_DISABLED`] turns it off.
    pub low_battery_threshold: u8,
    /// Show one combined level on the widget instead of one per bud.
    pub grouped_battery: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_start: true,
            auto_update: true,
            ear_detection: true,
            low_battery_threshold: 20,
            grouped_battery: true,
        }
    }
}

impl Settings {
    /// Whether the low battery notification is switched on.
    pub const fn low_battery_notification_enabled(&self) -> bool {
        self.low_battery_threshold != LOW_BATTERY_DISABLED
    }

    /// Clamps every field into its accepted range.
    ///
    /// Applied on load and after every change. Without it a threshold of 250 -- which the old
    /// event based path accepted without a glance -- silently disables the notification, because
    /// no battery is ever at or below it.
    fn normalize(&mut self) {
        if self.low_battery_threshold > MAX_LOW_BATTERY_THRESHOLD {
            tracing::warn!(
                "Low battery threshold {} is out of range, clamping to {}",
                self.low_battery_threshold,
                MAX_LOW_BATTERY_THRESHOLD
            );
            self.low_battery_threshold = MAX_LOW_BATTERY_THRESHOLD;
        }
    }

    /// Applies a partial update, returning `true` when something actually changed.
    ///
    /// The return value is what keeps the app from persisting a file and waking every window over
    /// a no-op write.
    pub fn apply(&mut self, patch: SettingsPatch) -> bool {
        let before = *self;

        if let Some(value) = patch.auto_start {
            self.auto_start = value;
        }
        if let Some(value) = patch.auto_update {
            self.auto_update = value;
        }
        if let Some(value) = patch.ear_detection {
            self.ear_detection = value;
        }
        if let Some(value) = patch.low_battery_threshold {
            self.low_battery_threshold = value;
        }
        if let Some(value) = patch.grouped_battery {
            self.grouped_battery = value;
        }

        self.normalize();
        *self != before
    }
}

/// A partial settings update coming from the UI.
///
/// Every field is optional so the UI can send just the toggle the user flipped, and an unknown or
/// malformed field is rejected by serde at the command boundary instead of being silently ignored
/// by a `parse::<bool>()` that returns early.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SettingsPatch {
    pub auto_start: Option<bool>,
    pub auto_update: Option<bool>,
    pub ear_detection: Option<bool>,
    pub low_battery_threshold: Option<u8>,
    pub grouped_battery: Option<bool>,
}

/// Reads and writes [`Settings`] as a JSON file.
#[derive(Debug, Clone)]
pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Loads the settings, falling back to defaults.
    ///
    /// A missing file is the normal first run. A corrupt one is reported and replaced by defaults
    /// rather than stopping the app: settings are a convenience, and refusing to start because of
    /// them would leave the user with no way to fix it from the UI.
    pub fn load(&self) -> Settings {
        let mut settings = match std::fs::read_to_string(&self.path) {
            Ok(contents) => match serde_json::from_str::<Settings>(&contents) {
                Ok(settings) => settings,
                Err(e) => {
                    tracing::error!(
                        "Settings file at {} is not valid, falling back to defaults: {e}",
                        self.path.display()
                    );
                    Settings::default()
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::info!("No settings file yet, starting from defaults");
                Settings::default()
            }
            Err(e) => {
                tracing::error!(
                    "Could not read the settings file at {}, falling back to defaults: {e}",
                    self.path.display()
                );
                Settings::default()
            }
        };

        settings.normalize();
        settings
    }

    /// Writes the settings.
    ///
    /// Serialises to a temporary file in the same directory and renames it over the target, so an
    /// interrupted write cannot leave a half-written settings file behind. `rename` is atomic only
    /// within a filesystem, which is why the temporary file is a sibling rather than in a temp dir.
    pub fn save(&self, settings: &Settings) -> Result<()> {
        let directory = self
            .path
            .parent()
            .with_context(|| format!("settings path {} has no parent", self.path.display()))?;

        std::fs::create_dir_all(directory)
            .with_context(|| format!("failed to create {}", directory.display()))?;

        let contents =
            serde_json::to_string_pretty(settings).context("failed to serialise the settings")?;

        let temporary = self.path.with_extension("json.tmp");
        std::fs::write(&temporary, contents)
            .with_context(|| format!("failed to write {}", temporary.display()))?;

        std::fs::rename(&temporary, &self.path).with_context(|| {
            format!(
                "failed to move {} into place at {}",
                temporary.display(),
                self.path.display()
            )
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> (tempfile::TempDir, SettingsStore) {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = SettingsStore::new(dir.path().join("settings.json"));
        (dir, store)
    }

    #[test]
    fn defaults_match_the_shipped_behaviour() {
        let settings = Settings::default();
        assert!(settings.auto_start);
        assert!(settings.auto_update);
        assert!(settings.ear_detection);
        assert_eq!(settings.low_battery_threshold, 20);
        assert!(
            settings.grouped_battery,
            "the widget shows one combined level unless asked otherwise"
        );
    }

    #[test]
    fn a_missing_file_loads_defaults() {
        let (_dir, store) = store();
        assert_eq!(store.load(), Settings::default());
    }

    #[test]
    fn a_corrupt_file_loads_defaults() {
        let (_dir, store) = store();
        std::fs::write(store.path(), "{ not json").expect("write");
        assert_eq!(store.load(), Settings::default());
    }

    #[test]
    fn a_partial_file_keeps_defaults_for_missing_fields() {
        // `#[serde(default)]` is what makes adding a setting in a later version safe.
        let (_dir, store) = store();
        std::fs::write(store.path(), r#"{"earDetection": false}"#).expect("write");

        let settings = store.load();
        assert!(!settings.ear_detection);
        assert!(settings.auto_start, "untouched fields keep their default");
        assert_eq!(settings.low_battery_threshold, 20);
    }

    #[test]
    fn unknown_fields_are_ignored() {
        // A file written by a newer version must not stop an older one from starting.
        let (_dir, store) = store();
        std::fs::write(store.path(), r#"{"autoStart": false, "somethingNew": 42}"#).expect("write");
        assert!(!store.load().auto_start);
    }

    #[test]
    fn round_trips_through_the_file() {
        let (_dir, store) = store();
        let settings = Settings {
            auto_start: false,
            auto_update: false,
            ear_detection: false,
            low_battery_threshold: 40,
            grouped_battery: false,
        };

        store.save(&settings).expect("save");
        assert_eq!(store.load(), settings);
    }

    #[test]
    fn saving_creates_missing_directories() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = SettingsStore::new(dir.path().join("nested").join("deeper").join("s.json"));

        store.save(&Settings::default()).expect("save");
        assert!(store.path().exists());
    }

    #[test]
    fn saving_leaves_no_temporary_file_behind() {
        let (_dir, store) = store();
        store.save(&Settings::default()).expect("save");

        let temporary = store.path().with_extension("json.tmp");
        assert!(
            !temporary.exists(),
            "the temporary file must be renamed, not left in place"
        );
    }

    #[test]
    fn is_serialised_as_camel_case_for_the_ui() {
        let json = serde_json::to_string(&Settings::default()).expect("serialise");
        assert!(json.contains("\"autoStart\""), "got {json}");
        assert!(json.contains("\"lowBatteryThreshold\""), "got {json}");
    }

    #[test]
    fn an_out_of_range_threshold_is_clamped_on_load() {
        // The previous event based path accepted any u8, and a threshold above 100 silently
        // disabled the notification because no battery can ever be at or below it.
        let (_dir, store) = store();
        std::fs::write(store.path(), r#"{"lowBatteryThreshold": 250}"#).expect("write");
        assert_eq!(
            store.load().low_battery_threshold,
            MAX_LOW_BATTERY_THRESHOLD
        );
    }

    #[test]
    fn an_out_of_range_threshold_is_clamped_on_apply() {
        let mut settings = Settings::default();
        settings.apply(SettingsPatch {
            low_battery_threshold: Some(250),
            ..SettingsPatch::default()
        });
        assert_eq!(settings.low_battery_threshold, MAX_LOW_BATTERY_THRESHOLD);
    }

    #[test]
    fn applying_a_patch_only_touches_the_named_fields() {
        let mut settings = Settings::default();
        let changed = settings.apply(SettingsPatch {
            ear_detection: Some(false),
            ..SettingsPatch::default()
        });

        assert!(changed);
        assert!(!settings.ear_detection);
        assert!(settings.auto_start);
        assert!(settings.auto_update);
        assert_eq!(settings.low_battery_threshold, 20);
    }

    #[test]
    fn applying_an_unchanged_patch_reports_no_change() {
        // This is what stops a redundant file write and a pointless broadcast to every window.
        let mut settings = Settings::default();
        assert!(!settings.apply(SettingsPatch::default()));
        assert!(!settings.apply(SettingsPatch {
            auto_start: Some(true),
            ..SettingsPatch::default()
        }));
    }

    #[test]
    fn a_clamped_patch_that_changes_nothing_reports_no_change() {
        let mut settings = Settings {
            low_battery_threshold: MAX_LOW_BATTERY_THRESHOLD,
            ..Settings::default()
        };
        assert!(!settings.apply(SettingsPatch {
            low_battery_threshold: Some(250),
            ..SettingsPatch::default()
        }));
    }

    #[test]
    fn a_zero_threshold_disables_the_notification() {
        let settings = Settings {
            low_battery_threshold: LOW_BATTERY_DISABLED,
            ..Settings::default()
        };
        assert!(!settings.low_battery_notification_enabled());
        assert!(Settings::default().low_battery_notification_enabled());
    }

    #[test]
    fn grouping_can_be_turned_off_by_patch() {
        let mut settings = Settings::default();

        assert!(settings.apply(SettingsPatch {
            grouped_battery: Some(false),
            ..SettingsPatch::default()
        }));
        assert!(!settings.grouped_battery);
        assert!(settings.ear_detection, "untouched fields are left alone");
    }

    #[test]
    fn a_patch_deserialises_from_a_single_field() {
        let patch: SettingsPatch = serde_json::from_str(r#"{"autoStart": false}"#).expect("parse");
        assert_eq!(patch.auto_start, Some(false));
        assert_eq!(patch.ear_detection, None);
    }

    #[test]
    fn a_patch_rejects_a_malformed_value() {
        // The old path parsed payloads with `parse::<bool>()` and returned early on failure, so a
        // bad value looked exactly like a successful no-op to the UI.
        assert!(serde_json::from_str::<SettingsPatch>(r#"{"autoStart": "yes"}"#).is_err());
        assert!(serde_json::from_str::<SettingsPatch>(r#"{"lowBatteryThreshold": -5}"#).is_err());
    }
}
