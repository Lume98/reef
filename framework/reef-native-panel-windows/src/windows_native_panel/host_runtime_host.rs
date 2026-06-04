use crate::native_panel_renderer::facade::{
    descriptor::NativePanelPointerRegion, host::native_panel_presentation_cards_visible,
};

use super::{
    host_runtime::WindowsNativePanelHost,
    host_window::WindowsNativePanelDrawFrame,
    paint_bridge::{
        consume_presenter_into_shell, consume_presenter_into_shell_result, take_pending_draw_frame,
    },
    window_shell::{WindowsNativePanelShellCommand, WindowsNativePanelShellPresentResult},
};

impl WindowsNativePanelHost {
    pub(super) fn record_platform_loop_spawn(&mut self) {
        self.shell.record_platform_loop_spawn();
    }

    pub(super) fn take_pending_draw_frame(&mut self) -> Option<WindowsNativePanelDrawFrame> {
        take_pending_draw_frame(&mut self.presenter)
    }

    pub(super) fn take_pending_shell_commands(&mut self) -> Vec<WindowsNativePanelShellCommand> {
        self.shell.take_pending_commands()
    }

    pub(super) fn sync_shell_mouse_event_passthrough(&mut self, ignores_mouse_events: bool) {
        self.shell
            .sync_mouse_event_passthrough(ignores_mouse_events);
    }

    pub(super) fn consume_presenter_into_shell(&mut self) -> bool {
        consume_presenter_into_shell(&mut self.presenter, &mut self.shell)
    }

    pub(super) fn consume_presenter_into_shell_result(
        &mut self,
    ) -> WindowsNativePanelShellPresentResult {
        consume_presenter_into_shell_result(&mut self.presenter, &mut self.shell)
    }

    pub(super) fn resolved_pointer_regions(&self) -> &[NativePanelPointerRegion] {
        self.window
            .pointer_regions(&self.renderer.last_pointer_regions)
    }

    pub(super) fn cards_visible(&self) -> bool {
        let current = self.renderer.latest_scene_presentation_model();
        native_panel_presentation_cards_visible(
            self.window.presented_presentation_model.as_ref(),
            current.as_ref(),
        )
    }
}
