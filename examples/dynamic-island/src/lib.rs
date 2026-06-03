//! 灵动岛 Tauri 扩展入口。
//!
//! 这个 crate 只负责当前项目的装配，不承担可复用框架职责。
//! 业务层负责应用设置、显示器选择、状态机和场景输入，视图层负责预览、桥接、
//! 原生窗口和渲染协调。

pub use echoisland_runtime;

// 应用级配置、显示器枚举和灵动岛模式生命周期。
mod app_settings;
mod config;
mod display_settings;
#[cfg(feature = "tauri-host")]
mod error;
mod host_platform;
#[cfg(feature = "tauri-host")]
mod mode_lifecycle;
#[cfg(feature = "tauri-host")]
mod monitor_manager;
mod native_window;
mod notification_sound;
#[cfg(feature = "tauri-host")]
mod state_machine;
mod updater_service;
#[cfg(feature = "tauri-host")]
mod window_operations;

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

#[cfg(feature = "tauri-host")]
pub use error::*;
#[cfg(feature = "tauri-host")]
pub use mode_lifecycle::{
    emergency_reset_dynamic_island, enter_dynamic_island_mode, exit_dynamic_island_mode,
    is_dynamic_island_mode, snap_dynamic_island_mode,
};
#[cfg(feature = "tauri-host")]
pub use monitor_manager::{MonitorInfo, MonitorManager};
pub use native_panel_core::{
    panel_display_key, PanelDisplayGeometry, PanelIslandWidthPreset, PanelLanguage, PanelRect,
};
pub use preview_host::{
    dynamic_island_ui_preview_snapshot, run_dynamic_island_ui_preview_standalone,
    DynamicIslandUiPreviewHost, StandaloneDynamicIslandUiPreviewHost,
};
#[cfg(feature = "tauri-host")]
pub use preview_host::{show_dynamic_island_ui_preview, TauriDynamicIslandUiPreviewHost};
#[cfg(feature = "tauri-host")]
pub use state_machine::{DynamicIslandState, DynamicIslandStateMachine, WindowSnapshot};
#[cfg(feature = "tauri-host")]
pub use window_operations::WindowOperationBatch;

/// 初始化灵动岛扩展
#[cfg(feature = "tauri-host")]
pub fn init<R: tauri::Runtime>(
    _app: &tauri::AppHandle<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("初始化灵动岛扩展");
    Ok(())
}
