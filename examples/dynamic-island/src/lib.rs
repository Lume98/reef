//! 灵动岛独立预览与原生窗口入口。
//!
//! 这个 crate 只负责当前项目的装配，不承担可复用框架职责。
//! 业务层负责应用设置、显示器选择和场景输入，视图层负责预览、桥接、
//! 原生窗口和渲染协调。

pub use echoisland_runtime;

// 应用级配置、显示器枚举和场景输入。
mod app_settings;
mod config;
mod display_settings;
mod native_window;
mod notification_sound;
mod updater_service;

pub mod business;

// 对外暴露平台无关的面板核心类型，避免调用方直接依赖 reef-ui crate 路径。
pub mod native_panel_core {
    pub use reef_native_panel_core::native_panel_core::*;
}

// 原生渲染协调层：把场景模型转换为渲染命令，并处理运行时交互。
mod native_panel_renderer;

// 运行时快照 → reef-widgets IslandWidget 桥接层。
pub mod island_widget_bridge;

// 对外暴露平台无关的场景类型，供扩展内外共享 UI 结构。
pub mod native_panel_scene {
    pub use reef_native_panel_core::native_panel_scene::*;
}

// 应用设置/显示器信息到场景构建输入的适配层。
mod native_panel_scene_input;

// 保存面板核心状态，并按最新运行时快照构建各 Surface 场景。
mod panel_scene_service;
mod preview_host;

pub mod view;

// Windows 平台的原生窗口、消息循环和 Direct2D 绘制实现。
#[cfg(target_os = "windows")]
mod windows_native_panel;

pub use native_panel_core::{
    panel_display_key, PanelDisplayGeometry, PanelIslandWidthPreset, PanelLanguage, PanelRect,
};
pub use preview_host::{
    dynamic_island_ui_preview_snapshot, run_dynamic_island_ui_preview_standalone,
    DynamicIslandUiPreviewHost, StandaloneDynamicIslandUiPreviewHost,
};
