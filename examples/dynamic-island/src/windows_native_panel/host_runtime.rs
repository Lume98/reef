use std::time::Instant;

#[cfg(feature = "tauri-host")]
use tauri::AppHandle;

#[cfg(feature = "tauri-host")]
use super::runtime_input::windows_runtime_input_descriptor;
use super::{
    draw_presenter::WindowsNativePanelDrawPresenter,
    host_window::WindowsNativePanelHostWindow,
    message_dispatch::pump_window_messages as pump_dispatched_window_messages,
    platform_loop::WindowsNativePanelPlatformLoopState,
    window_shell::WindowsNativePanelWindowShell,
    WindowsNativePanelRenderer,
};
use crate::{
    native_panel_core::{
        HoverTransition, PanelAnimationDescriptor, PanelPoint, PanelState,
    },
    native_panel_renderer::facade::{
        command::{
            NativePanelPlatformEvent, NativePanelPointerInput, NativePanelPointerInputOutcome,
            NativePanelRuntimeCommandHandler,
        },
        descriptor::NativePanelRuntimeInputDescriptor,
        interaction::{
            dispatch_native_panel_click_command_at_point_with_handler,
            handle_native_panel_pointer_input_with_handler,
            handle_optional_native_panel_pointer_input_with_handler,
            native_panel_click_state_slots, sync_native_panel_hover_and_refresh_for_runtime,
            sync_native_panel_hover_interaction_and_rerender_at_point_with_input_descriptor,
            sync_native_panel_hover_interaction_and_rerender_for_inside_with_input_descriptor,
            sync_native_panel_hover_interaction_and_rerender_for_pointer_input_with_input_descriptor,
            sync_native_panel_hover_interaction_for_state, NativePanelHoverSyncResult,
            NativePanelSettingsSurfaceToggleResult,
        },
        presentation::NativePanelPresentationModel,
        renderer::NativePanelRuntimeSceneCache,
        runtime::{
            apply_native_panel_hover_sync_result_for_runtime,
            apply_native_panel_runtime_scene_sync_result_for_runtime,
            apply_native_panel_settings_surface_toggle_result_for_runtime,
            rerender_runtime_scene_sync_result_to_host_for_runtime_with_input_descriptor,
            toggle_native_panel_settings_surface_and_rerender_for_runtime_with_input_descriptor,
            NativePanelRuntimeSceneSyncResult,
        },
        host::{
            create_native_panel_via_host_controller, hide_native_panel_via_host_controller,
            reposition_native_panel_host_from_input_descriptor_via_controller,
            set_native_panel_host_shared_body_height_via_controller, NativePanelHost,
        },
        shell::pump_native_panel_host_shell_runtime,
        transition::NativePanelTransitionRequest,
    },
};

#[derive(Default)]
pub(crate) struct WindowsNativePanelHost {
    pub(super) renderer: WindowsNativePanelRenderer,
    pub(super) window: WindowsNativePanelHostWindow,
    pub(super) presenter: WindowsNativePanelDrawPresenter,
    pub(super) shell: WindowsNativePanelWindowShell,
    pub(super) pending_events: Vec<NativePanelPlatformEvent>,
}

#[derive(Default)]
pub(crate) struct WindowsNativePanelRuntime {
    pub(super) panel_state: PanelState,
    pub(super) primary_pointer_down: bool,
    pub(super) ignores_mouse_events: bool,
    pub(super) user_hidden: bool,
    pub(super) host: WindowsNativePanelHost,
    pub(super) platform_loop: WindowsNativePanelPlatformLoopState,
    pub(super) scene_cache: NativePanelRuntimeSceneCache,
    pub(super) animation_scheduler:
        reef_ui::native_panel_ui::render::NativePanelAnimationFrameScheduler,
    pub(super) next_animation_wake_at: Option<Instant>,
    pub(super) last_animation_descriptor: Option<PanelAnimationDescriptor>,
    pub(super) last_transition_request: Option<NativePanelTransitionRequest>,
    pub(super) pending_close_presentation: Option<NativePanelPresentationModel>,
    /// True for the duration of a close animation that was triggered by the user
    /// hovering out of the panel (not by status-queue auto-collapse). Used to keep
    /// the edge action buttons (settings / quit) visible during the close so they
    /// fade out via the normal width morph instead of popping off, and to preserve
    /// cards from any surface — not just Status — across mid-close re-renders.
    pub(super) hover_close_in_progress: bool,
    pub(super) active_count_marquee_started_at: Option<Instant>,
    pub(super) mascot_animation_started_at: Option<Instant>,
    pub(super) last_focus_click: Option<(String, Instant)>,
}

