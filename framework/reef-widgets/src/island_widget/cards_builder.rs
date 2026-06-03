use crate::{Badge, BodyLine, Card, CardStyle, SettingsRow};

use super::IslandWidgetContentInput;

const DEFAULT_SETTINGS_SUBTITLE: &str = "v0.1.0";

pub(super) fn build_cards(input: &IslandWidgetContentInput) -> Vec<Card> {
    if input.settings_active {
        return build_settings_cards();
    }

    let mut cards = Vec::new();

    for pending in &input.pending_permissions {
        cards.push(
            Card::new(CardStyle::PendingApproval)
                .title("Approval Required")
                .subtitle(format!("#{} · Approval", short_id(&pending.session_id)))
                .badge(Badge::status("Approval", true))
                .badge(Badge::source(&pending.source))
                .body_line(BodyLine::plain(
                    Some("!"),
                    pending
                        .tool_description
                        .clone()
                        .unwrap_or_else(|| "Waiting for your approval".to_string()),
                ))
                .action_hint("Allow / Deny in terminal")
                .height(80.0),
        );
    }

    for pending in &input.pending_questions {
        cards.push(
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
                .height(80.0),
        );
    }

    for session in &input.sessions {
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

        cards.push(card.height(100.0));
    }

    if cards.is_empty() {
        cards.push(
            Card::new(CardStyle::Empty)
                .title("No active sessions")
                .body_line(BodyLine::plain(
                    None,
                    "Reef UI is watching for new activity.",
                ))
                .height(60.0),
        );
    }

    cards
}

fn build_settings_cards() -> Vec<Card> {
    vec![Card::new(CardStyle::Settings)
        .title("Settings")
        .subtitle(DEFAULT_SETTINGS_SUBTITLE)
        .settings_rows(default_settings_rows())
        .height(230.0)]
}

fn default_settings_rows() -> Vec<SettingsRow> {
    vec![
        SettingsRow {
            title: "Display".into(),
            value: "1".into(),
            active: true,
        },
        SettingsRow {
            title: "Width".into(),
            value: "Auto".into(),
            active: false,
        },
        SettingsRow {
            title: "Language".into(),
            value: "En".into(),
            active: false,
        },
        SettingsRow {
            title: "Sound".into(),
            value: "On".into(),
            active: true,
        },
        SettingsRow {
            title: "Mascot".into(),
            value: "On".into(),
            active: true,
        },
        SettingsRow {
            title: "Updates".into(),
            value: "Check".into(),
            active: false,
        },
    ]
}

fn short_id(id: &str) -> String {
    id.chars().take(6).collect()
}
