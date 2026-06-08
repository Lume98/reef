use std::time::Instant;

use super::{
    draw_presenter::WindowsPanelDrawPresenter,
    host_window::WindowsPanelHostWindow,
    message_dispatch::pump_window_messages as pump_dispatched_window_messages,
    platform_loop::WindowsPanelPlatformLoopState,
    window_shell::{
        panel_point_from_window_lparam, WindowsPanelWindowShell, WINDOWS_WM_LBUTTONDOWN,
        WINDOWS_WM_LBUTTONUP, WINDOWS_WM_MOUSELEAVE,
    },
    WindowsPanelRenderer,
};
use crate::{
    page::{
        dynamic_island_target_for_hit_target, is_dynamic_island_horizontal_swipe,
        resolve_dynamic_island_gesture, resolve_dynamic_island_gesture_effect,
        resolve_dynamic_island_root_gesture_at_point, resolve_dynamic_island_target_effect,
        DynamicIslandInteractionContext, DynamicIslandInteractionEffect,
        DynamicIslandRuntimeEffect, DynamicIslandSwipeSpec,
    },
    runtime::facade::{
        command::{
            dispatch_native_panel_click_command_with_handler, dispatch_native_panel_platform_event,
            NativePanelPlatformEvent, NativePanelPointerInput, NativePanelPointerInputOutcome,
            NativePanelRuntimeCommandHandler,
        },
        descriptor::NativePanelRuntimeInputDescriptor,
        host::{
            create_native_panel_via_host_controller, hide_native_panel_via_host_controller,
            reposition_native_panel_host_from_input_descriptor_via_controller,
            set_native_panel_host_shared_body_height_via_controller, NativePanelHost,
        },
        interaction::{
            handle_native_panel_pointer_input_with_handler,
            handle_optional_native_panel_pointer_input_with_handler,
            native_panel_click_state_slots, resolve_native_panel_click_command_for_pointer_state,
            sync_native_panel_hover_and_refresh_for_runtime,
            sync_native_panel_hover_interaction_and_rerender_at_point_with_input_descriptor,
            sync_native_panel_hover_interaction_and_rerender_for_inside_with_input_descriptor,
            sync_native_panel_hover_interaction_and_rerender_for_pointer_input_with_input_descriptor,
            sync_native_panel_hover_interaction_for_state, NativePanelClickInteractionHost,
            NativePanelHoverSyncResult, NativePanelSettingsSurfaceToggleResult,
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
        shell::pump_native_panel_host_shell_runtime,
        transition::NativePanelTransitionRequest,
    },
    state::{
        ExpandedSurface, HoverTransition, PanelAnimationDescriptor, PanelInteractionCommand,
        PanelPoint, PanelState,
    },
};

#[derive(Default)]
pub(crate) struct WindowsPanelHost {
    pub(super) renderer: WindowsPanelRenderer,
    pub(super) window: WindowsPanelHostWindow,
    pub(super) presenter: WindowsPanelDrawPresenter,
    pub(super) shell: WindowsPanelWindowShell,
    pub(super) pending_events: Vec<NativePanelPlatformEvent>,
}

#[derive(Default)]
pub(crate) struct WindowsPanelRuntime {
    pub(super) panel_state: PanelState,
    pub(super) primary_pointer_down: bool,
    pub(super) ignores_mouse_events: bool,
    pub(super) user_hidden: bool,
    pub(super) host: WindowsPanelHost,
    pub(super) platform_loop: WindowsPanelPlatformLoopState,
    pub(super) scene_cache: NativePanelRuntimeSceneCache,
    pub(super) animation_scheduler: crate::presentation::render::NativePanelAnimationFrameScheduler,
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
    pub(super) pointer_drag_origin: Option<PanelPoint>,
}

impl WindowsPanelRuntime {
    pub(super) fn pump_platform_loop(&mut self) -> Result<(), String> {
        if self.host.shell.has_pending_destroy_command() {
            super::platform_loop::clear_windows_panel_hit_regions(
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
        self.panel_state.surface_mode = crate::state::ExpandedSurface::Default;
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
            &mut WindowsPanelHost,
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
            crate::state::HOVER_DELAY_MS,
        )
        .transition
    }

    pub(super) fn refresh_active_count_marquee_frame_at(&mut self, now: Instant) -> bool {
        self.refresh_active_count_marquee_frame_at_impl(now)
    }

    pub(super) fn refresh_mascot_animation_frame_at(&mut self, now: Instant) -> bool {
        self.refresh_mascot_animation_frame_at_impl(now)
    }

    fn lightweight_refresh_plan(&self) -> crate::state::NativePanelLightweightRefreshPlan {
        self.lightweight_refresh_plan_impl()
    }

    pub(super) fn rerender_from_last_snapshot_with_input(
        &mut self,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<bool, String> {
        rerender_runtime_scene_sync_result_to_host_for_runtime_with_input_descriptor(self, input)
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
                crate::state::HOVER_DELAY_MS,
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
                crate::state::HOVER_DELAY_MS,
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
                crate::state::HOVER_DELAY_MS,
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
        let pointer_state = self.host.click_pointer_state_at_point(point);
        let cards_visible = self.host.click_cards_visible() || pointer_state.hit_target.is_some();
        let mut click_state =
            native_panel_click_state_slots(&self.panel_state, &mut self.last_focus_click);
        let command = resolve_native_panel_click_command_for_pointer_state(
            &mut click_state,
            &pointer_state,
            true,
            cards_visible,
            now,
            crate::state::CARD_FOCUS_CLICK_DEBOUNCE_MS,
        );

        match self
            .dispatch_declarative_target_click_command_with_handler(command.clone(), handler)?
        {
            Some(event) => Ok(Some(event)),
            None => match dispatch_native_panel_click_command_with_handler(handler, command)? {
                Some(event) => Ok(Some(event)),
                None => self.dispatch_declarative_click_fallback_with_handler(point, handler),
            },
        }
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
        if message_id == WINDOWS_WM_LBUTTONDOWN {
            self.pointer_drag_origin = Some(panel_point_from_window_lparam(lparam));
            return Ok(None);
        }
        if message_id == WINDOWS_WM_MOUSELEAVE {
            self.pointer_drag_origin = None;
        }
        if let Some(outcome) = self.handle_swipe_window_message_with_handler(message_id, lparam)? {
            return Ok(Some(outcome));
        }
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
    ) -> Result<Option<crate::presentation::render::NativePanelAnimationFrame>, String> {
        self.advance_animation_frame_at_impl(now)
    }

    fn apply_animation_frame(
        &mut self,
        frame: crate::presentation::render::NativePanelAnimationFrame,
    ) -> Result<(), String> {
        self.apply_animation_frame_impl(frame)
    }

    fn resolve_animation_target(
        &self,
        request: NativePanelTransitionRequest,
    ) -> crate::presentation::render::NativePanelAnimationTarget {
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

    fn dispatch_declarative_click_fallback_with_handler<H>(
        &mut self,
        point: PanelPoint,
        handler: &mut H,
    ) -> Result<Option<NativePanelPlatformEvent>, H::Error>
    where
        H: NativePanelRuntimeCommandHandler,
    {
        let Some(snapshot) = self.scene_cache.last_snapshot.as_ref() else {
            return Ok(None);
        };
        let effect = resolve_dynamic_island_root_gesture_at_point(
            self.host.resolved_pointer_regions(),
            point,
            DynamicIslandInteractionContext {
                snapshot,
                panel_expanded: self.panel_state.expanded,
                settings_active: self.panel_state.surface_mode == ExpandedSurface::Settings,
            },
            crate::page::DynamicIslandGesture::Click,
            |context, gesture| {
                resolve_dynamic_island_gesture_effect(
                    context.snapshot,
                    context.panel_expanded,
                    context.settings_active,
                    gesture,
                )
                .map(map_dynamic_island_runtime_effect)
            },
        );

        match effect {
            Some(DynamicIslandInteractionEffect::PlatformEvent(event)) => {
                crate::presentation::render::dispatch_native_panel_platform_event(
                    handler,
                    event.clone(),
                )?;
                Ok(Some(event))
            }
            Some(DynamicIslandInteractionEffect::Transition(request)) => {
                self.last_transition_request = Some(request);
                Ok(None)
            }
            None => Ok(None),
        }
    }

    fn dispatch_declarative_target_click_command_with_handler<H>(
        &mut self,
        command: PanelInteractionCommand,
        handler: &mut H,
    ) -> Result<Option<NativePanelPlatformEvent>, H::Error>
    where
        H: NativePanelRuntimeCommandHandler,
    {
        let PanelInteractionCommand::HitTarget(target) = command else {
            return Ok(None);
        };
        let Some(snapshot) = self.scene_cache.last_snapshot.as_ref() else {
            return Ok(None);
        };
        let Some(target_key) = dynamic_island_target_for_hit_target(&target) else {
            return Ok(None);
        };
        let Some(effect) = resolve_dynamic_island_target_effect(
            snapshot,
            self.panel_state.expanded,
            self.panel_state.surface_mode == ExpandedSurface::Settings,
            &target_key,
            crate::page::DynamicIslandGesture::Click,
        )
        .map(map_dynamic_island_runtime_effect) else {
            return Ok(None);
        };

        match effect {
            DynamicIslandInteractionEffect::PlatformEvent(event) => {
                dispatch_native_panel_platform_event(handler, event.clone())?;
                Ok(Some(event))
            }
            DynamicIslandInteractionEffect::Transition(request) => {
                self.last_transition_request = Some(request);
                Ok(None)
            }
        }
    }

    fn handle_swipe_window_message_with_handler(
        &mut self,
        message_id: u32,
        lparam: isize,
    ) -> Result<Option<NativePanelPointerInputOutcome>, String> {
        if message_id != WINDOWS_WM_LBUTTONUP {
            return Ok(None);
        }

        let end = panel_point_from_window_lparam(lparam);
        let Some(start) = self.pointer_drag_origin.take() else {
            return Ok(None);
        };
        if !is_dynamic_island_horizontal_swipe(start, end, DynamicIslandSwipeSpec::default()) {
            return Ok(None);
        }

        let Some(snapshot) = self.scene_cache.last_snapshot.as_ref() else {
            return Ok(None);
        };
        if let Some(DynamicIslandInteractionEffect::Transition(request)) =
            resolve_dynamic_island_gesture(
                DynamicIslandInteractionContext {
                    snapshot,
                    panel_expanded: self.panel_state.expanded,
                    settings_active: self.panel_state.surface_mode == ExpandedSurface::Settings,
                },
                crate::page::DynamicIslandGesture::Swipe,
                |context, gesture| {
                    resolve_dynamic_island_gesture_effect(
                        context.snapshot,
                        context.panel_expanded,
                        context.settings_active,
                        gesture,
                    )
                    .map(map_dynamic_island_runtime_effect)
                },
            )
        {
            self.last_transition_request = Some(request);
            return Ok(Some(NativePanelPointerInputOutcome::Click(None)));
        }

        Ok(None)
    }
}

fn map_dynamic_island_runtime_effect(
    effect: DynamicIslandRuntimeEffect,
) -> DynamicIslandInteractionEffect<NativePanelPlatformEvent, NativePanelTransitionRequest> {
    match effect {
        DynamicIslandRuntimeEffect::PlatformEvent(event) => {
            DynamicIslandInteractionEffect::PlatformEvent(event)
        }
        DynamicIslandRuntimeEffect::Transition(request) => {
            DynamicIslandInteractionEffect::Transition(request)
        }
    }
}
