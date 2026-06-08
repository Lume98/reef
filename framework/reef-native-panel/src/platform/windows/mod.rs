//! Reef 原生面板 Windows 适配层。
//!
//! 当前对外提供 snapshot preview standalone facade；Win32 host 的绘制入口只接受
//! `NativePanelPaintPlan` 协议，旧 widget `DrawPlan` 仍保留在 core/source facade 边界之外。

use echoisland_runtime::RuntimeSnapshot;

pub mod direct2d;
pub mod directwrite;
pub mod dpi;
pub mod hit_region;
pub mod layered_window;
pub mod platform_loop_control;
pub mod resource_cache;
pub mod screen_geometry;
pub mod window_geometry;

pub use direct2d::WindowsDirect2DFactory;
pub use directwrite::{
    WindowsDirectWriteFactory, WindowsDirectWriteFontFallback, WindowsDirectWriteTextLayoutRequest,
};
pub use dpi::{
    ensure_windows_process_dpi_awareness, resolve_windows_dpi_scale_for_window,
    resolve_windows_system_dpi_scale, WindowsDpiScale, WindowsPhysicalRect,
};
pub use hit_region::{resolve_windows_panel_hit_test, WindowsPanelHitTest};
pub use layered_window::{
    apply_windows_layered_window_initial_attributes,
    windows_layered_window_composition_mode_for_painter, WindowsLayeredAlphaBitmap,
    WindowsLayeredBitmapSize, WindowsLayeredWindowCompositionMode,
};
pub use platform_loop_control::{
    ensure_windows_native_platform_loop_thread, platform_loop_thread_started,
    schedule_windows_native_platform_loop_wake,
    wait_windows_native_platform_loop_processed_at_least, wake_windows_native_platform_loop,
    windows_native_platform_loop_generations,
};
pub use resource_cache::{WindowsDirect2DResourceCacheState, WindowsDirect2DResourceKey};
pub use screen_geometry::{
    fallback_standalone_display_geometry, windows_standalone_screen_frame_with_scale,
};
pub use window_geometry::{resolve_windows_panel_window_frame, windows_client_pointer_regions};

pub(crate) mod app_settings;
pub(crate) mod business;
pub(crate) mod config;
pub(crate) mod display_settings;
pub(crate) mod native_window;
pub(crate) mod notification_sound;
pub mod page;
pub(crate) mod panel_scene_input;
pub(crate) mod platform_windows_host;
mod updater_service;

pub fn run_dynamic_island_preview_standalone() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // The migrated Windows host currently carries a placeholder mascot sprite asset.
        // Force the stable vector mascot path for the standalone example to avoid a white block
        // where the assistant should be rendered.
        std::env::set_var("ECHOISLAND_MASCOT_SPRITE", "0");
        let snapshot = crate::preview::dynamic_island_ui_preview_snapshot();
        native_window::show_without_app(&snapshot)?;
        loop {
            std::thread::park();
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}

pub fn create_native_panel() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        return platform_windows_host::create_native_panel();
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}

pub fn spawn_platform_loops_without_app() {
    #[cfg(target_os = "windows")]
    {
        platform_windows_host::spawn_platform_loops_without_app();
    }
}

pub fn update_native_panel_snapshot_without_app(snapshot: &RuntimeSnapshot) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        return platform_windows_host::update_native_panel_snapshot_without_app(snapshot);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = snapshot;
        Ok(())
    }
}

pub fn hide_native_panel_without_app() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        return platform_windows_host::hide_native_panel_without_app();
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
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
