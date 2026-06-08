use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::state::{PanelIslandWidthPreset, PanelLanguage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    #[serde(default = "default_completion_sound_enabled")]
    pub completion_sound_enabled: bool,
    #[serde(default = "default_mascot_enabled")]
    pub mascot_enabled: bool,
    #[serde(default)]
    pub debug_mode_enabled: bool,
    #[serde(default)]
    pub island_width_preset: PanelIslandWidthPreset,
    #[serde(default)]
    pub language: PanelLanguage,
    #[serde(default)]
    pub preferred_display_index: usize,
    #[serde(default)]
    pub preferred_display_key: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            completion_sound_enabled: default_completion_sound_enabled(),
            mascot_enabled: default_mascot_enabled(),
            debug_mode_enabled: false,
            island_width_preset: PanelIslandWidthPreset::Standard,
            language: PanelLanguage::En,
            preferred_display_index: 0,
            preferred_display_key: None,
        }
    }
}

static APP_SETTINGS_CACHE: OnceLock<Mutex<AppSettings>> = OnceLock::new();

pub fn app_settings_path() -> PathBuf {
    crate::config::get_app_config_dir().join("native-panel-settings.json")
}

pub fn current_app_settings() -> AppSettings {
    APP_SETTINGS_CACHE
        .get_or_init(|| Mutex::new(load_app_settings_from_disk().unwrap_or_default()))
        .lock()
        .map(|guard| guard.clone())
        .unwrap_or_default()
}

pub fn update_completion_sound_enabled(enabled: bool) -> Result<AppSettings> {
    update_app_settings(|settings| settings.completion_sound_enabled = enabled)
}

pub fn update_mascot_enabled(enabled: bool) -> Result<AppSettings> {
    update_app_settings(|settings| settings.mascot_enabled = enabled)
}

pub fn update_debug_mode_enabled(enabled: bool) -> Result<AppSettings> {
    update_app_settings(|settings| settings.debug_mode_enabled = enabled)
}

pub fn update_island_width_preset(preset: PanelIslandWidthPreset) -> Result<AppSettings> {
    update_app_settings(|settings| settings.island_width_preset = preset)
}

pub fn update_language(language: PanelLanguage) -> Result<AppSettings> {
    update_app_settings(|settings| settings.language = language)
}

pub fn update_preferred_display_selection(
    index: usize,
    key: Option<String>,
) -> Result<AppSettings> {
    update_app_settings(|settings| {
        settings.preferred_display_index = index;
        settings.preferred_display_key = key;
    })
}

fn update_app_settings(update: impl FnOnce(&mut AppSettings)) -> Result<AppSettings> {
    let cache = APP_SETTINGS_CACHE
        .get_or_init(|| Mutex::new(load_app_settings_from_disk().unwrap_or_default()));
    let mut guard = cache
        .lock()
        .map_err(|_| anyhow::anyhow!("native panel settings lock poisoned"))?;
    let previous = guard.clone();
    update(&mut guard);
    if *guard != previous {
        save_app_settings(&app_settings_path(), &guard)?;
    }
    Ok(guard.clone())
}

fn load_app_settings_from_disk() -> Result<AppSettings> {
    load_app_settings(&app_settings_path())
}

fn load_app_settings(path: &Path) -> Result<AppSettings> {
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let raw = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_slice(&raw).context("failed to decode native panel settings")
}

fn save_app_settings(path: &Path, settings: &AppSettings) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let encoded =
        serde_json::to_vec_pretty(settings).context("failed to encode native panel settings")?;
    fs::write(path, encoded).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn default_completion_sound_enabled() -> bool {
    false
}

fn default_mascot_enabled() -> bool {
    true
}
