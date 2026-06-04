//! Reef 原生面板共享核心。
//!
//! 这个 crate 只承载跨平台可复用的模型、契约和预览辅助，不绑定 Windows 窗口实现。

pub use echoisland_runtime;

pub mod native_panel_core {
    pub use reef_ui::panel::core::*;
}

pub mod native_panel_scene {
    pub use reef_ui::panel::scene::*;
}

pub mod native_panel_ui {
    pub use reef_ui::panel::ui::*;
}

pub mod panel {
    pub use crate::native_panel_core as core;
    pub use crate::native_panel_scene as scene;
    pub use crate::native_panel_ui as ui;
}

pub mod dynamic_island_interaction;
pub mod dynamic_island_page;
pub mod dynamic_island_source;
pub mod host_platform;
pub mod native_window;
pub mod notification_sound;
pub mod preview_host;
pub mod runtime_input;
pub mod updater_service;

pub use dynamic_island_interaction::{
    is_dynamic_island_horizontal_swipe, resolve_dynamic_island_gesture,
    resolve_dynamic_island_root_gesture_at_point, DynamicIslandInteractionContext,
    DynamicIslandInteractionEffect, DynamicIslandSwipeSpec,
};
pub use dynamic_island_page::{
    build_dynamic_island_page_model, dynamic_island_page, dynamic_island_target_for_hit_target,
    resolve_dynamic_island_effect, resolve_dynamic_island_source_gesture_effect,
    resolve_dynamic_island_source_target_effect, DynamicIslandPageModel,
    DynamicIslandRuntimeAction, DynamicIslandRuntimeEffect, DynamicIslandViewState,
    RuntimeSnapshotDynamicIslandSource,
};
pub use dynamic_island_source::DynamicIslandSource;
pub use host_platform::NativePanelHostPlatform;
pub use native_panel_core::{
    panel_display_key, PanelDisplayGeometry, PanelIslandWidthPreset, PanelLanguage, PanelRect,
};
pub use preview_host::{
    dynamic_island_ui_preview_snapshot, run_dynamic_island_ui_preview_standalone,
    DynamicIslandUiPreviewHost, StandaloneDynamicIslandUiPreviewHost,
};
pub use runtime_input::{
    native_panel_runtime_input_descriptor_from_context,
    native_panel_runtime_input_descriptor_from_parts, panel_scene_build_input_from_parts,
};
pub use updater_service::{current_update_status, AppUpdatePhase, AppUpdateStatus};
