//! 灵动岛业务层 façade。
//!
//! 这里收拢当前项目的业务规则和状态入口，避免视图层直接散落地依赖各个内部模块。

use crate::{
    native_panel_core::{
        resolve_preferred_panel_display_index, PanelSettingsState,
    },
    native_panel_scene::{
        panel_display_option_state_with_width_support, PanelDisplayOptionState,
    },
};

pub use crate::app_settings::{
    app_settings_path, current_app_settings, update_completion_sound_enabled,
    update_debug_mode_enabled, update_island_width_preset, update_language,
    update_mascot_enabled, update_preferred_display_selection, AppSettings,
};
pub use crate::config::get_app_config_dir;
pub use crate::display_settings::{
    display_key_for_panel_geometry, display_option_from_panel_geometry,
    display_option_from_panel_geometry_with_width_support, panel_rect_from_panel_geometry,
    DisplayOption,
};
pub use crate::updater_service::{current_update_status, AppUpdatePhase, AppUpdateStatus};

#[cfg(feature = "tauri-host")]
pub use crate::display_settings::{
    display_option_from_monitor, display_options_from_monitors, list_available_displays,
    panel_geometry_for_monitor, panel_rect_from_monitor,
};

#[cfg(feature = "tauri-host")]
pub use crate::mode_lifecycle::{
    emergency_reset_dynamic_island, enter_dynamic_island_mode, exit_dynamic_island_mode,
    is_dynamic_island_mode, snap_dynamic_island_mode,
};

#[cfg(feature = "tauri-host")]
pub use crate::monitor_manager::{MonitorInfo, MonitorManager};

#[cfg(feature = "tauri-host")]
pub use crate::state_machine::{DynamicIslandState, DynamicIslandStateMachine, WindowSnapshot};

#[cfg(feature = "tauri-host")]
pub use crate::window_operations::WindowOperationBatch;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NativePanelDisplaySelectionUpdate {
    pub(crate) selected_display_index: usize,
    pub(crate) selected_display_key: String,
}

pub(crate) fn panel_settings_state_from_app_settings(
    selected_display_index: usize,
    settings: &AppSettings,
) -> PanelSettingsState {
    PanelSettingsState {
        selected_display_index,
        island_width_preset: settings.island_width_preset,
        completion_sound_enabled: settings.completion_sound_enabled,
        mascot_enabled: settings.mascot_enabled,
        debug_mode_enabled: settings.debug_mode_enabled,
        language: settings.language,
    }
}

pub(crate) fn panel_display_options_from_display_options(
    displays: &[DisplayOption],
) -> Vec<PanelDisplayOptionState> {
    displays
        .iter()
        .map(|display| {
            panel_display_option_state_with_width_support(
                display.index,
                display.key.clone(),
                &display.name,
                display.width,
                display.height,
                display.supports_wide_island,
            )
        })
        .collect()
}

pub(crate) fn resolve_panel_selected_display_index(
    display_keys: &[String],
    settings: &AppSettings,
    fallback_index: Option<usize>,
) -> usize {
    resolve_preferred_panel_display_index(
        display_keys,
        settings.preferred_display_key.as_deref(),
        settings.preferred_display_index,
        fallback_index,
    )
    .unwrap_or(0)
}

pub(crate) fn resolve_selected_display_index_from_display_options(
    displays: &[DisplayOption],
    settings: &AppSettings,
    fallback_index: Option<usize>,
) -> usize {
    if displays.is_empty() {
        return fallback_index.unwrap_or(settings.preferred_display_index);
    }
    resolve_panel_selected_display_index(
        &displays
            .iter()
            .map(|display| display.key.clone())
            .collect::<Vec<_>>(),
        settings,
        fallback_index,
    )
}

pub(crate) fn resolve_next_display_selection_update_from_display_options(
    displays: &[DisplayOption],
    settings: &AppSettings,
) -> Option<NativePanelDisplaySelectionUpdate> {
    if displays.is_empty() {
        return None;
    }
    let current = resolve_selected_display_index_from_display_options(displays, settings, Some(0));
    let selected = displays.get((current + 1) % displays.len())?;
    Some(NativePanelDisplaySelectionUpdate {
        selected_display_index: selected.index,
        selected_display_key: selected.key.clone(),
    })
}