impl WindowsNativePanelRuntime {
    pub(super) fn pump_platform_loop(&mut self) -> Result<(), String> {
        if self.host.shell.has_pending_destroy_command() {
            super::platform_loop::clear_windows_native_panel_hit_regions(
                self.platform_loop.last_raw_window_handle,
            );
        }
        let had_unstarted_hover_open = self.has_unstarted_hover_open_request();
        let input = self.platform_loop_runtime_input_descriptor();
        let status_queue_refresh =
            self.refresh_status_queue_from_last_raw_snapshot_with_input(&input)?;
        let result = pump_native_panel_host_shell_runtime(self);
        self.cancel_unstarted_hover_open_if_pointer_left(had_unstarted_hover_open);
        let now = Instant::now();
        let poll_result = self.sync_current_pointer_polling_interaction(now, &input)?;
        let animation_frame = self.advance_animation_frame_at(now)?;
        let marquee_frame = self.refresh_active_count_marquee_frame_at(now);
        let mascot_frame = self.refresh_mascot_animation_frame_at(now);
        if status_queue_refresh
            || poll_result.is_some()
            || animation_frame.is_some()
            || marquee_frame
            || mascot_frame
        {
            pump_native_panel_host_shell_runtime(self)?;
        }
        self.schedule_next_status_queue_refresh_wake();
        self.schedule_next_animation_frame_wake(now);
        result
    }

    fn has_unstarted_hover_open_request(&self) -> bool {
        self.last_transition_request == Some(NativePanelTransitionRequest::Open)
            && !self.panel_state.transitioning
    }

    fn cancel_unstarted_hover_open_if_pointer_left(&mut self, had_unstarted_hover_open: bool) {
        if !had_unstarted_hover_open
            || self.panel_state.transitioning
            || self.host.shell.last_pointer_input() != Some(NativePanelPointerInput::Leave)
        {
            return;
        }

        self.last_transition_request = None;
        self.panel_state.expanded = false;
        self.panel_state.status_auto_expanded = false;
        self.panel_state.surface_mode = crate::native_panel_core::ExpandedSurface::Default;
        self.panel_state.pointer_inside_since = None;
    }

    fn platform_loop_runtime_input_descriptor(&self) -> NativePanelRuntimeInputDescriptor {
        super::runtime_input::windows_platform_loop_runtime_input_descriptor(
            self.host.window.descriptor.preferred_display_index,
            self.host.window.descriptor.screen_frame,
        )
    }

    pub(super) fn pump_window_messages(&mut self) -> Result<(), String> {
        pump_dispatched_window_messages(self)
    }

