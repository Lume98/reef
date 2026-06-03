use super::IslandWidgetContentInput;
use crate::{
    Badge, BodyLine, Card, CardStyle, ChromeVisibility, CompactBar, CompletionBadge, DisplayMode,
    IslandRevealSpec, IslandWidget, IslandWidgetSpec, MascotPose, MascotWidget, SettingsRow,
};

const DEFAULT_HEADLINE: &str = "Reef";
const DEFAULT_SETTINGS_SUBTITLE: &str = "v0.1.0";

pub fn build_island_widget_spec(input: &IslandWidgetContentInput) -> IslandWidgetSpec {
    let mut compact_bar = build_compact_bar(input);
    compact_bar.chrome = if input.mode == DisplayMode::Expanded {
        ChromeVisibility::expanded()
    } else {
        ChromeVisibility::compact()
    };

    IslandWidgetSpec {
        mode: input.mode,
        layout: input.layout,
        compact_bar,
        expanded_shell: crate::island::ExpandedShell::new(),
        cards: build_cards(input),
        mascot: build_mascot(input),
        glow: None,
        shoulder_left: None,
        shoulder_right: None,
        chrome: if input.mode == DisplayMode::Expanded {
            ChromeVisibility::expanded()
        } else {
            ChromeVisibility::compact()
        },
        reveal: IslandRevealSpec::default(),
    }
}

pub fn build_island_widget(input: &IslandWidgetContentInput) -> IslandWidget {
    IslandWidget::from_spec(build_island_widget_spec(input))
}

fn build_compact_bar(input: &IslandWidgetContentInput) -> CompactBar {
    let mut bar = CompactBar::new();
    bar.headline = DEFAULT_HEADLINE.to_string();
    bar.headline_emphasized = input.mode == DisplayMode::Expanded;
    bar.active_count = input.active_session_count.to_string();
    bar.total_count = input.total_session_count.to_string();
    bar.completion_count = 0;
    bar.show_actions = input.mode == DisplayMode::Expanded || input.settings_active;
    bar.debug_mode = false;
    bar
}

fn build_cards(input: &IslandWidgetContentInput) -> Vec<Card> {
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

fn build_mascot(input: &IslandWidgetContentInput) -> Option<MascotWidget> {
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

fn short_id(id: &str) -> String {
    id.chars().take(6).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IslandSessionInput, IslandWidgetLayout};

    fn empty_input() -> IslandWidgetContentInput {
        IslandWidgetContentInput::default()
    }

    #[test]
    fn island_widget_spec_maps_expanded_settings_state() {
        let mut input = empty_input();
        input.mode = DisplayMode::Expanded;
        input.settings_active = true;

        let spec = build_island_widget_spec(&input);
        assert_eq!(spec.mode, DisplayMode::Expanded);
        assert_eq!(spec.layout, IslandWidgetLayout::default());
        assert_eq!(spec.chrome, ChromeVisibility::expanded());
        assert!(spec.compact_bar.show_actions);
        assert_eq!(spec.cards.len(), 1);
        assert!(spec.mascot.is_some());
        assert!(spec.glow.is_none());
    }

    #[test]
    fn island_widget_spec_maps_status_state() {
        let mut input = empty_input();
        input.mode = DisplayMode::Compact;
        input.active_session_count = 2;
        input.total_session_count = 5;
        input.sessions = vec![IslandSessionInput {
            status: "Running".into(),
            source: "Claude".into(),
            model: Some("claude".into()),
            last_user_prompt: Some("ping".into()),
            last_assistant_message: Some("pong".into()),
            current_tool: Some("bash".into()),
            tool_description: Some("run command".into()),
        }];

        let spec = build_island_widget_spec(&input);
        assert_eq!(spec.cards.len(), 1);
        assert_eq!(spec.compact_bar.headline, DEFAULT_HEADLINE);
        assert_eq!(spec.compact_bar.active_count, "2");
        assert_eq!(spec.compact_bar.total_count, "5");
        assert!(spec.mascot.is_none());
    }
}
