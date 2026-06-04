use crate::prelude::{Badge, BodyLine, Card, CardStyle};

use super::IslandSessionInput;

fn compact_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_string)
}

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

    if let Some(tool) = &session.current_tool {
        card = card.tool(tool.clone(), session.tool_description.clone());
    }
    if let Some(reply) = compact_text(
        session
            .last_assistant_message
            .as_deref()
            .or(session.tool_description.as_deref()),
    ) {
        card = card.body_line(BodyLine::plain(Some("$"), reply));
    }
    if let Some(prompt) = compact_text(session.last_user_prompt.as_deref()) {
        card = card.body_line(BodyLine::plain(Some(">"), prompt));
    }

    card.height(100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::BodyRole;

    fn session() -> IslandSessionInput {
        IslandSessionInput {
            status: "Running".into(),
            source: "Claude".into(),
            model: Some("claude".into()),
            last_user_prompt: None,
            last_assistant_message: None,
            current_tool: None,
            tool_description: None,
        }
    }

    #[test]
    fn build_session_card_keeps_assistant_body_visible_when_reply_is_whitespace_free() {
        let mut session = session();
        session.last_user_prompt = Some("ping".into());
        session.last_assistant_message = Some("pong".into());

        let card = build_session_card(&session);

        assert_eq!(card.body_lines.len(), 2);
        assert_eq!(card.body_lines[0].role, BodyRole::Plain);
        assert_eq!(card.body_lines[0].prefix.as_deref(), Some("$"));
        assert_eq!(card.body_lines[0].text, "pong");
        assert_eq!(card.body_lines[1].prefix.as_deref(), Some(">"));
        assert_eq!(card.body_lines[1].text, "ping");
    }

    #[test]
    fn build_session_card_falls_back_to_tool_description_for_assistant_line() {
        let mut session = session();
        session.last_user_prompt = Some("ping".into());
        session.current_tool = Some("bash".into());
        session.tool_description = Some("run command".into());

        let card = build_session_card(&session);

        assert_eq!(
            card.tool.as_ref().map(|tool| tool.name.as_str()),
            Some("bash")
        );
        assert_eq!(card.body_lines.len(), 2);
        assert_eq!(card.body_lines[0].prefix.as_deref(), Some("$"));
        assert_eq!(card.body_lines[0].text, "run command");
        assert_eq!(card.body_lines[1].prefix.as_deref(), Some(">"));
        assert_eq!(card.body_lines[1].text, "ping");
    }
}
