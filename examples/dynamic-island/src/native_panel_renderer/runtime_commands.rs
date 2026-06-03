use echoisland_runtime::RuntimeSnapshot;
#[cfg(feature = "tauri-host")]
use std::sync::atomic::AtomicBool;
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};
#[cfg(feature = "tauri-host")]
use tokio::sync::Notify;
use tracing::warn;

use crate::{
    app_settings::{
        current_app_settings, update_completion_sound_enabled, update_debug_mode_enabled,
        update_island_width_preset, update_language, update_mascot_enabled,
    },
    native_panel_core::{
        next_island_width_preset_for_display, next_panel_language, PanelInteractionCommand,
    },
};
#[cfg(feature = "tauri-host")]
use crate::{
    app_settings::{update_preferred_display_selection, AppSettings},
    business::resolve_next_display_selection_update_from_display_options,
    display_settings::list_available_displays,
    host_platform::NativePanelHostPlatform,
};

#[cfg(feature = "tauri-host")]
use super::descriptors::{
    dispatch_native_panel_platform_events, NativePanelPlatformEvent, NativePanelPointerInput,
    NativePanelPointerInputOutcome, NativePanelQueuedRuntimeCommandHandler,
    NativePanelRuntimeCommandCapability,
};
#[cfg(feature = "tauri-host")]
use super::host_runtime_facade::NativePanelRuntimeDispatchMode;
#[cfg(feature = "tauri-host")]
use super::runtime_platform_backend::{
    current_native_panel_runtime_backend, NativePanelPlatformRuntimeBackend,
};
#[cfg(feature = "tauri-host")]
use super::runtime_click::dispatch_native_panel_click_command_with_handler;
use super::runtime_interaction::NativePanelSettingsSurfaceSnapshotUpdate;
use super::transition_controller::NativePanelTransitionRequest;

const DEBUG_MODE_TRIGGER_CLICK_THRESHOLD: u8 = 10;
static DEBUG_MODE_TRIGGER_CLICK_COUNT: AtomicU8 = AtomicU8::new(0);

#[cfg(feature = "tauri-host")]
pub(crate) fn execute_native_panel_focus_session_command(
    host: &impl NativePanelHostPlatform,
    _session_id: String,
) {
    let _ = host.focus_main_window();
}

#[cfg(feature = "tauri-host")]
pub(crate) fn execute_native_panel_quit_application_command(host: &impl NativePanelHostPlatform) {
    host.quit_application();
}

#[cfg(feature = "tauri-host")]
pub(crate) fn execute_native_panel_cycle_display_command<H>(
    host: &H,
    reposition: impl FnOnce(&H) -> Result<(), String>,
) -> Result<AppSettings, String>
where
    H: NativePanelHostPlatform,
{
    let displays = list_available_displays(host)?;
    let settings = current_app_settings();
    let Some(next_selection) =
        resolve_next_display_selection_update_from_display_options(&displays, &settings)
    else {
        return Ok(settings);
    };
    let settings = update_preferred_display_selection(
        next_selection.selected_display_index,
        Some(next_selection.selected_display_key),
    )
    .map_err(|error| error.to_string())?;
    reposition(host)?;
    Ok(settings)
}

pub(crate) fn execute_native_panel_toggle_completion_sound_command(
    refresh: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let next_enabled = !current_app_settings().completion_sound_enabled;
    update_completion_sound_enabled(next_enabled).map_err(|error| error.to_string())?;
    refresh()
}

pub(crate) fn execute_native_panel_cycle_island_width_command(
    supports_wide: bool,
    refresh: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let next_preset = next_island_width_preset_for_display(
        current_app_settings().island_width_preset,
        supports_wide,
    );
    update_island_width_preset(next_preset).map_err(|error| error.to_string())?;
    refresh()
}

pub(crate) fn execute_native_panel_cycle_language_command(
    refresh: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let next_language = next_panel_language(current_app_settings().language);
    update_language(next_language).map_err(|error| error.to_string())?;
    refresh()
}

