use crate::native_panel_core::PanelRect;

use super::descriptors::{
    native_panel_host_window_frame, sync_native_panel_host_window_shared_body_height,
    sync_native_panel_host_window_timeline, sync_native_panel_host_window_visibility,
    NativePanelHostWindowDescriptor, NativePanelHostWindowState, NativePanelPointerRegion,
    NativePanelTimelineDescriptor,
};
use super::host_runtime_command::{
    native_panel_host_display_reposition, sync_native_panel_host_display_reposition,
    NativePanelHostDisplayReposition,
};

pub(crate) fn sync_runtime_host_visibility_in_state<S>(state: &mut S, visible: bool)
where
    S: super::host_runtime_command::NativePanelRuntimeHostState,
{
    sync_native_panel_host_window_visibility(state.host_window_descriptor_mut(), visible);
}

pub(crate) fn sync_runtime_pointer_regions_in_state<S>(
    state: &mut S,
    regions: &[NativePanelPointerRegion],
) where
    S: super::host_runtime_command::NativePanelRuntimeHostState,
{
    *state.pointer_regions_mut() = regions.to_vec();
}

pub(crate) fn sync_runtime_host_screen_frame_in_state<S>(
    state: &mut S,
    preferred_display_index: usize,
    screen_frame: PanelRect,
) where
    S: super::host_runtime_command::NativePanelRuntimeHostState,
{
    sync_native_panel_host_display_reposition(
        state.host_window_descriptor_mut(),
        native_panel_host_display_reposition(preferred_display_index, Some(screen_frame)),
    );
}

pub(crate) fn sync_runtime_host_display_reposition_in_state<S>(
    state: &mut S,
    reposition: NativePanelHostDisplayReposition,
) where
    S: super::host_runtime_command::NativePanelRuntimeHostState,
{
    sync_native_panel_host_display_reposition(state.host_window_descriptor_mut(), reposition);
}

pub(crate) fn sync_runtime_host_shared_body_height_in_state<S>(
    state: &mut S,
    shared_body_height: Option<f64>,
) where
    S: super::host_runtime_command::NativePanelRuntimeHostState,
{
    sync_native_panel_host_window_shared_body_height(
        state.host_window_descriptor_mut(),
        shared_body_height,
    );
}

pub(crate) fn sync_runtime_host_timeline_in_state<S>(
    state: &mut S,
    descriptor: NativePanelTimelineDescriptor,
) where
    S: super::host_runtime_command::NativePanelRuntimeHostState,
{
    sync_native_panel_host_window_timeline(state.host_window_descriptor_mut(), Some(descriptor));
}

pub(crate) trait NativePanelComputedHostWindow {
    fn host_window_descriptor(&self) -> NativePanelHostWindowDescriptor;

    fn host_window_descriptor_mut(&mut self) -> &mut NativePanelHostWindowDescriptor;

    fn host_window_last_frame_mut(&mut self) -> &mut Option<PanelRect>;

    fn host_window_frame(&self) -> Option<PanelRect>;

    fn set_host_window_created(&mut self);

    fn fallback_screen_frame(&self) -> PanelRect;

    fn compact_width(&self) -> f64;

    fn expanded_width(&self) -> f64;

    fn host_window_create(&mut self) {
        self.set_host_window_created();
    }

    fn host_window_show(&mut self) {
        self.host_window_create();
        sync_native_panel_host_window_visibility(self.host_window_descriptor_mut(), true);
    }

    fn host_window_hide(&mut self) {
        sync_native_panel_host_window_visibility(self.host_window_descriptor_mut(), false);
    }

    fn host_window_reposition_to_display(
        &mut self,
        preferred_display_index: usize,
        screen_frame: Option<PanelRect>,
    ) {
        self.host_window_create();
        sync_native_panel_host_display_reposition(
            self.host_window_descriptor_mut(),
            native_panel_host_display_reposition(preferred_display_index, screen_frame),
        );
        self.refresh_host_window_frame_from_descriptor();
    }

    fn host_window_set_shared_body_height(&mut self, body_height: f64) {
        sync_native_panel_host_window_shared_body_height(
            self.host_window_descriptor_mut(),
            Some(body_height),
        );
    }

    fn host_window_apply_timeline_descriptor(&mut self, descriptor: NativePanelTimelineDescriptor) {
        self.host_window_create();
        sync_native_panel_host_window_timeline(self.host_window_descriptor_mut(), Some(descriptor));
        self.refresh_host_window_frame_from_descriptor();
    }

    fn refresh_host_window_frame_from_descriptor(&mut self) {
        let descriptor = self.host_window_descriptor();
        if descriptor.animation_descriptor().is_none() {
            return;
        }
        *self.host_window_last_frame_mut() = native_panel_host_window_frame(
            descriptor,
            descriptor
                .screen_frame
                .unwrap_or_else(|| self.fallback_screen_frame()),
            self.compact_width(),
            self.expanded_width(),
        );
    }

    fn computed_host_window_state(&self) -> NativePanelHostWindowState {
        self.host_window_descriptor()
            .window_state(self.host_window_frame())
    }
}
