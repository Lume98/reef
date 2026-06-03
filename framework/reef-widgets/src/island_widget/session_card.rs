use crate::{Badge, BodyLine, Card, CardStyle};

use super::IslandSessionInput;

pub(super) fn build_session_card(session: &IslandSessionInput) -> Card {
    let title = if session.status.is_empty() {
        "Session"
    } else {
        &session.status
    };
    let subtitle = format!(
        "{} · {}",
        session.source,
        if session.model.as_deref().unwrap_or("") == "claude" {
            "Claude"
        } else {
            &session.source
        }
    );
    let mut card = Card::new(CardStyle::Default)
        .title(title.to_string())
        .subtitle(subtitle)
        .badge(Badge::status(&session.status, true))
        .badge(Badge::source(&session.source));

    if let Some(prompt) = &session.last_user_prompt {
        card = card.body_line(BodyLine::plain(Some(">"), prompt.clone()));
    }
    if let Some(reply) = &session.last_assistant_message {
        card = card.body_line(BodyLine::plain(Some("$"), reply.clone()));
    }
    if let Some(tool) = &session.current_tool {
        card = card.tool(tool.clone(), session.tool_description.clone());
    }

    card.height(100.0)
}
