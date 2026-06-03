use super::host_runtime::WindowsNativePanelRuntime;
use crate::{
    native_panel_core::HoverTransition,
    native_panel_renderer::facade::{
        command::NativePanelPointerInput,
        descriptor::NativePanelRuntimeInputDescriptor,
        interaction::NativePanelHostPollingInteractionResult,
    },
};

impl WindowsNativePanelRuntime {
    pub(super) fn sync_hover_at_point(
        &mut self,
        point: crate::native_panel_core::PanelPoint,
        now: std::time::Instant,
    ) -> Option<HoverTransition> {
        self.sync_hover_at_point_impl(point, now)
    }

    pub(super) fn sync_hover_for_pointer_input(
        &mut self,
        input: NativePanelPointerInput,
        now: std::time::Instant,
    ) -> Option<HoverTransition> {
        self.sync_hover_for_pointer_input_impl(input, now)
    }

    pub(super) fn sync_host_polling_interaction(
        &mut self,
        pointer: crate::native_panel_core::PanelPoint,
        primary_mouse_down: bool,
        now: std::time::Instant,
    ) -> Option<NativePanelHostPollingInteractionResult> {
        self.sync_host_polling_interaction_impl(pointer, primary_mouse_down, now)
    }

    pub(super) fn sync_host_polling_interaction_and_refresh(
        &mut self,
        pointer: crate::native_panel_core::PanelPoint,
        primary_mouse_down: bool,
        now: std::time::Instant,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<Option<NativePanelHostPollingInteractionResult>, String> {
        self.sync_host_polling_interaction_and_refresh_impl(pointer, primary_mouse_down, now, input)
    }

    pub(super) fn sync_current_pointer_polling_interaction(
        &mut self,
        now: std::time::Instant,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<Option<NativePanelHostPollingInteractionResult>, String> {
        self.sync_current_pointer_polling_interaction_impl(now, input)
    }
}
