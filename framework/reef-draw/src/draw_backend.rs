use crate::primitive::DrawPlan;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FrameSubmission {
    pub hidden: bool,
    pub plans: Vec<DrawPlan>,
}

pub trait DrawBackend {
    type Error;

    fn submit_frame(&mut self, submission: &FrameSubmission) -> Result<(), Self::Error>;
}

pub fn submit_visual_plan(
    backend: &mut dyn DrawBackend<Error = ()>,
    plan: &DrawPlan,
) -> Result<(), ()> {
    let submission = FrameSubmission {
        hidden: plan.hidden,
        plans: vec![plan.clone()],
    };
    backend.submit_frame(&submission)
}
