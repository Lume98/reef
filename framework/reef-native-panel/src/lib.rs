pub mod platform;
pub mod presentation;
pub mod preview;
pub mod runtime;
pub mod scene;
pub mod state;
pub mod updater_service;

pub use updater_service::{AppUpdatePhase, AppUpdateStatus};

pub(crate) use platform::windows::{
    app_settings, business, config, display_settings, dpi, native_window, notification_sound, page,
    panel_scene_input, platform_windows_host,
};
pub(crate) use platform::windows::{
    wait_windows_native_platform_loop_processed_at_least, windows_native_platform_loop_generations,
};

pub use platform::windows::{
    create_native_panel, hide_native_panel_without_app, run_dynamic_island_preview_standalone,
    spawn_platform_loops_without_app, update_native_panel_snapshot_without_app,
};

#[cfg(feature = "tauri-host")]
pub use platform::windows::{
    hide_native_panel, refresh_native_panel_from_last_snapshot,
    reposition_native_panel_to_selected_display, set_shared_expanded_body_height,
    spawn_platform_loops, update_native_panel_snapshot,
};
