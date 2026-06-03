use std::time::{Duration, Instant};

use super::host_runtime::WindowsNativePanelRuntime;
use crate::{
    native_panel_core::{
        panel_state_needs_status_queue_refresh, take_pending_status_reopen_after_transition,
    },
    native_panel_renderer::facade::{
        host::NativePanelHost,
        renderer::NativePanelCloseTrigger,
        transition::NativePanelTransitionRequest,
    },
};

impl WindowsNativePanelRuntime {
    pub(super) fn refresh_active_count_marquee_frame_at_impl(&mut self, now: Instant) -> bool {
        let plan = self.lightweight_refresh_plan_impl();
        if plan.active_count_marquee.reset_timer {
            self.active_count_marquee_started_at = None;
            return false;
        }
        if !plan.active_count_marquee.refresh_allowed {
            return false;
        }
        let started_at = *self.active_count_marquee_started_at.get_or_insert(now);
        self.host
            .shell
            .refresh_active_count_marquee(now.duration_since(started_at).as_millis())
    }

    pub(super) fn refresh_mascot_animation_frame_at_impl(&mut self, now: Instant) -> bool {
        let plan = self.lightweight_refresh_plan_impl();
        if plan.mascot_animation.reset_timer {
            self.mascot_animation_started_at = None;
            return false;
        }
        if !plan.mascot_animation.refresh_allowed {
            return false;
        }
        let Some(started_at) = self.mascot_animation_started_at else {
            self.mascot_animation_started_at = Some(now);
            return self.host.shell.pending_paint_job().is_none()
                && self.host.shell.refresh_mascot_animation(0);
        };
        self.host
            .shell
            .refresh_mascot_animation(now.duration_since(started_at).as_millis())
    }

    pub(super) fn lightweight_refresh_plan_impl(
        &self,
    ) -> crate::native_panel_core::NativePanelLightweightRefreshPlan {
        crate::native_panel_core::resolve_native_panel_lightweight_refresh_plan(
            crate::native_panel_core::NativePanelLightweightRefreshInput {
                transitioning: self.panel_state.transitioning,
                animation_active: self.animation_scheduler.is_active(),
                active_count_marquee_needs_refresh: self
                    .host
                    .shell
                    .active_count_marquee_needs_refresh(),
                mascot_animation_needs_refresh: self.host.shell.mascot_animation_needs_refresh(),
            },
        )
    }

    pub(super) fn advance_animation_frame_at_impl(
        &mut self,
        now: Instant,
    ) -> Result<Option<reef_ui::native_panel_ui::render::NativePanelAnimationFrame>, String> {
        if let Some(request) = self.last_transition_request.take() {
            if request != NativePanelTransitionRequest::Close {
                self.hover_close_in_progress = false;
                self.pending_close_presentation = None;
            }
            let close_preservation = self.status_close_preservation_plan_for_request(Some(request));
            if request == NativePanelTransitionRequest::Close
                && close_preservation.should_prepare_close_animation_stack
            {
                let preserved_presentation = self
                    .pending_close_presentation
                    .take()
                    .or_else(|| self.capture_status_presentation_for_close_transition());
                let plan = self.close_presentation_plan_for_request(
                    NativePanelCloseTrigger::StatusAuto,
                    Some(request),
                    preserved_presentation.is_some(),
                );
                self.panel_state.skip_next_close_card_exit = false;
                self.host
                    .renderer
                    .apply_close_presentation_plan(preserved_presentation.as_ref(), plan);
            }
            if request == NativePanelTransitionRequest::Open
                && self.panel_state.status_auto_expanded
                && self.panel_state.surface_mode
                    == crate::native_panel_core::ExpandedSurface::Status
            {
                let input = crate::native_panel_renderer::facade::descriptor::NativePanelRuntimeInputDescriptor {
                    scene_input: Default::default(),
                    screen_frame: self.host.window.descriptor.screen_frame,
                };
                self.rerender_from_last_snapshot_with_input(&input)?;
            }
            let target = self.resolve_animation_target_impl(request);
            self.panel_state.transitioning = true;
            let frame = self.animation_scheduler.start(target, now);
            self.apply_animation_frame_impl(frame)?;
            return Ok(Some(frame));
        }

        let Some(frame) = self.animation_scheduler.sample(now) else {
            self.next_animation_wake_at = None;
            return Ok(None);
        };
        self.apply_animation_frame_impl(frame)?;
        Ok(Some(frame))
    }