pub(crate) fn execute_native_panel_toggle_mascot_command(
    refresh: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let next_enabled = !current_app_settings().mascot_enabled;
    update_mascot_enabled(next_enabled).map_err(|error| error.to_string())?;
    refresh()
}

pub(crate) fn execute_native_panel_debug_mode_trigger_command(
    refresh: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    if current_app_settings().debug_mode_enabled {
        let next_count = DEBUG_MODE_TRIGGER_CLICK_COUNT
            .fetch_add(1, Ordering::SeqCst)
            .saturating_add(1);
        if next_count < DEBUG_MODE_TRIGGER_CLICK_THRESHOLD {
            return Ok(());
        }

        DEBUG_MODE_TRIGGER_CLICK_COUNT.store(0, Ordering::SeqCst);
        update_debug_mode_enabled(false).map_err(|error| error.to_string())?;
        return refresh();
    }

    let next_count = DEBUG_MODE_TRIGGER_CLICK_COUNT
        .fetch_add(1, Ordering::SeqCst)
        .saturating_add(1);

    if next_count < DEBUG_MODE_TRIGGER_CLICK_THRESHOLD {
        return Ok(());
    }

    DEBUG_MODE_TRIGGER_CLICK_COUNT.store(0, Ordering::SeqCst);
    update_debug_mode_enabled(true).map_err(|error| error.to_string())?;
    refresh()
}

#[cfg(feature = "tauri-host")]
pub(crate) fn execute_native_panel_open_settings_location_command<H>(host: &H) -> Result<(), String>
where
    H: NativePanelHostPlatform,
{
    host.open_settings_location()
}

#[cfg(feature = "tauri-host")]
pub(crate) fn execute_native_panel_open_release_page_command<H>(host: &H) -> Result<(), String>
where
    H: NativePanelHostPlatform,
{
    crate::updater_service::spawn_native_update_flow((*host).clone());
    Ok(())
}

pub(crate) fn execute_native_panel_settings_surface_command(
    sync_update: impl FnOnce() -> Result<Option<NativePanelSettingsSurfaceSnapshotUpdate>, String>,
    dispatch_update: impl FnOnce(
        Option<NativePanelTransitionRequest>,
        Option<RuntimeSnapshot>,
    ) -> Result<(), String>,
) -> Result<bool, String> {
    let Some(update) = sync_update()? else {
        return Ok(false);
    };
    dispatch_update(update.transition_request, update.snapshot)?;
    Ok(true)
}

#[cfg(feature = "tauri-host")]
pub(crate) fn dispatch_native_panel_click_command_with_host<H>(
    host: H,
    command: PanelInteractionCommand,
    toggle_settings_surface: fn(&H) -> Result<(), String>,
    dispatch_mode: NativePanelRuntimeDispatchMode,
) -> Result<Option<NativePanelPlatformEvent>, String>
where
    H: NativePanelHostPlatform,
{
    let mut executor =
        native_panel_host_runtime_command_executor(host, toggle_settings_surface, dispatch_mode);
    dispatch_native_panel_click_command_with_handler(&mut executor, command)
}

#[cfg(feature = "tauri-host")]
pub(crate) fn dispatch_native_panel_platform_events_with_host<H>(
    host: H,
    events: impl IntoIterator<Item = NativePanelPlatformEvent>,
    toggle_settings_surface: fn(&H) -> Result<(), String>,
    dispatch_mode: NativePanelRuntimeDispatchMode,
) -> Result<(), String>
where
    H: NativePanelHostPlatform,
{
    let mut executor =
        native_panel_host_runtime_command_executor(host, toggle_settings_surface, dispatch_mode);
    dispatch_native_panel_platform_events(&mut executor, events)
}

#[cfg(feature = "tauri-host")]
pub(crate) fn run_native_panel_runtime_with_queued_command_dispatch<H, T>(
    host: &H,
    run: impl FnOnce(&mut NativePanelQueuedRuntimeCommandHandler) -> Result<T, String>,
    toggle_settings_surface: fn(&H) -> Result<(), String>,
    dispatch_mode: NativePanelRuntimeDispatchMode,
) -> Result<T, String>
where
    H: NativePanelHostPlatform,
{
    let mut handler = NativePanelQueuedRuntimeCommandHandler::default();
    let value = run(&mut handler)?;
    dispatch_native_panel_platform_events_with_host(
        (*host).clone(),
        handler.take_events(),
        toggle_settings_surface,
        dispatch_mode,
    )?;
    Ok(value)
}

