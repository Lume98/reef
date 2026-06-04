use std::sync::atomic::AtomicBool;

use echoisland_runtime::RuntimeSnapshot;

use crate::{
    native_panel_renderer::facade::command::{
        execute_native_panel_cycle_island_width_command,
        execute_native_panel_cycle_language_command,
        execute_native_panel_debug_mode_trigger_command,
        execute_native_panel_toggle_completion_sound_command,
        execute_native_panel_toggle_mascot_command, NativePanelPlatformEvent,
    },
    notification_sound::play_message_card_sound,
};

use super::dpi::ensure_windows_process_dpi_awareness;
use super::runtime_entry::{
    drain_windows_native_panel_platform_events, spawn_platform_loops_internal,
    with_windows_native_panel_runtime,
};
use super::runtime_input::windows_runtime_input_descriptor_without_app;

static WINDOWS_NATIVE_PANEL_STANDALONE_EVENT_DISPATCH_LOOP_STARTED: AtomicBool =
    AtomicBool::new(false);

pub(crate) fn native_ui_enabled() -> bool {
    windows_native_ui_enabled_by_default()
}

pub(crate) fn create_native_panel() -> Result<(), String> {
    with_windows_native_panel_runtime(|runtime| runtime.create_panel())
}

pub(crate) fn spawn_platform_loops_without_app() {
    ensure_windows_process_dpi_awareness();
    spawn_platform_loops_internal();
    spawn_windows_native_panel_standalone_event_dispatch_loop();
}

pub(crate) fn update_native_panel_snapshot_without_app(
    snapshot: &RuntimeSnapshot,
) -> Result<(), String> {
    let input = windows_runtime_input_descriptor_without_app();
    let sync = with_windows_native_panel_runtime(|runtime| {
        runtime.sync_snapshot_bundle(snapshot, &input)
    })?;
    if sync.is_some_and(|sync| sync.reminder.play_sound) {
        play_message_card_sound();
    }
    Ok(())
}

pub(crate) fn refresh_native_panel_from_last_snapshot_without_app() -> Result<(), String> {
    let input = windows_runtime_input_descriptor_without_app();
    with_windows_native_panel_runtime(|runtime| {
        runtime
            .rerender_from_last_snapshot_with_input(&input)
            .map(|_| ())
    })
}

pub(crate) fn reposition_native_panel_to_selected_display_without_app() -> Result<(), String> {
    let input = windows_runtime_input_descriptor_without_app();
    with_windows_native_panel_runtime(|runtime| {
        runtime.reposition_to_selected_display_with_input(&input)
    })
}

fn spawn_windows_native_panel_standalone_event_dispatch_loop() {
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    if WINDOWS_NATIVE_PANEL_STANDALONE_EVENT_DISPATCH_LOOP_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    std::thread::spawn(|| loop {
        std::thread::sleep(Duration::from_millis(33));
        if let Err(error) = dispatch_queued_native_panel_platform_events_without_app() {
            tracing::warn!(error = %error, "failed to dispatch standalone Windows native panel event");
        }
    });
}

fn dispatch_queued_native_panel_platform_events_without_app() -> Result<(), String> {
    let events = drain_windows_native_panel_platform_events()?;
    if events.is_empty() {
        return Ok(());
    }

    for event in events {
        dispatch_native_panel_platform_event_without_app(event)?;
    }
    Ok(())
}

fn dispatch_native_panel_platform_event_without_app(
    event: NativePanelPlatformEvent,
) -> Result<(), String> {
    match event {
        NativePanelPlatformEvent::ToggleSettingsSurface => {
            let input = windows_runtime_input_descriptor_without_app();
            with_windows_native_panel_runtime(|runtime| {
                runtime
                    .toggle_settings_surface_with_input(&input)
                    .map(|_| ())
            })
        }
        NativePanelPlatformEvent::QuitApplication => {
            std::process::exit(0);
        }
        NativePanelPlatformEvent::CycleIslandWidth => {
            execute_native_panel_cycle_island_width_command(
                true,
                refresh_native_panel_from_last_snapshot_without_app,
            )
        }
        NativePanelPlatformEvent::ToggleCompletionSound => {
            execute_native_panel_toggle_completion_sound_command(
                refresh_native_panel_from_last_snapshot_without_app,
            )
        }
        NativePanelPlatformEvent::ToggleMascot => execute_native_panel_toggle_mascot_command(
            refresh_native_panel_from_last_snapshot_without_app,
        ),
        NativePanelPlatformEvent::CycleLanguage => execute_native_panel_cycle_language_command(
            refresh_native_panel_from_last_snapshot_without_app,
        ),
        NativePanelPlatformEvent::DebugModeTrigger => {
            execute_native_panel_debug_mode_trigger_command(
                refresh_native_panel_from_last_snapshot_without_app,
            )
        }
        NativePanelPlatformEvent::CycleDisplay => {
            reposition_native_panel_to_selected_display_without_app()?;
            refresh_native_panel_from_last_snapshot_without_app()
        }
        NativePanelPlatformEvent::FocusSession(_) => Ok(()),
        NativePanelPlatformEvent::OpenSettingsLocation
        | NativePanelPlatformEvent::OpenReleasePage => Ok(()),
    }
}

#[cfg(windows)]
fn windows_native_ui_enabled_by_default() -> bool {
    true
}

#[cfg(not(windows))]
fn windows_native_ui_enabled_by_default() -> bool {
    false
}
