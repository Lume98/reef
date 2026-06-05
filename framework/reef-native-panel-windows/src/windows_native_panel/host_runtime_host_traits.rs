use crate::native_panel_renderer::facade::{
    command::NativePanelPlatformEvent,
    descriptor::{
        NativePanelHostWindowDescriptor, NativePanelHostWindowState, NativePanelPointerRegion,
    },
    host::{
        NativePanelHost, NativePanelHostDisplayReposition, NativePanelRuntimeHostController,
        NativePanelSceneHost,
    },
    interaction::{
        NativePanelPointerRegionInteractionBridge, NativePanelQueuedPlatformEventBridge,
    },
};

use super::{
    host_runtime::WindowsNativePanelHost, paint_bridge::present_window_into_presenter,
    WindowsNativePanelRenderer,
};

impl NativePanelHost for WindowsNativePanelHost {
    type Error = String;
    type Renderer = WindowsNativePanelRenderer;

    fn renderer(&mut self) -> &mut Self::Renderer {
        &mut self.renderer
    }

    fn host_window_descriptor(&self) -> NativePanelHostWindowDescriptor {
        self.window.descriptor
    }

    fn host_window_descriptor_mut(&mut self) -> &mut NativePanelHostWindowDescriptor {
        &mut self.window.descriptor
    }

    fn window_state(&self) -> NativePanelHostWindowState {
        self.window.window_state()
    }

    fn create(&mut self) -> Result<(), Self::Error> {
        self.window.create();
        self.shell.create();
        self.shell.sync_window_state(self.window.window_state());
        self.sync_renderer_host_window_descriptor()
    }

    fn after_host_window_descriptor_updated(&mut self) -> Result<(), Self::Error> {
        self.window.refresh_frame_from_descriptor();
        self.shell.sync_window_state(self.window.window_state());
        Ok(())
    }

    fn show(&mut self) -> Result<(), Self::Error> {
        NativePanelHost::create(self)?;
        self.window.show();
        self.shell.show();
        self.shell.sync_window_state(self.window.window_state());
        self.sync_renderer_host_window_descriptor()
    }

    fn hide(&mut self) -> Result<(), Self::Error> {
        self.window.hide();
        self.shell.hide();
        self.shell.sync_window_state(self.window.window_state());
        self.sync_renderer_host_window_descriptor()
    }

    fn take_platform_events(&mut self) -> Vec<NativePanelPlatformEvent> {
        std::mem::take(&mut self.pending_events)
    }

    fn present_renderer_state(&mut self) -> Result<(), Self::Error> {
        let window_state = self
            .renderer
            .last_window_state
            .unwrap_or_else(|| self.window.window_state());
        present_window_into_presenter(
            &mut self.window,
            &mut self.presenter,
            window_state,
            &self.renderer.last_pointer_regions,
            self.renderer.latest_scene_presentation_model(),
        );
        Ok(())
    }
}

impl NativePanelSceneHost for WindowsNativePanelHost {}

impl NativePanelPointerRegionInteractionBridge for WindowsNativePanelHost {
    fn interaction_pointer_regions(&self) -> &[NativePanelPointerRegion] {
        self.resolved_pointer_regions()
    }

    fn interaction_cards_visible(&self) -> bool {
        self.cards_visible()
    }
}

impl NativePanelQueuedPlatformEventBridge for WindowsNativePanelHost {
    fn queued_platform_events_mut(&mut self) -> &mut Vec<NativePanelPlatformEvent> {
        &mut self.pending_events
    }

    fn queued_pointer_regions(&self) -> &[NativePanelPointerRegion] {
        self.resolved_pointer_regions()
    }
}

impl NativePanelRuntimeHostController for WindowsNativePanelHost {
    type Error = String;

    fn runtime_host_create_panel(&mut self) -> Result<(), Self::Error> {
        self.show()
    }

    fn runtime_host_hide_panel(&mut self) -> Result<(), Self::Error> {
        self.hide()
    }

    fn runtime_host_reposition(
        &mut self,
        reposition: NativePanelHostDisplayReposition,
    ) -> Result<(), Self::Error> {
        self.reposition_to_display_with_payload(reposition)
    }

    fn runtime_host_set_shared_body_height(&mut self, body_height: f64) -> Result<(), Self::Error> {
        self.set_shared_body_height(body_height)
    }
}