#[cfg(feature = "tauri-host")]
pub(crate) fn run_native_panel_pointer_input_with_queued_command_dispatch<H>(
    host: &H,
    input_event: NativePanelPointerInput,
    now: std::time::Instant,
    run: impl FnOnce(
        NativePanelPointerInput,
        std::time::Instant,
        &mut NativePanelQueuedRuntimeCommandHandler,
    ) -> Result<NativePanelPointerInputOutcome, String>,
    toggle_settings_surface: fn(&H) -> Result<(), String>,
    dispatch_mode: NativePanelRuntimeDispatchMode,
) -> Result<NativePanelPointerInputOutcome, String>
where
    H: NativePanelHostPlatform,
{
    run_native_panel_runtime_with_queued_command_dispatch(
        host,
        |handler| run(input_event, now, handler),
        toggle_settings_surface,
        dispatch_mode,
    )
}

#[cfg(feature = "tauri-host")]
pub(crate) fn dispatch_drained_native_panel_platform_events_with_host<H>(
    host: H,
    drain_events: fn() -> Result<Vec<NativePanelPlatformEvent>, String>,
    toggle_settings_surface: fn(&H) -> Result<(), String>,
    dispatch_mode: NativePanelRuntimeDispatchMode,
) -> Result<(), String>
where
    H: NativePanelHostPlatform,
{
    let events = drain_events()?;
    if events.is_empty() {
        return Ok(());
    }
    dispatch_native_panel_platform_events_with_host(
        host,
        events,
        toggle_settings_surface,
        dispatch_mode,
    )
}

#[cfg(feature = "tauri-host")]
pub(crate) fn spawn_native_panel_platform_event_dispatch_loop<R>(
    loop_started: &'static AtomicBool,
    host: R,
    notifier: Arc<Notify>,
    dispatch: fn(R) -> Result<(), String>,
    error_message: &'static str,
) where
    R: NativePanelHostPlatform,
{
    if loop_started.swap(true, Ordering::SeqCst) {
        return;
    }

    let loop_notifier = notifier.clone();
    tauri::async_runtime::spawn(async move {
        loop {
            loop_notifier.notified().await;
            if let Err(error) = dispatch(host.clone()) {
                warn!(error = %error, "{error_message}");
            }
        }
    });
    notifier.notify_one();
}

#[cfg(feature = "tauri-host")]
pub(crate) fn spawn_native_panel_platform_loops_with_event_dispatch<H>(
    host: H,
    spawn_platform_loops: impl FnOnce(),
    spawn_event_dispatch_loop: impl FnOnce(H),
) where
    H: NativePanelHostPlatform,
{
    spawn_platform_loops();
    spawn_event_dispatch_loop(host);
}

#[cfg(feature = "tauri-host")]
pub(crate) fn dispatch_native_panel_app_command<H, F>(host: &H, command: F) -> Result<(), String>
where
    H: NativePanelHostPlatform,
    F: FnOnce(H) -> Result<(), String> + Send + 'static,
{
    command((*host).clone())
}

