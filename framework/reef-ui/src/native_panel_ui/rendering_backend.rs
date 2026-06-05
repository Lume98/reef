pub use reef_draw::draw_backend::{DrawBackend, FrameSubmission};

pub fn native_panel_frame_submission_from_visual_plan(
    plan: &reef_draw::primitive::DrawPlan,
) -> FrameSubmission {
    FrameSubmission {
        hidden: plan.hidden,
        plans: vec![plan.clone()],
    }
}

pub fn native_panel_submit_visual_plan<B>(
    backend: &mut B,
    plan: &reef_draw::primitive::DrawPlan,
) -> Result<(), B::Error>
where
    B: DrawBackend,
{
    let submission = native_panel_frame_submission_from_visual_plan(plan);
    backend.submit_frame(&submission)
}
