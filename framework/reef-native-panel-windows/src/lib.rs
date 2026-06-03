//! Reef 原生面板 Windows 适配层。
//!
//! 当前保留为独立适配 facade，供后续迁移真正的 Win32 / Direct2D 实现。

pub use reef_native_panel_core as core;
pub use reef_native_panel_core::native_panel_core;
pub use reef_native_panel_core::native_panel_scene;
pub use reef_native_panel_core::native_panel_ui;

use echoisland_runtime::RuntimeSnapshot;

pub mod dpi;
pub mod direct2d;
pub mod directwrite;
pub mod layered_window;
pub mod resource_cache;
pub mod hit_region;
pub mod screen_geometry;

pub use dpi::{
    ensure_windows_process_dpi_awareness, resolve_windows_dpi_scale_for_window,
    resolve_windows_system_dpi_scale, WindowsDpiScale, WindowsPhysicalRect,
};
pub use direct2d::WindowsDirect2DFactory;
pub use directwrite::{
    WindowsDirectWriteFactory, WindowsDirectWriteFontFallback, WindowsDirectWriteTextLayoutRequest,
};
pub use hit_region::{resolve_windows_native_panel_hit_test, WindowsNativePanelHitTest};
pub use screen_geometry::{
    fallback_standalone_display_geometry, windows_standalone_screen_frame_with_scale,
};
pub use layered_window::{
    apply_windows_layered_window_initial_attributes,
    windows_layered_window_composition_mode_for_painter, WindowsLayeredAlphaBitmap,
    WindowsLayeredBitmapSize, WindowsLayeredWindowCompositionMode,
};
pub use resource_cache::{
    WindowsDirect2DResourceCacheState, WindowsDirect2DResourceKey,
};

pub fn create_native_panel() -> Result<(), String> {
    Ok(())
}

pub fn spawn_platform_loops_without_app() {}

pub fn update_native_panel_snapshot_without_app(
    snapshot: &RuntimeSnapshot,
) -> Result<(), String> {
    let _ = snapshot;
    Ok(())
}

pub fn hide_native_panel_without_app() -> Result<(), String> {
    Ok(())
}

#[cfg(feature = "tauri-host")]
pub fn spawn_platform_loops<R: tauri::Runtime + 'static>(_app: tauri::AppHandle<R>) {}

#[cfg(feature = "tauri-host")]
pub fn update_native_panel_snapshot<R: tauri::Runtime>(
    _app: &tauri::AppHandle<R>,
    snapshot: &RuntimeSnapshot,
) -> Result<(), String> {
    update_native_panel_snapshot_without_app(snapshot)
}

#[cfg(feature = "tauri-host")]
pub fn hide_native_panel<R: tauri::Runtime>(_: &tauri::AppHandle<R>) -> Result<(), String> {
    hide_native_panel_without_app()
}

#[cfg(feature = "tauri-host")]
pub fn refresh_native_panel_from_last_snapshot<R: tauri::Runtime>(
    _: &tauri::AppHandle<R>,
) -> Result<(), String> {
    Ok(())
}

#[cfg(feature = "tauri-host")]
pub fn reposition_native_panel_to_selected_display<R: tauri::Runtime>(
    _: &tauri::AppHandle<R>,
) -> Result<(), String> {
    Ok(())
}

#[cfg(feature = "tauri-host")]
pub fn set_shared_expanded_body_height<R: tauri::Runtime>(
    _: &tauri::AppHandle<R>,
    _: f64,
) -> Result<(), String> {
    Ok(())
}