#[cfg(feature = "tauri-host")]
pub(crate) fn spawn_native_panel_app_command<H, F>(host: H, command: F, error_message: &'static str)
where
    H: NativePanelHostPlatform,
    F: FnOnce(H) -> Result<(), String> + Send + 'static,
{
    tauri::async_runtime::spawn(async move {
        if let Err(error) = command(host) {
            warn!(error = %error, "{error_message}");
        }
    });
}

#[cfg(feature = "tauri-host")]
pub(crate) fn dispatch_native_panel_command(
    command: impl FnOnce() -> Result<(), String> + Send + 'static,
) -> Result<(), String> {
    command()
}

#[cfg(feature = "tauri-host")]
pub(crate) fn spawn_native_panel_command(
    command: impl FnOnce() -> Result<(), String> + Send + 'static,
    error_message: &'static str,
) {
    tauri::async_runtime::spawn(async move {
        if let Err(error) = command() {
            warn!(error = %error, "{error_message}");
        }
    });
}

#[cfg(feature = "tauri-host")]
pub(crate) struct NativePanelAppHandleRuntimeCommandExecutor<H: NativePanelHostPlatform> {
    pub(crate) host: H,
    pub(crate) toggle_settings_surface: fn(&H) -> Result<(), String>,
    pub(crate) dispatch_mode: NativePanelRuntimeDispatchMode,
}

#[cfg(feature = "tauri-host")]
pub(crate) fn native_panel_host_runtime_command_executor<H: NativePanelHostPlatform>(
    host: H,
    toggle_settings_surface: fn(&H) -> Result<(), String>,
    dispatch_mode: NativePanelRuntimeDispatchMode,
) -> NativePanelAppHandleRuntimeCommandExecutor<H> {
    NativePanelAppHandleRuntimeCommandExecutor {
        host,
        toggle_settings_surface,
        dispatch_mode,
    }
}

#[cfg(feature = "tauri-host")]
pub(crate) trait NativePanelAppHandleRuntimeCommandBackend {
    type Host: NativePanelHostPlatform;

    fn host(&self) -> &Self::Host;

    fn dispatch_app_command(
        &mut self,
        command: impl FnOnce(Self::Host) -> Result<(), String> + Send + 'static,
        _error_message: &'static str,
    ) -> Result<(), String> {
        dispatch_native_panel_app_command(self.host(), command)
    }

    fn dispatch_command(
        &mut self,
        command: impl FnOnce() -> Result<(), String> + Send + 'static,
        _error_message: &'static str,
    ) -> Result<(), String> {
        dispatch_native_panel_command(command)
    }

    fn refresh_from_last_snapshot_with_host(host: &Self::Host) -> Result<(), String> {
        NativePanelPlatformRuntimeBackend::refresh_from_last_snapshot(
            &current_native_panel_runtime_backend(),
            host,
        )
    }

    fn reposition_to_selected_display_with_host(host: &Self::Host) -> Result<(), String> {
        NativePanelPlatformRuntimeBackend::reposition_to_selected_display(
            &current_native_panel_runtime_backend(),
            host,
        )
    }

    fn focus_session_command(&mut self, session_id: String) -> Result<(), String> {
        self.dispatch_app_command(
            move |host| {
                execute_native_panel_focus_session_command(&host, session_id);
                Ok(())
            },
            "failed to focus session from native panel",
        )
    }

    fn toggle_settings_surface_command(&mut self) -> Result<(), String>;

    fn quit_application_command(&mut self) -> Result<(), String> {
        self.dispatch_app_command(
            move |host| {
                execute_native_panel_quit_application_command(&host);
                Ok(())
            },
            "failed to quit application from native panel",
        )
    }

    fn cycle_display_command(&mut self) -> Result<(), String> {
        self.dispatch_app_command(
            move |host| {
                execute_native_panel_cycle_display_command(&host, |host| {
                    Self::reposition_to_selected_display_with_host(host)
                })
                .and_then(|_| Self::refresh_from_last_snapshot_with_host(&host))
            },
            "failed to update preferred display",
        )
    }

    fn cycle_island_width_command(&mut self) -> Result<(), String> {
        self.dispatch_app_command(
            move |host| {
                let supports_wide = true;
                execute_native_panel_cycle_island_width_command(supports_wide, || {
                    Self::refresh_from_last_snapshot_with_host(&host)
                })
            },
            "failed to update island width setting",
        )
    }

    fn toggle_completion_sound_command(&mut self) -> Result<(), String> {
        self.dispatch_app_command(
            move |host| {
                execute_native_panel_toggle_completion_sound_command(|| {
                    Self::refresh_from_last_snapshot_with_host(&host)
                })
            },
            "failed to update completion sound setting",
        )
    }

    fn toggle_mascot_command(&mut self) -> Result<(), String> {
        self.dispatch_app_command(
            move |host| {
                execute_native_panel_toggle_mascot_command(|| {
                    Self::refresh_from_last_snapshot_with_host(&host)
                })
            },
            "failed to update mascot setting",
        )
    }

    fn cycle_language_command(&mut self) -> Result<(), String> {
        self.dispatch_app_command(
            move |host| {
                execute_native_panel_cycle_language_command(|| {
                    Self::refresh_from_last_snapshot_with_host(&host)
                })
            },
            "failed to update native panel language setting",
        )
    }

    fn debug_mode_trigger_command(&mut self) -> Result<(), String> {
        self.dispatch_app_command(
            move |host| {
                execute_native_panel_debug_mode_trigger_command(|| {
                    Self::refresh_from_last_snapshot_with_host(&host)
                })
            },
            "failed to enable debug mode from native panel",
        )
    }

    fn open_settings_location_command(&mut self) -> Result<(), String> {
        self.dispatch_app_command(
            |host| execute_native_panel_open_settings_location_command(&host),
            "failed to open settings folder",
        )
    }

    fn open_release_page_command(&mut self) -> Result<(), String> {
        self.dispatch_app_command(
            |host| execute_native_panel_open_release_page_command(&host),
            "failed to start update flow",
        )
    }
}

#[cfg(feature = "tauri-host")]
impl<H: NativePanelHostPlatform> NativePanelRuntimeCommandCapability
    for NativePanelAppHandleRuntimeCommandExecutor<H>
where
    NativePanelAppHandleRuntimeCommandExecutor<H>: NativePanelAppHandleRuntimeCommandBackend,
{
    type Error = String;

    fn focus_session(&mut self, session_id: String) -> Result<(), Self::Error> {
        self.focus_session_command(session_id)
    }

    fn toggle_settings_surface(&mut self) -> Result<(), Self::Error> {
        self.toggle_settings_surface_command()
    }

    fn quit_application(&mut self) -> Result<(), Self::Error> {
        self.quit_application_command()
    }

    fn cycle_display(&mut self) -> Result<(), Self::Error> {
        self.cycle_display_command()
    }

    fn cycle_island_width(&mut self) -> Result<(), Self::Error> {
        self.cycle_island_width_command()
    }

    fn toggle_completion_sound(&mut self) -> Result<(), Self::Error> {
        self.toggle_completion_sound_command()
    }

    fn toggle_mascot(&mut self) -> Result<(), Self::Error> {
        self.toggle_mascot_command()
    }

    fn cycle_language(&mut self) -> Result<(), Self::Error> {
        self.cycle_language_command()
    }

    fn debug_mode_trigger(&mut self) -> Result<(), Self::Error> {
        self.debug_mode_trigger_command()
    }

    fn open_settings_location(&mut self) -> Result<(), Self::Error> {
        self.open_settings_location_command()
    }

    fn open_release_page(&mut self) -> Result<(), Self::Error> {
        self.open_release_page_command()
    }
}

#[cfg(feature = "tauri-host")]
impl<H: NativePanelHostPlatform> NativePanelAppHandleRuntimeCommandBackend
    for NativePanelAppHandleRuntimeCommandExecutor<H>
{
    type Host = H;

    fn host(&self) -> &Self::Host {
        &self.host
    }

    fn dispatch_app_command(
        &mut self,
        command: impl FnOnce(Self::Host) -> Result<(), String> + Send + 'static,
        error_message: &'static str,
    ) -> Result<(), String> {
        match self.dispatch_mode {
            NativePanelRuntimeDispatchMode::Immediate => {
                dispatch_native_panel_app_command(self.host(), command)
            }
            NativePanelRuntimeDispatchMode::Scheduled => {
                spawn_native_panel_app_command(self.host.clone(), command, error_message);
                Ok(())
            }
        }
    }

    fn dispatch_command(
        &mut self,
        command: impl FnOnce() -> Result<(), String> + Send + 'static,
        error_message: &'static str,
    ) -> Result<(), String> {
        match self.dispatch_mode {
            NativePanelRuntimeDispatchMode::Immediate => dispatch_native_panel_command(command),
            NativePanelRuntimeDispatchMode::Scheduled => {
                spawn_native_panel_command(command, error_message);
                Ok(())
            }
        }
    }

    fn toggle_settings_surface_command(&mut self) -> Result<(), String> {
        (self.toggle_settings_surface)(&self.host)
    }
}

#[cfg(test)]
mod tests {
    use super::execute_native_panel_settings_surface_command;
    use crate::native_panel_renderer::{
        runtime_interaction::NativePanelSettingsSurfaceSnapshotUpdate,
        transition_controller::NativePanelTransitionRequest,
    };
    use chrono::Utc;
    use echoisland_runtime::{RuntimeSnapshot, SessionSnapshotView};

    fn runtime_snapshot(status: &str, session_status: &str) -> RuntimeSnapshot {
        RuntimeSnapshot {
            status: status.to_string(),
            primary_source: "codex".to_string(),
            active_session_count: 1,
            total_session_count: 1,
            pending_permission_count: 0,
            pending_question_count: 0,
            pending_permission: None,
            pending_question: None,
            pending_permissions: vec![],
            pending_questions: vec![],
            sessions: vec![SessionSnapshotView {
                session_id: "session-1".to_string(),
                source: "codex".to_string(),
                project_name: None,
                cwd: None,
                model: None,
                terminal_app: None,
                terminal_bundle: None,
                host_app: None,
                window_title: None,
                tty: None,
                terminal_pid: None,
                cli_pid: None,
                iterm_session_id: None,
                kitty_window_id: None,
                tmux_env: None,
                tmux_pane: None,
                tmux_client_tty: None,
                status: session_status.to_string(),
                current_tool: None,
                tool_description: None,
                last_user_prompt: None,
                last_assistant_message: Some("done".to_string()),
                tool_history_count: 0,
                tool_history: vec![],
                last_activity: Utc::now(),
            }],
        }
    }

    #[test]
    fn settings_surface_command_dispatches_synced_update() {
        let mut dispatched = None;

        let changed = execute_native_panel_settings_surface_command(
            || {
                Ok(Some(NativePanelSettingsSurfaceSnapshotUpdate {
                    transition_request: Some(NativePanelTransitionRequest::SurfaceSwitch),
                    snapshot: Some(runtime_snapshot("idle", "Running")),
                }))
            },
            |request, snapshot| {
                dispatched = Some((request, snapshot.map(|snapshot| snapshot.status)));
                Ok(())
            },
        )
        .expect("dispatch settings surface update");

        assert!(changed);
        assert_eq!(
            dispatched,
            Some((
                Some(NativePanelTransitionRequest::SurfaceSwitch),
                Some("idle".to_string()),
            ))
        );
    }

    #[test]
    fn settings_surface_command_skips_dispatch_without_update() {
        let mut dispatched = false;

        let changed = execute_native_panel_settings_surface_command(
            || Ok(None),
            |_, _| {
                dispatched = true;
                Ok(())
            },
        )
        .expect("skip settings surface dispatch");

        assert!(!changed);
        assert!(!dispatched);
    }

    #[test]
    fn settings_surface_command_propagates_sync_error() {
        let error = execute_native_panel_settings_surface_command(
            || Err("sync failed".to_string()),
            |_, _| Ok(()),
        )
        .expect_err("surface sync errors should propagate");

        assert_eq!(error, "sync failed");
    }

    #[test]
    fn settings_surface_command_propagates_dispatch_error() {
        let error = execute_native_panel_settings_surface_command(
            || {
                Ok(Some(NativePanelSettingsSurfaceSnapshotUpdate {
                    transition_request: Some(NativePanelTransitionRequest::SurfaceSwitch),
                    snapshot: Some(runtime_snapshot("idle", "Running")),
                }))
            },
            |_, _| Err("dispatch failed".to_string()),
        )
        .expect_err("surface dispatch errors should propagate");

        assert_eq!(error, "dispatch failed");
    }
}
