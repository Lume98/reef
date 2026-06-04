//! Reef 原生面板 Windows 适配层。
//!
//! 当前对外提供 source-based standalone facade；底层 Win32 host 仍在持续迁移。

extern crate self as reef_native_panel_windows;

pub use reef_native_panel_core as core;
pub use reef_native_panel_core::native_panel_core;
pub use reef_native_panel_core::native_panel_scene;
pub use reef_native_panel_core::native_panel_ui;

use echoisland_runtime::RuntimeSnapshot;
use reef_core::geometry::Size;
use reef_native_panel_core::{DynamicIslandSource, DynamicIslandViewState};
use reef_render::primitive::VisualPlan;
use reef_view::create_root;

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
pub use hit_region::{resolve_windows_native_panel_hit_test, WindowsNativePanelHitTest};
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

mod app_settings;
mod business;
mod config;
mod display_settings;
mod native_panel_renderer;
mod native_panel_scene_input;
mod native_window;
mod notification_sound;
pub mod page;
mod panel_scene_service;
mod updater_service;
mod windows_native_panel;

pub fn render_dynamic_island_initial_plan<S>(source: &S) -> VisualPlan
where
    S: DynamicIslandSource,
    S::Action: 'static,
{
    let state = DynamicIslandViewState::default();
    let island = source.build(state);
    let widget = island.to_widget();
    let mut root = create_root(Size {
        width: widget.width.max(1.0),
        height: widget.expanded_height.max(widget.compact_height).max(1.0),
    });
    root.set_root(island);
    root.render_current()
}

pub fn run_dynamic_island_standalone<S>(source: S) -> Result<(), String>
where
    S: DynamicIslandSource,
    S::Action: 'static,
{
    let _ = render_dynamic_island_initial_plan(&source);
    #[cfg(target_os = "windows")]
    {
        // The migrated Windows host currently carries a placeholder mascot sprite asset.
        // Force the stable vector mascot path for the standalone example to avoid a white block
        // where the assistant should be rendered.
        std::env::set_var("ECHOISLAND_MASCOT_SPRITE", "0");
        // The current standalone host still renders from the snapshot-backed runtime path.
        // We use the framework preview snapshot to keep the window alive until the source-based
        // host fully replaces the legacy runtime.
        let snapshot = reef_native_panel_core::preview_host::dynamic_island_ui_preview_snapshot();
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
        return windows_native_panel::create_native_panel();
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}

pub fn spawn_platform_loops_without_app() {
    #[cfg(target_os = "windows")]
    {
        windows_native_panel::spawn_platform_loops_without_app();
    }
}

pub fn update_native_panel_snapshot_without_app(snapshot: &RuntimeSnapshot) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        return windows_native_panel::update_native_panel_snapshot_without_app(snapshot);
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
        return windows_native_panel::hide_native_panel_without_app();
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
