//! 灵动岛业务层 façade。
//!
//! 这里收拢当前项目的业务规则和状态入口，避免视图层直接散落地依赖各个内部模块。

mod display_selection;
mod input;

pub use crate::app_settings::{
    app_settings_path, current_app_settings, update_completion_sound_enabled,
    update_debug_mode_enabled, update_island_width_preset, update_language, update_mascot_enabled,
    update_preferred_display_selection, AppSettings,
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

pub(crate) use display_selection::{
    resolve_next_display_selection_update_from_display_options,
    resolve_panel_selected_display_index, resolve_selected_display_index_from_display_options,
    NativePanelDisplaySelectionUpdate,
};
pub(crate) use input::{
    native_panel_runtime_input_context_from_display_options,
    native_panel_runtime_input_context_from_display_options_with_screen_frame,
    native_panel_runtime_input_descriptor_from_app_settings,
    native_panel_runtime_input_descriptor_from_context,
    native_panel_runtime_input_descriptor_from_display_options_with_screen_frame,
    panel_display_options_from_display_options, panel_scene_build_input_from_app_settings,
    panel_scene_build_input_from_display_options,
};

#[cfg(test)]
mod tests;
