use std::time::Instant;

use tracing::info;

use super::{
    host_runtime::WindowsNativePanelRuntime,
    platform_loop::current_windows_native_panel_pointer_polling_sample,
};
use crate::{
    native_panel_core::{HoverTransition, PanelPoint},
    native_panel_renderer::facade::{
        command::NativePanelPointerInput,
        interaction::{
            native_panel_interactive_inside_from_host_facts,
            native_panel_polling_interaction_input_from_host_facts,
            sync_native_panel_host_behavior_for_interactive_inside,
            sync_native_panel_host_polling_interaction_for_state,
            sync_native_panel_hover_interaction_at_point_for_state,
            sync_native_panel_hover_interaction_for_pointer_input_for_state,
            NativePanelHostBehaviorCommand, NativePanelHostPollingInteractionResult,
            NativePanelPollingHostFacts,
        },
        presentation::NativePanelPresentationModel,
        renderer::NativePanelRuntimeSceneStateBridge,
        transition::NativePanelTransitionRequest,
    },
};

impl WindowsNativePanelRuntime {
    pub(super) fn sync_hover_at_point_impl(
        &mut self,
        point: PanelPoint,
        now: Instant,
    ) -> Option<HoverTransition> {
        sync_native_panel_hover_interaction_at_point_for_state(
            &mut self.panel_state,
            &self.host,
            point,
            now,
            crate::native_panel_core::HOVER_DELAY_MS,
        )
    }

    pub(super) fn sync_hover_for_pointer_input_impl(
        &mut self,
        input: NativePanelPointerInput,
        now: Instant,
    ) -> Option<HoverTransition> {
        sync_native_panel_hover_interaction_for_pointer_input_for_state(
            &mut self.panel_state,
            &self.host,
            input,
            now,
            crate::native_panel_core::HOVER_DELAY_MS,
        )
    }

    pub(super) fn sync_host_polling_interaction_impl(
        &mut self,
        pointer: PanelPoint,
        primary_mouse_down: bool,
        now: Instant,
    ) -> Option<NativePanelHostPollingInteractionResult> {
        let input = self
            .polling_host_facts(pointer, primary_mouse_down)
            .map(native_panel_polling_interaction_input_from_host_facts)?;
        let result = sync_native_panel_host_polling_interaction_for_state(
            self,
            input,
            now,
            crate::native_panel_core::HOVER_DELAY_MS,
            crate::native_panel_core::CARD_FOCUS_CLICK_DEBOUNCE_MS,
        );
        self.apply_host_behavior_commands(result.host_behavior.commands.clone());
        Some(result)
    }

    pub(super) fn sync_host_polling_interaction_and_refresh_impl(
        &mut self,
        pointer: PanelPoint,
        primary_mouse_down: bool,
        now: Instant,
        input: &crate::native_panel_renderer::facade::descriptor::NativePanelRuntimeInputDescriptor,
    ) -> Result<Option<NativePanelHostPollingInteractionResult>, String> {
        let Some(result) = self.sync_host_polling_interaction_impl(pointer, primary_mouse_down, now)
        else {
            return Ok(None);
        };
        if let Some(snapshot) = result.transition_snapshot.as_ref() {
            self.sync_snapshot_bundle(snapshot, input)?;
        }
        if let Some(request) = result.transition_request {
            self.last_transition_request = Some(request);
        }
        Ok(Some(result))
    }

    pub(super) fn polling_host_facts(
        &self,
        pointer: PanelPoint,
        primary_mouse_down: bool,
    ) -> Option<NativePanelPollingHostFacts<'_>> {
        self.host.shell.polling_host_facts(
            pointer,
            primary_mouse_down,
            self.runtime_scene_current_snapshot().cloned(),
        )
    }

    pub(super) fn interactive_inside_for_pointer_input(
        &self,
        input: NativePanelPointerInput,
    ) -> Option<bool> {
        match input {
            NativePanelPointerInput::Move(point) => self
                .polling_host_facts(point, false)
                .map(native_panel_interactive_inside_from_host_facts),
            NativePanelPointerInput::Leave => Some(false),
            NativePanelPointerInput::Click(_) => None,
        }
    }

    pub(super) fn sync_mouse_passthrough_for_pointer_input(
        &mut self,
        input: NativePanelPointerInput,
    ) {
        let Some(interactive_inside) = self.interactive_inside_for_pointer_input(input) else {
            return;
        };
        let plan = sync_native_panel_host_behavior_for_interactive_inside(self, interactive_inside);
        self.apply_host_behavior_commands(plan.commands);
    }

    pub(super) fn apply_host_behavior_commands(
        &mut self,
        commands: impl IntoIterator<Item = NativePanelHostBehaviorCommand>,
    ) {
        for command in commands {
            match command {
                NativePanelHostBehaviorCommand::SetMouseEventPassthrough {
                    ignores_mouse_events,
                } => self
                    .host
                    .sync_shell_mouse_event_passthrough(ignores_mouse_events),
            }
        }
    }

    pub(super) fn capture_presentation_for_hover_close_transition(
        &self,
    ) -> Option<NativePanelPresentationModel> {
        self.host
            .window
            .presented_presentation_model
            .as_ref()
            .or(self.host.renderer.last_presentation_model.as_ref())
            .cloned()
            .filter(|presentation| !presentation.card_stack.cards.is_empty())
    }

    pub(super) fn sync_current_pointer_polling_interaction_impl(
        &mut self,
        now: Instant,
        input: &crate::native_panel_renderer::facade::descriptor::NativePanelRuntimeInputDescriptor,
    ) -> Result<Option<NativePanelHostPollingInteractionResult>, String> {
        if self.user_hidden {
            return Ok(None);
        }
        let Some(sample) = current_windows_native_panel_pointer_polling_sample(
            self.host.shell.raw_window_handle(),
        ) else {
            log_windows_native_hover_probe(
                self.host.shell.raw_window_handle(),
                None,
                self.host.shell.pointer_regions().len(),
                None,
                None,
                self.panel_state.expanded,
            );
            return Ok(None);
        };
        let interaction =
            self.sync_host_polling_interaction_and_refresh_impl(sample.point, false, now, input)?;
        log_windows_native_hover_probe(
            self.host.shell.raw_window_handle(),
            Some(sample.point),
            self.host.shell.pointer_regions().len(),
            interaction.as_ref().map(|result| result.interactive_inside),
            interaction
                .as_ref()
                .and_then(|result| result.transition_request),
            self.panel_state.expanded,
        );
        Ok(interaction)
    }
}

fn windows_native_hover_probe_enabled() -> bool {
    std::env::var("ECHOISLAND_WINDOWS_HOVER_PROBE")
        .ok()
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
}

fn log_windows_native_hover_probe(
    raw_window_handle: Option<isize>,
    pointer: Option<PanelPoint>,
    pointer_region_count: usize,
    interactive_inside: Option<bool>,
    transition_request: Option<NativePanelTransitionRequest>,
    expanded: bool,
) {
    if !windows_native_hover_probe_enabled() {
        return;
    }
    info!(
        raw_window_handle = ?raw_window_handle,
        pointer_x = pointer.map(|point| point.x),
        pointer_y = pointer.map(|point| point.y),
        pointer_region_count,
        interactive_inside,
        transition_request = ?transition_request,
        expanded,
        "windows native hover probe"
    );
}
