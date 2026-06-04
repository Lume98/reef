//! 灵动岛视图层 façade。
//!
//! 这里收拢预览、窗口和 widget 桥接入口，保持视图相关 API 集中暴露。

pub use reef_view::{create_root, WidgetRoot};
pub use reef_widgets::{
    BodyLine, Card, CardStyle, CompactBar, IslandWidget, MascotPose, MascotWidget,
};
pub use reef_widgets::island_widget::DisplayMode;

pub use crate::island_widget_bridge::{build_island_widget, island_render_overrides};
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
