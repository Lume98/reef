//! EchoIsland 平台无关 UI 模型。
//!
//! 这里维护灵动岛的状态归一化、场景构建和更新提示状态。平台窗口只消费这些模型，
//! 不在这里直接处理 Win32、Direct2D 或 Tauri 窗口生命周期。

// 面板核心：快照同步、布局常量、交互状态、动画/刷新队列等纯逻辑。
pub mod native_panel_core;

// 面板场景：把核心状态和运行时快照转换成可渲染的 Surface/Card/Settings 场景树。
pub mod native_panel_scene;

// 原生宿主消费的纯 UI 描述、表现模型、视觉计划和渲染命令。
pub mod native_panel_ui;

// 更新状态：为设置面板和状态卡片提供应用更新阶段的共享模型。
pub mod updater_service;

pub use updater_service::{AppUpdatePhase, AppUpdateStatus};
