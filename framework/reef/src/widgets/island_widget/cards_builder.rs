use crate::prelude::Card;

use super::{
    approval_card::build_pending_approval_card, empty_card::build_empty_sessions_card,
    question_card::build_pending_question_card, session_card::build_session_card,
    settings_cards::build_settings_cards, IslandWidgetContentInput,
};

pub fn build_cards_from_input(input: &IslandWidgetContentInput) -> Vec<Card> {
    if input.settings_active {
        return build_settings_cards();
    }

    let mut cards = Vec::new();

    for pending in &input.pending_permissions {
        cards.push(build_pending_approval_card(pending));
    }

    for pending in &input.pending_questions {
        cards.push(build_pending_question_card(pending));
    }

    for session in &input.sessions {
        cards.push(build_session_card(session));
    }

    if cards.is_empty() {
        cards.push(build_empty_sessions_card());
    }

    cards
}
