use crate::{Badge, BodyLine, Card, CardStyle};

use super::{short_id::short_id, IslandPendingQuestionInput};

pub(super) fn build_pending_question_card(pending: &IslandPendingQuestionInput) -> Card {
    Card::new(CardStyle::PendingQuestion)
        .title(
            pending
                .header
                .clone()
                .unwrap_or_else(|| "Question".to_string()),
        )
        .subtitle(format!("#{} · Question", short_id(&pending.session_id)))
        .badge(Badge::status("Question", true))
        .badge(Badge::source(&pending.source))
        .body_line(BodyLine::plain(Some("?"), pending.text.clone()))
        .action_hint("Answer in terminal")
        .height(80.0)
}
