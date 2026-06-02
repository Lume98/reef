//! 桥接模块：将 PanelState + RuntimeSnapshot 转换为 reef-widgets 的 IslandWidget。
//!
//! 这是未来替代 reef-ui 场景→表现→视觉计划管线的入口点。
//! 当前作为概念验证，验证 Widget 框架可以承载生产级的灵动岛 UI。

use reef_widgets::{
    Badge, BodyLine, Card, CardStyle, ChromeVisibility, CompactBar, IslandWidget, MascotPose,
    MascotWidget, SettingsRow,
};

use echoisland_runtime::RuntimeSnapshot;

/// 将运行时快照转换为顶层 IslandWidget。
///
/// `panel_expanded` 控制展示模式（Compact / Expanded）。
/// `settings_active` 控制是否展示设置卡片。
pub fn build_island_widget(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> IslandWidget {
    let mode = if panel_expanded {
        reef_widgets::island_widget::DisplayMode::Expanded
    } else {
        reef_widgets::island_widget::DisplayMode::Compact
    };

    let compact_bar = build_compact_bar(snapshot, panel_expanded, settings_active);
    let cards = if settings_active {
        build_settings_cards()
    } else {
        build_status_cards(snapshot)
    };
    let mascot = build_mascot(snapshot, panel_expanded);

    IslandWidget {
        mode,
        compact_bar,
        cards,
        mascot,
        chrome: if panel_expanded {
            ChromeVisibility::expanded()
        } else {
            ChromeVisibility::compact()
        },
        reveal_progress: 1.0,
        entering: true,
        ..IslandWidget::new()
    }
}

fn build_compact_bar(
    snapshot: &RuntimeSnapshot,
    expanded: bool,
    settings_active: bool,
) -> CompactBar {
    let mut bar = CompactBar::new();
    bar.headline = "Reef".to_string();
    bar.headline_emphasized = expanded;
    bar.active_count = snapshot.active_session_count.to_string();
    bar.total_count = snapshot.total_session_count.to_string();
    bar.completion_count = 0;
    bar.show_actions = expanded || settings_active;
    bar.debug_mode = false;

    if expanded {
        bar.chrome = ChromeVisibility::expanded();
    } else {
        bar.chrome = ChromeVisibility::compact();
    }

    bar
}

fn build_status_cards(snapshot: &RuntimeSnapshot) -> Vec<Card> {
    let mut cards = Vec::new();

    // Pending approvals → PendingApproval cards
    for pending in &snapshot.pending_permissions {
        let card = Card::new(CardStyle::PendingApproval)
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
            .height(80.0);
        cards.push(card);
    }

    // Pending questions → PendingQuestion cards
    for pending in &snapshot.pending_questions {
        let card = Card::new(CardStyle::PendingQuestion)
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
            .height(80.0);
        cards.push(card);
    }

    // Active sessions → Default cards
    for session in &snapshot.sessions {
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

        card = card.height(100.0);
        cards.push(card);
    }

    // Empty placeholder
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
        .subtitle("v0.1.0")
        .settings_rows(vec![
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
        ])
        .height(230.0)]
}

fn build_mascot(snapshot: &RuntimeSnapshot, expanded: bool) -> Option<MascotWidget> {
    if !expanded {
        return None;
    }

    let pose = if snapshot.pending_permission_count > 0 {
        MascotPose::Approval
    } else if snapshot.pending_question_count > 0 {
        MascotPose::Question
    } else if snapshot.active_session_count > 0 {
        MascotPose::Running
    } else {
        MascotPose::Idle
    };

    let mut mascot = MascotWidget::new(200.0, 24.0, 14.0).pose(pose);

    // Completion badge when sessions exist
    if snapshot.total_session_count > 0 && snapshot.active_session_count == 0 {
        mascot.completion_badge = Some(reef_widgets::mascot_badge::CompletionBadge::new(
            200.0,
            10.0,
            snapshot.total_session_count,
        ));
    }

    Some(mascot)
}

fn short_id(id: &str) -> String {
    id.chars().take(6).collect()
}
