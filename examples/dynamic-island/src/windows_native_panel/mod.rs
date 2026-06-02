//! Windows 原生面板实现。
//!
//! 该模块封装 Win32 窗口、平台消息循环、命中区域、Direct2D/DirectWrite 绘制和运行时宿主。
//! 上层只通过 facade 暴露的生命周期函数进入，避免平台细节泄漏到 Tauri 扩展入口。

// Direct2D/DirectWrite 绘制基础设施。
mod d2d_painter;
mod d2d_resource_cache;
mod direct2d;
mod directwrite;
mod dpi;
mod draw_presenter;

// 对 dynamic_island crate 内部暴露的 Windows 面板门面。
pub(crate) mod facade;

// 窗口、命中测试、消息分发和运行时宿主。
mod hit_region;
mod host_runtime;
mod host_runtime_close;
mod host_runtime_hover;
mod host_window;
mod layered_window;
mod message_dispatch;
mod paint_backend;
mod paint_bridge;
mod platform_loop;
mod renderer;
#[cfg(feature = "tauri-host")]
mod runtime_backend;
mod runtime_entry;
mod runtime_input;
mod runtime_traits;
mod window_shell;

pub(crate) use host_runtime::WindowsNativePanelRuntime;
pub(crate) use renderer::WindowsNativePanelRenderer;
#[cfg(feature = "tauri-host")]
pub(crate) use runtime_backend::{
    current_windows_native_panel_runtime_backend, WindowsNativePanelRuntimeBackendFacade,
};

pub(crate) use facade::{
    create_native_panel, native_ui_enabled, spawn_platform_loops_without_app,
    update_native_panel_snapshot_without_app,
};
#[cfg(feature = "tauri-host")]
pub(crate) use facade::{
    hide_native_panel, refresh_native_panel_from_last_snapshot,
    reposition_native_panel_to_selected_display, set_shared_expanded_body_height,
    spawn_platform_loops, update_native_panel_snapshot,
};

#[cfg(test)]
pub(crate) use host_runtime::WindowsNativePanelHost;
#[cfg(test)]
pub(crate) use host_window::WindowsNativePanelDrawFrame;
#[cfg(test)]
pub(crate) use platform_loop::{
    clear_windows_native_panel_window_messages, queue_windows_native_panel_window_message,
    wait_windows_native_platform_loop_processed_at_least, windows_native_platform_loop_generations,
};
#[cfg(test)]
pub(crate) use window_shell::WINDOWS_WM_PAINT;

pub(crate) fn hide_native_panel_without_app() -> Result<(), String> {
    runtime_entry::with_windows_native_panel_runtime(|runtime| runtime.hide_panel())
}

const WINDOWS_FALLBACK_PANEL_SCREEN_FRAME: crate::native_panel_core::PanelRect =
    crate::native_panel_core::PanelRect {
        x: 0.0,
        y: 0.0,
        width: 1440.0,
        height: 900.0,
    };

#[cfg(test)]
mod tests;
