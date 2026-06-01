use super::PanelTransitionFrame;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelAnimationKind {
    Open,
    Close,
    SurfaceSwitch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelAnimationDescriptor {
    pub kind: PanelAnimationKind,
    pub canvas_height: f64,
    pub visible_height: f64,
    pub width_progress: f64,
    pub height_progress: f64,
    pub shoulder_progress: f64,
    pub drop_progress: f64,
    pub cards_progress: f64,
}

pub fn resolve_panel_animation_descriptor(
    kind: PanelAnimationKind,
    frame: PanelTransitionFrame,
) -> PanelAnimationDescriptor {
    PanelAnimationDescriptor {
        kind,
        canvas_height: frame.canvas_height,
        visible_height: frame.visible_height,
        width_progress: frame.bar_progress.clamp(0.0, 1.0),
        height_progress: frame.height_progress.clamp(0.0, 1.0),
        shoulder_progress: frame.shoulder_progress.clamp(0.0, 1.0),
        drop_progress: frame.drop_progress.clamp(0.0, 1.0),
        cards_progress: frame.cards_progress.clamp(0.0, 1.0),
    }
}
