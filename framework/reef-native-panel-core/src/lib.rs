//! Reef 原生面板共享核心。
//!
//! 这个 crate 只承载平台无关的运行时预览和宿主占位契约，不代理 UI/组件模型。

pub use echoisland_runtime;

pub mod native_window;
pub mod notification_sound;
pub mod preview_host;

pub use preview_host::{
    dynamic_island_ui_preview_snapshot, run_dynamic_island_ui_preview_standalone,
    DynamicIslandUiPreviewHost, StandaloneDynamicIslandUiPreviewHost,
};
