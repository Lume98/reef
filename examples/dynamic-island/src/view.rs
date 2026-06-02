//! 灵动岛视图层 façade。
//!
//! 这里收拢预览、窗口和 widget 桥接入口，保持视图相关 API 集中暴露。

pub use crate::island_widget_bridge::{build_island_widget, build_island_widget_spec};
pub use crate::native_window::{hide, show_without_app, snap};
#[cfg(feature = "tauri-host")]
pub use crate::preview_host::{
    dynamic_island_ui_preview_snapshot, run_dynamic_island_ui_preview_standalone,
    show_dynamic_island_ui_preview, DynamicIslandUiPreviewHost,
    StandaloneDynamicIslandUiPreviewHost,
};

#[cfg(feature = "tauri-host")]
pub use crate::native_window::show;

#[cfg(feature = "tauri-host")]
pub use crate::preview_host::TauriDynamicIslandUiPreviewHost;