    pub(super) fn apply_animation_frame_impl(
        &mut self,
        frame: reef_ui::native_panel_ui::render::NativePanelAnimationFrame,
    ) -> Result<(), String> {
        self.host.apply_timeline_descriptor(frame.plan.timeline)?;
        self.last_animation_descriptor = Some(frame.plan.timeline.animation);
        if !frame.continue_animating {
            self.panel_state.transitioning = false;
            self.next_animation_wake_at = None;
            if frame.descriptor.animation.kind
                == crate::native_panel_core::PanelAnimationKind::Close
            {
                self.hover_close_in_progress = false;
                self.pending_close_presentation = None;
                if take_pending_status_reopen_after_transition(&mut self.panel_state) {
                    self.last_transition_request = Some(NativePanelTransitionRequest::Open);
                }
            }
        }
        Ok(())
    }

    pub(super) fn resolve_animation_target_impl(
        &self,
        request: NativePanelTransitionRequest,
    ) -> reef_ui::native_panel_ui::render::NativePanelAnimationTarget {
        let start_height = self
            .host
            .renderer
            .last_animation_descriptor
            .map(|descriptor| descriptor.visible_height)
            .unwrap_or(crate::native_panel_core::COLLAPSED_PANEL_HEIGHT);
        let target_height = match request {
            NativePanelTransitionRequest::Close => crate::native_panel_core::COLLAPSED_PANEL_HEIGHT,
            NativePanelTransitionRequest::Open | NativePanelTransitionRequest::SurfaceSwitch => {
                self.resolved_expanded_target_height_impl()
            }
        };
        reef_ui::native_panel_ui::render::NativePanelAnimationTarget {
            request,
            start_height,
            target_height,
            card_count: self
                .host
                .renderer
                .scene_cache
                .last_scene
                .as_ref()
                .map(|scene| scene.cards.len())
                .unwrap_or_default(),
        }
    }

    pub(super) fn resolved_expanded_target_height_impl(&self) -> f64 {
        let body_height = self
            .host
            .renderer
            .latest_scene_body_height_for_current_width()
            .or_else(|| {
                self.host
                    .renderer
                    .latest_scene_presentation_model()
                    .map(|presentation| presentation.metrics.expanded_body_height)
            })
            .or(self.host.window.descriptor.shared_body_height)
            .unwrap_or(0.0);
        (crate::native_panel_core::DEFAULT_COMPACT_PILL_HEIGHT
            + crate::native_panel_core::EXPANDED_CONTENT_TOP_GAP
            + body_height
            + crate::native_panel_core::EXPANDED_CONTENT_BOTTOM_INSET)
            .max(crate::native_panel_core::COLLAPSED_PANEL_HEIGHT)
    }

    pub(super) fn schedule_next_animation_frame_wake_impl(&mut self, now: Instant) {
        let Some(delay_ms) = self.animation_scheduler.next_frame_delay_ms() else {
            self.next_animation_wake_at = None;
            return;
        };
        let next_wake = now + Duration::from_millis(delay_ms);
        if self
            .next_animation_wake_at
            .is_some_and(|scheduled| scheduled > now)
        {
            return;
        }
        self.next_animation_wake_at = Some(next_wake);
        super::platform_loop::schedule_windows_native_platform_loop_wake(delay_ms);
    }

    pub(super) fn schedule_next_status_queue_refresh_wake_impl(&self) {
        if panel_state_needs_status_queue_refresh(&self.panel_state) {
            super::platform_loop::schedule_windows_native_platform_loop_wake(
                crate::native_panel_core::STATUS_QUEUE_REFRESH_MS,
            );
            return;
        }
        let lightweight_refresh = self.lightweight_refresh_plan_impl();
        if lightweight_refresh.active_count_marquee.refresh_allowed {
            super::platform_loop::schedule_windows_native_platform_loop_wake(
                crate::native_panel_core::ACTIVE_COUNT_SCROLL_REFRESH_MS,
            );
            return;
        }
        if lightweight_refresh.mascot_animation.refresh_allowed {
            super::platform_loop::schedule_windows_native_platform_loop_wake(
                crate::native_panel_core::MASCOT_ANIMATION_REFRESH_MS,
            );
            return;
        }
        super::platform_loop::schedule_windows_native_platform_loop_wake(
            crate::native_panel_core::HOVER_POLL_MS,
        );
    }
}
