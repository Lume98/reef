use super::visual_primitives::{
    NativePanelDrawPlan, NativePanelDrawPrimitive, NativePanelVisualTextAlignment,
    NativePanelVisualTextRole, NativePanelVisualTextWeight,
};
use crate::native_panel_core::{PanelPoint, PanelRect};

#[derive(Clone, Debug, PartialEq)]
pub enum NativePanelRenderCommand {
    ClipStart {
        frame: PanelRect,
    },
    ClipEnd,
    CompletionGlow {
        frame: PanelRect,
        opacity: f64,
    },
    RoundRect {
        frame: PanelRect,
        radius: f64,
        color: super::visual_primitives::NativePanelVisualColor,
    },
    Rect {
        frame: PanelRect,
        color: super::visual_primitives::NativePanelVisualColor,
    },
    Ellipse {
        frame: PanelRect,
        color: super::visual_primitives::NativePanelVisualColor,
    },
    StrokeLine {
        from: PanelPoint,
        to: PanelPoint,
        color: super::visual_primitives::NativePanelVisualColor,
        width: i32,
    },
    Text {
        role: NativePanelVisualTextRole,
        origin: PanelPoint,
        max_width: f64,
        text: String,
        color: super::visual_primitives::NativePanelVisualColor,
        size: i32,
        weight: NativePanelVisualTextWeight,
        alignment: NativePanelVisualTextAlignment,
        alpha: f64,
    },
    Primitive(NativePanelDrawPrimitive),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NativePanelDrawFrameSubmission {
    pub hidden: bool,
    pub commands: Vec<NativePanelRenderCommand>,
}

impl From<NativePanelDrawPrimitive> for NativePanelRenderCommand {
    fn from(value: NativePanelDrawPrimitive) -> Self {
        match value {
            NativePanelDrawPrimitive::ClipStart { frame } => Self::ClipStart { frame },
            NativePanelDrawPrimitive::ClipEnd => Self::ClipEnd,
            NativePanelDrawPrimitive::CompletionGlow { frame, opacity } => {
                Self::CompletionGlow { frame, opacity }
            }
            NativePanelDrawPrimitive::RoundRect {
                frame,
                radius,
                color,
            } => Self::RoundRect {
                frame,
                radius,
                color,
            },
            NativePanelDrawPrimitive::Rect { frame, color } => Self::Rect { frame, color },
            NativePanelDrawPrimitive::Ellipse { frame, color } => Self::Ellipse { frame, color },
            NativePanelDrawPrimitive::StrokeLine {
                from,
                to,
                color,
                width,
            } => Self::StrokeLine {
                from,
                to,
                color,
                width,
            },
            NativePanelDrawPrimitive::Text {
                role,
                origin,
                max_width,
                text,
                color,
                size,
                weight,
                alignment,
                alpha,
            } => Self::Text {
                role,
                origin,
                max_width,
                text,
                color,
                size,
                weight,
                alignment,
                alpha,
            },
            other => Self::Primitive(other),
        }
    }
}

pub trait NativePanelDrawBackend {
    type Error;

    fn submit_frame(
        &mut self,
        submission: &NativePanelDrawFrameSubmission,
    ) -> Result<(), Self::Error>;
}

pub fn native_panel_frame_submission_from_visual_plan(
    plan: &NativePanelDrawPlan,
) -> NativePanelDrawFrameSubmission {
    NativePanelDrawFrameSubmission {
        hidden: plan.hidden,
        commands: plan
            .primitives
            .iter()
            .cloned()
            .map(NativePanelRenderCommand::from)
            .collect(),
    }
}

pub fn native_panel_submit_visual_plan<B>(
    backend: &mut B,
    plan: &NativePanelDrawPlan,
) -> Result<(), B::Error>
where
    B: NativePanelDrawBackend,
{
    let submission = native_panel_frame_submission_from_visual_plan(plan);
    backend.submit_frame(&submission)
}