    pub(super) fn apply_runtime_scene_sync_result(
        &mut self,
        sync_result: NativePanelRuntimeSceneSyncResult,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<(), String> {
        apply_native_panel_runtime_scene_sync_result_for_runtime(self, sync_result, input)
            .map(|_| ())
    }

    pub(super) fn apply_settings_surface_toggle_result(
        &mut self,
        result: NativePanelSettingsSurfaceToggleResult,
    ) -> bool {
        apply_native_panel_settings_surface_toggle_result_for_runtime(self, result)
    }

    pub(super) fn apply_hover_sync_result(
        &mut self,
        hover_sync: NativePanelHoverSyncResult,
    ) -> Option<HoverTransition> {
        apply_native_panel_hover_sync_result_for_runtime(self, hover_sync)
    }

    pub(super) fn sync_hover_and_refresh_with_input(
        &mut self,
        resolve: impl FnOnce(
            &mut WindowsNativePanelHost,
            &mut NativePanelRuntimeSceneCache,
            &mut PanelState,
        ) -> Result<Option<NativePanelHoverSyncResult>, String>,
    ) -> Result<Option<HoverTransition>, String> {
        sync_native_panel_hover_and_refresh_for_runtime(self, resolve)
    }

    pub(super) fn create_panel(&mut self) -> Result<(), String> {
        self.user_hidden = false;
        create_native_panel_via_host_controller(&mut self.host)
    }

    pub(super) fn hide_panel(&mut self) -> Result<(), String> {
        self.user_hidden = true;
        hide_native_panel_via_host_controller(&mut self.host)
    }

    pub(super) fn reposition_to_selected_display_with_input(
        &mut self,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<(), String> {
        reposition_native_panel_host_from_input_descriptor_via_controller(&mut self.host, input)
    }

    pub(super) fn set_shared_expanded_body_height(
        &mut self,
        body_height: f64,
    ) -> Result<(), String> {
        set_native_panel_host_shared_body_height_via_controller(&mut self.host, body_height)
    }

    pub(super) fn sync_hover_inside(
        &mut self,
        inside: bool,
        now: Instant,
    ) -> Option<HoverTransition> {
        sync_native_panel_hover_interaction_for_state(
            &mut self.panel_state,
            inside,
            now,
            crate::native_panel_core::HOVER_DELAY_MS,
        )
        .transition
    }

    pub(super) fn refresh_active_count_marquee_frame_at(&mut self, now: Instant) -> bool {
        self.refresh_active_count_marquee_frame_at_impl(now)
    }

    pub(super) fn refresh_mascot_animation_frame_at(&mut self, now: Instant) -> bool {
        self.refresh_mascot_animation_frame_at_impl(now)
    }

    fn lightweight_refresh_plan(
        &self,
    ) -> crate::native_panel_core::NativePanelLightweightRefreshPlan {
        self.lightweight_refresh_plan_impl()
    }

    pub(super) fn rerender_from_last_snapshot_with_input(
        &mut self,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<bool, String> {
        rerender_runtime_scene_sync_result_to_host_for_runtime_with_input_descriptor(self, input)
    }

    #[cfg(feature = "tauri-host")]
    pub(super) fn rerender_from_last_snapshot<R: tauri::Runtime>(
        &mut self,
        app: &AppHandle<R>,
    ) -> Result<bool, String> {
        let input = windows_runtime_input_descriptor(app);
        self.rerender_from_last_snapshot_with_input(&input)
    }

    pub(super) fn toggle_settings_surface_with_input(
        &mut self,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<bool, String> {
        toggle_native_panel_settings_surface_and_rerender_for_runtime_with_input_descriptor(
            self, input,
        )
    }

    pub(super) fn sync_hover_and_refresh_at_point_with_input(
        &mut self,
        point: PanelPoint,
        now: Instant,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<Option<HoverTransition>, String> {
        self.sync_hover_and_refresh_with_input(|host, cache, state| {
            sync_native_panel_hover_interaction_and_rerender_at_point_with_input_descriptor(
                host,
                cache,
                state,
                point,
                now,
                crate::native_panel_core::HOVER_DELAY_MS,
                input,
            )
            .map(Some)
        })
    }

    pub(super) fn sync_hover_and_refresh_inside_with_input(
        &mut self,
        inside: bool,
        now: Instant,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<Option<HoverTransition>, String> {
        self.sync_hover_and_refresh_with_input(|host, cache, state| {
            sync_native_panel_hover_interaction_and_rerender_for_inside_with_input_descriptor(
                host,
                cache,
                state,
                inside,
                now,
                crate::native_panel_core::HOVER_DELAY_MS,
                input,
            )
            .map(Some)
        })
    }

    pub(super) fn sync_hover_and_refresh_for_pointer_input_with_input(
        &mut self,
        input_event: NativePanelPointerInput,
        now: Instant,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<Option<HoverTransition>, String> {
        self.sync_hover_and_refresh_with_input(|host, cache, state| {
            sync_native_panel_hover_interaction_and_rerender_for_pointer_input_with_input_descriptor(
                host,
                cache,
                state,
                input_event,
                now,
                crate::native_panel_core::HOVER_DELAY_MS,
                input,
            )
        })
    }

    pub(super) fn dispatch_click_command_at_point_with_handler<H>(
        &mut self,
        point: PanelPoint,
        now: Instant,
        handler: &mut H,
    ) -> Result<Option<NativePanelPlatformEvent>, H::Error>
    where
        H: NativePanelRuntimeCommandHandler,
    {
        let mut click_state =
            native_panel_click_state_slots(&self.panel_state, &mut self.last_focus_click);
        dispatch_native_panel_click_command_at_point_with_handler(
            &mut click_state,
            &self.host,
            point,
            now,
            crate::native_panel_core::CARD_FOCUS_CLICK_DEBOUNCE_MS,
            handler,
        )
    }

    pub(super) fn take_queued_platform_events(&mut self) -> Vec<NativePanelPlatformEvent> {
        self.host.take_platform_events()
    }

    pub(super) fn handle_window_message_with_handler(
        &mut self,
        message_id: u32,
        lparam: isize,
        now: Instant,
        input: &NativePanelRuntimeInputDescriptor,
        handler: &mut impl NativePanelRuntimeCommandHandler<Error = String>,
    ) -> Result<Option<NativePanelPointerInputOutcome>, String> {
        let message = self.host.shell.decode_window_message(message_id, lparam);
        handle_optional_native_panel_pointer_input_with_handler(self, message, now, input, handler)
    }

    pub(super) fn handle_pointer_input_with_handler(
        &mut self,
        input_event: NativePanelPointerInput,
        now: Instant,
        input: &NativePanelRuntimeInputDescriptor,
        handler: &mut impl NativePanelRuntimeCommandHandler<Error = String>,
    ) -> Result<NativePanelPointerInputOutcome, String> {
        handle_native_panel_pointer_input_with_handler(self, input_event, now, input, handler)
    }

    pub(super) fn advance_animation_frame_at(
        &mut self,
        now: Instant,
    ) -> Result<Option<reef_ui::native_panel_ui::render::NativePanelAnimationFrame>, String> {
        self.advance_animation_frame_at_impl(now)
    }

    fn apply_animation_frame(
        &mut self,
        frame: reef_ui::native_panel_ui::render::NativePanelAnimationFrame,
    ) -> Result<(), String> {
        self.apply_animation_frame_impl(frame)
    }

    fn resolve_animation_target(
        &self,
        request: NativePanelTransitionRequest,
    ) -> reef_ui::native_panel_ui::render::NativePanelAnimationTarget {
        self.resolve_animation_target_impl(request)
    }

    pub(super) fn resolved_expanded_target_height(&self) -> f64 {
        self.resolved_expanded_target_height_impl()
    }

    fn schedule_next_animation_frame_wake(&mut self, now: Instant) {
        self.schedule_next_animation_frame_wake_impl(now)
    }

    fn schedule_next_status_queue_refresh_wake(&self) {
        self.schedule_next_status_queue_refresh_wake_impl()
    }
}
