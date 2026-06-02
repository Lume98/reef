use crate::primitive::VisualPlan;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FrameSubmission {
    pub hidden: bool,
    pub commands: Vec<VisualPlan>,
}

pub trait RenderBackend {
    type Error;

    fn submit_frame(&mut self, submission: &FrameSubmission) -> Result<(), Self::Error>;
}

pub fn submit_visual_plan(
    backend: &mut dyn RenderBackend<Error = ()>,
    plan: &VisualPlan,
) -> Result<(), ()> {
    let submission = FrameSubmission {
        hidden: plan.hidden,
        commands: vec![plan.clone()],
    };
    backend.submit_frame(&submission)
}
