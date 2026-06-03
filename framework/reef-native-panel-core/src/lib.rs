//! Reef 原生面板共享核心。
//!
//! 这个 crate 只承载跨平台可复用的模型、契约和预览辅助，不绑定 Windows 窗口实现。

pub use echoisland_runtime;

pub mod native_panel_core {
    pub use reef_ui::native_panel_core::*;
}

pub mod native_panel_scene {
    pub use reef_ui::native_panel_scene::*;
}

pub mod native_panel_ui {
    pub use reef_ui::native_panel_ui::*;
}

pub mod host_platform;
pub mod native_window;
pub mod notification_sound;
pub mod preview_host;
pub mod updater_service;

pub use host_platform::NativePanelHostPlatform;
pub use native_panel_core::{
    panel_display_key, PanelDisplayGeometry, PanelIslandWidthPreset, PanelLanguage, PanelRect,
};
pub use preview_host::{
    dynamic_island_ui_preview_snapshot, run_dynamic_island_ui_preview_standalone,
    DynamicIslandUiPreviewHost, StandaloneDynamicIslandUiPreviewHost,
};
pub use updater_service::{current_update_status, AppUpdatePhase, AppUpdateStatus};

