use crate::{CompletionBadge, MascotPose, MascotWidget};

use super::{DisplayMode, IslandWidgetContentInput};

pub(super) fn build_mascot(input: &IslandWidgetContentInput) -> Option<MascotWidget> {
    if input.mode != DisplayMode::Expanded {
        return None;
    }

    let pose = if !input.pending_permissions.is_empty() {
        MascotPose::Approval
    } else if !input.pending_questions.is_empty() {
        MascotPose::Question
    } else if input.active_session_count > 0 {
        MascotPose::Running
    } else {
        MascotPose::Idle
    };

    let mut mascot = MascotWidget::new(200.0, 24.0, 14.0).pose(pose);

    if input.total_session_count > 0 && input.active_session_count == 0 {
        mascot.completion_badge =
            Some(CompletionBadge::new(200.0, 10.0, input.total_session_count));
    }

    Some(mascot)
}
