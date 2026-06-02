use crate::native_panel_core::{
    resolve_native_panel_host_frame, PanelAnimationDescriptor, PanelAnimationKind, PanelRect,
};

#[derive(Clone, Debug, PartialEq)]
pub struct NativePanelRuntimeInputDescriptor {
    pub scene_input: crate::native_panel_scene::PanelSceneBuildInput,
    pub screen_frame: Option<PanelRect>,
}

impl NativePanelRuntimeInputDescriptor {
    pub fn selected_display_index(&self) -> usize {
        self.scene_input.settings.selected_display_index
    }
}

pub fn native_panel_runtime_input_descriptor_with_screen_frame(
    screen_frame: Option<PanelRect>,
) -> NativePanelRuntimeInputDescriptor {
    NativePanelRuntimeInputDescriptor {
        scene_input: crate::native_panel_scene::PanelSceneBuildInput::default(),
        screen_frame,
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NativePanelRuntimeInputContext {
    pub display_options: Vec<crate::native_panel_scene::PanelDisplayOptionState>,
    pub selected_display_index: usize,
    pub screen_frame: Option<PanelRect>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NativePanelHostWindowState {
    pub frame: Option<PanelRect>,
    pub visible: bool,
    pub preferred_display_index: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NativePanelHostWindowDescriptor {
    pub visible: bool,
    pub preferred_display_index: usize,
    pub screen_frame: Option<PanelRect>,
    pub shared_body_height: Option<f64>,
    pub timeline: Option<NativePanelTimelineDescriptor>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NativePanelHostWindowDescriptorPatch {
    pub visible: Option<bool>,
    pub preferred_display_index: Option<usize>,
    pub screen_frame: Option<Option<PanelRect>>,
    pub shared_body_height: Option<Option<f64>>,
    pub timeline: Option<Option<NativePanelTimelineDescriptor>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelTimelineDescriptor {
    pub animation: PanelAnimationDescriptor,
    pub cards_entering: bool,
}

impl NativePanelHostWindowDescriptor {
    pub fn animation_descriptor(self) -> Option<PanelAnimationDescriptor> {
        self.timeline.map(|timeline| timeline.animation)
    }

    pub fn window_state(self, frame: Option<PanelRect>) -> NativePanelHostWindowState {
        NativePanelHostWindowState {
            frame,
            visible: self.visible,
            preferred_display_index: self.preferred_display_index,
        }
    }
}

pub fn native_panel_host_window_descriptor(
    visible: bool,
    preferred_display_index: usize,
    screen_frame: Option<PanelRect>,
    shared_body_height: Option<f64>,
    timeline: Option<NativePanelTimelineDescriptor>,
) -> NativePanelHostWindowDescriptor {
    NativePanelHostWindowDescriptor {
        visible,
        preferred_display_index,
        screen_frame,
        shared_body_height,
        timeline,
    }
}

pub fn native_panel_timeline_descriptor(
    animation: PanelAnimationDescriptor,
    cards_entering: bool,
) -> NativePanelTimelineDescriptor {
    NativePanelTimelineDescriptor {
        animation,
        cards_entering,
    }
}

pub fn native_panel_timeline_descriptor_for_animation(
    animation: PanelAnimationDescriptor,
) -> NativePanelTimelineDescriptor {
    native_panel_timeline_descriptor(
        animation,
        native_panel_cards_entering_for_animation(animation),
    )
}

pub fn native_panel_cards_entering_for_animation(animation: PanelAnimationDescriptor) -> bool {
    !matches!(animation.kind, PanelAnimationKind::Close)
}

pub fn sync_native_panel_host_window_visibility(
    descriptor: &mut NativePanelHostWindowDescriptor,
    visible: bool,
) {
    patch_native_panel_host_window_descriptor(
        descriptor,
        NativePanelHostWindowDescriptorPatch {
            visible: Some(visible),
            ..NativePanelHostWindowDescriptorPatch::default()
        },
    );
}

pub fn sync_native_panel_host_window_screen_frame(
    descriptor: &mut NativePanelHostWindowDescriptor,
    preferred_display_index: usize,
    screen_frame: Option<PanelRect>,
) {
    patch_native_panel_host_window_descriptor(
        descriptor,
        NativePanelHostWindowDescriptorPatch {
            preferred_display_index: Some(preferred_display_index),
            screen_frame: Some(screen_frame),
            ..NativePanelHostWindowDescriptorPatch::default()
        },
    );
}

pub fn sync_native_panel_host_window_shared_body_height(
    descriptor: &mut NativePanelHostWindowDescriptor,
    shared_body_height: Option<f64>,
) {
    patch_native_panel_host_window_descriptor(
        descriptor,
        NativePanelHostWindowDescriptorPatch {
            shared_body_height: Some(shared_body_height),
            ..NativePanelHostWindowDescriptorPatch::default()
        },
    );
}

pub fn sync_native_panel_host_window_timeline(
    descriptor: &mut NativePanelHostWindowDescriptor,
    timeline: Option<NativePanelTimelineDescriptor>,
) {
    patch_native_panel_host_window_descriptor(
        descriptor,
        NativePanelHostWindowDescriptorPatch {
            timeline: Some(timeline),
            ..NativePanelHostWindowDescriptorPatch::default()
        },
    );
}

pub fn patch_native_panel_host_window_descriptor(
    descriptor: &mut NativePanelHostWindowDescriptor,
    patch: NativePanelHostWindowDescriptorPatch,
) {
    if let Some(visible) = patch.visible {
        descriptor.visible = visible;
    }
    if let Some(preferred_display_index) = patch.preferred_display_index {
        descriptor.preferred_display_index = preferred_display_index;
    }
    if let Some(screen_frame) = patch.screen_frame {
        descriptor.screen_frame = screen_frame;
    }
    if let Some(shared_body_height) = patch.shared_body_height {
        descriptor.shared_body_height = shared_body_height;
    }
    if let Some(timeline) = patch.timeline {
        descriptor.timeline = timeline;
    }
}

pub fn native_panel_host_window_frame(
    descriptor: NativePanelHostWindowDescriptor,
    fallback_screen_frame: PanelRect,
    compact_width: f64,
    expanded_width: f64,
) -> Option<PanelRect> {
    Some(resolve_native_panel_host_frame(
        descriptor.animation_descriptor()?,
        descriptor.screen_frame.unwrap_or(fallback_screen_frame),
        compact_width,
        expanded_width,
    ))
}
