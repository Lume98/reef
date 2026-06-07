use crate::native_panel_ui::visual_plan::{
    NativePanelPaintInput, NativePanelVisualCardBodyRole, NativePanelVisualCardInput,
    NativePanelVisualCardStyle,
};
use reef_widgets::card::{Badge, BodyLine, BodyRole, Card, CardStyle, SettingsRow};

pub(super) fn cards(input: &NativePanelPaintInput) -> Vec<Card> {
    if !input.cards_visible && input.cards.is_empty() {
        return Vec::new();
    }

    input.cards.iter().map(card).collect()
}

fn card(input: &NativePanelVisualCardInput) -> Card {
    let mut card = Card::new(card_style(input.style))
        .title(input.title.clone())
        .height(input.height);

    card.subtitle = input.subtitle.clone();
    card.collapsed_height = input.collapsed_height;
    card.compact = input.compact;
    card.reveal_phase = if input.removing { 0.0 } else { 1.0 };

    if let Some(badge) = &input.badge {
        card.badges
            .push(Badge::status(badge.text.clone(), badge.emphasized));
    }
    if let Some(badge) = &input.source_badge {
        card.badges.push(Badge::source(badge.text.clone()));
    }
    if let Some(body) = &input.body {
        card.body_lines.push(BodyLine {
            role: BodyRole::Plain,
            prefix: input.body_prefix.clone(),
            text: body.clone(),
            max_lines: 2,
        });
    }
    card.body_lines
        .extend(input.body_lines.iter().map(|line| BodyLine {
            role: body_role(line.role),
            prefix: line.prefix.clone(),
            text: line.text.clone(),
            max_lines: line.max_lines,
        }));
    card.action_hint = input.action_hint.clone();
    card.settings_rows = input
        .rows
        .iter()
        .map(|row| SettingsRow {
            title: row.title.clone(),
            value: row.value.clone(),
            active: row.active,
        })
        .collect();

    card
}

fn card_style(style: NativePanelVisualCardStyle) -> CardStyle {
    match style {
        NativePanelVisualCardStyle::Default => CardStyle::Default,
        NativePanelVisualCardStyle::Pending => CardStyle::Pending,
        NativePanelVisualCardStyle::PendingApproval => CardStyle::PendingApproval,
        NativePanelVisualCardStyle::PendingQuestion => CardStyle::PendingQuestion,
        NativePanelVisualCardStyle::PromptAssist => CardStyle::PromptAssist,
        NativePanelVisualCardStyle::Completion => CardStyle::Completion,
        NativePanelVisualCardStyle::Settings => CardStyle::Settings,
        NativePanelVisualCardStyle::Empty => CardStyle::Empty,
    }
}

fn body_role(role: NativePanelVisualCardBodyRole) -> BodyRole {
    match role {
        NativePanelVisualCardBodyRole::Assistant => BodyRole::Assistant,
        NativePanelVisualCardBodyRole::User => BodyRole::User,
        NativePanelVisualCardBodyRole::Tool => BodyRole::Tool,
        NativePanelVisualCardBodyRole::Plain => BodyRole::Plain,
        NativePanelVisualCardBodyRole::ActionHint => BodyRole::ActionHint,
    }
}
