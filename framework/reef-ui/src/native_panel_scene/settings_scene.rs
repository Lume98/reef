use serde::Serialize;

use crate::{
    native_panel_core::{
        island_width_preset_label, panel_language_label, settings_row_action, PanelHitAction,
        PanelLanguage, PanelSettingsState,
    },
    native_panel_scene::{PanelInteractionProfile, SettingsSurfaceProjection},
    updater_service::{AppUpdatePhase, AppUpdateStatus},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsSurfaceScene {
    pub title: String,
    pub version_text: String,
    pub rows: Vec<SettingsSurfaceRowScene>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsSurfaceRowScene {
    pub id: String,
    pub label: String,
    pub control_kind: SettingsSurfaceControlKind,
    pub value_text: String,
    pub checked: Option<bool>,
    pub enabled: bool,
    pub action_key: String,
    pub update_phase: Option<String>,
    pub can_install: bool,
    pub can_open_release_page: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsSurfaceControlKind {
    Toggle,
    Action,
}

pub fn build_settings_surface_scene(
    projection: SettingsSurfaceProjection,
    settings: PanelSettingsState,
    app_version: &str,
    update_status: &AppUpdateStatus,
    interaction_profile: PanelInteractionProfile,
) -> SettingsSurfaceScene {
    let SettingsSurfaceProjection {
        selected_display_label,
        selected_display_supports_wide: _selected_display_supports_wide,
        effective_width_preset,
        has_display_options,
    } = projection;
    let texts = settings_texts(settings.language);
    let mut rows = vec![
        SettingsSurfaceRowScene {
            id: "island_display".to_string(),
            label: texts.island_display.to_string(),
            control_kind: SettingsSurfaceControlKind::Action,
            value_text: selected_display_label,
            checked: None,
            enabled: has_display_options,
            action_key: settings_action_key(0),
            update_phase: None,
            can_install: false,
            can_open_release_page: false,
        },
        SettingsSurfaceRowScene {
            id: "island_width".to_string(),
            label: texts.island_width.to_string(),
            control_kind: SettingsSurfaceControlKind::Action,
            value_text: island_width_preset_label(effective_width_preset).to_string(),
            checked: None,
            enabled: true,
            action_key: settings_action_key(1),
            update_phase: None,
            can_install: false,
            can_open_release_page: false,
        },
        SettingsSurfaceRowScene {
            id: "language".to_string(),
            label: texts.language.to_string(),
            control_kind: SettingsSurfaceControlKind::Action,
            value_text: panel_language_label(settings.language).to_string(),
            checked: None,
            enabled: true,
            action_key: settings_action_key(2),
            update_phase: None,
            can_install: false,
            can_open_release_page: false,
        },
        SettingsSurfaceRowScene {
            id: "completion_sound".to_string(),
            label: texts.mute_sound.to_string(),
            control_kind: SettingsSurfaceControlKind::Toggle,
            value_text: if !settings.completion_sound_enabled {
                texts.on.to_string()
            } else {
                texts.off.to_string()
            },
            checked: Some(!settings.completion_sound_enabled),
            enabled: true,
            action_key: settings_action_key(3),
            update_phase: None,
            can_install: false,
            can_open_release_page: false,
        },
        SettingsSurfaceRowScene {
            id: "mascot".to_string(),
            label: texts.mascot.to_string(),
            control_kind: SettingsSurfaceControlKind::Toggle,
            value_text: if settings.mascot_enabled {
                texts.on.to_string()
            } else {
                texts.off.to_string()
            },
            checked: Some(settings.mascot_enabled),
            enabled: true,
            action_key: settings_action_key(4),
            update_phase: None,
            can_install: false,
            can_open_release_page: false,
        },
    ];
    if interaction_profile != PanelInteractionProfile::Standalone {
        rows.push(SettingsSurfaceRowScene {
            id: "update".to_string(),
            label: update_status.label.clone(),
            control_kind: SettingsSurfaceControlKind::Action,
            value_text: settings_update_value_text(update_status, &texts),
            checked: None,
            enabled: !matches!(
                update_status.phase,
                AppUpdatePhase::Checking
                    | AppUpdatePhase::Downloading
                    | AppUpdatePhase::Installing
                    | AppUpdatePhase::Installed
            ),
            action_key: settings_action_key(5),
            update_phase: Some(update_phase_key(update_status.phase).to_string()),
            can_install: update_status.can_install,
            can_open_release_page: update_status.can_open_release_page,
        });
    }
    SettingsSurfaceScene {
        title: texts.title.to_string(),
        version_text: format!("Reef UI v{app_version}"),
        rows,
    }
}

pub fn settings_surface_row_action(index: usize) -> Option<PanelHitAction> {
    settings_row_action(index)
}

fn settings_action_key(index: usize) -> String {
    match settings_row_action(index) {
        Some(PanelHitAction::CycleDisplay) => "cycle_display",
        Some(PanelHitAction::CycleIslandWidth) => "cycle_island_width",
        Some(PanelHitAction::CycleLanguage) => "cycle_language",
        Some(PanelHitAction::ToggleCompletionSound) => "toggle_completion_sound",
        Some(PanelHitAction::ToggleMascot) => "toggle_mascot",
        Some(PanelHitAction::OpenSettingsLocation) => "open_settings_location",
        Some(PanelHitAction::OpenReleasePage) => "open_release_page",
        Some(PanelHitAction::FocusSession) => "focus_session",
        None => "unknown",
    }
    .to_string()
}

struct SettingsSurfaceTexts {
    title: &'static str,
    island_display: &'static str,
    island_width: &'static str,
    language: &'static str,
    mute_sound: &'static str,
    mascot: &'static str,
    on: &'static str,
    off: &'static str,
    release: &'static str,
}

fn settings_texts(language: PanelLanguage) -> SettingsSurfaceTexts {
    match language {
        PanelLanguage::Zh => SettingsSurfaceTexts {
            title: "设置",
            island_display: "灵动岛显示器",
            island_width: "灵动岛宽度",
            language: "面板语言",
            mute_sound: "静音",
            mascot: "助手",
            on: "开",
            off: "关",
            release: "发布",
        },
        PanelLanguage::Ja => SettingsSurfaceTexts {
            title: "設定",
            island_display: "表示先",
            island_width: "アイランド幅",
            language: "パネル言語",
            mute_sound: "ミュート",
            mascot: "マスコット",
            on: "オン",
            off: "オフ",
            release: "リリース",
        },
        PanelLanguage::En => SettingsSurfaceTexts {
            title: "Settings",
            island_display: "Island Display",
            island_width: "Island Width",
            language: "Panel Language",
            mute_sound: "Mute Sound",
            mascot: "Mascot",
            on: "On",
            off: "Off",
            release: "Release",
        },
    }
}

fn settings_update_value_text(
    update_status: &AppUpdateStatus,
    texts: &SettingsSurfaceTexts,
) -> String {
    if update_status.phase == AppUpdatePhase::Idle && update_status.value_text == "Release" {
        texts.release.to_string()
    } else {
        update_status.value_text.clone()
    }
}

fn update_phase_key(phase: AppUpdatePhase) -> &'static str {
    match phase {
        AppUpdatePhase::Idle => "idle",
        AppUpdatePhase::Checking => "checking",
        AppUpdatePhase::UpToDate => "up_to_date",
        AppUpdatePhase::Available => "available",
        AppUpdatePhase::Downloading => "downloading",
        AppUpdatePhase::Installing => "installing",
        AppUpdatePhase::Installed => "installed",
        AppUpdatePhase::Failed => "failed",
        AppUpdatePhase::UnsupportedPortable => "unsupported_portable",
    }
}
