//! Bridge from native panel paint input into `reef-widgets`.
//!
//! The legacy visual plan remains the reference renderer. This module gives the
//! component-library migration a concrete, testable candidate path.

use crate::native_panel_core::{
    PanelChromeVisibilitySpec, PanelPoint, PanelRect, EXPANDED_PANEL_RADIUS,
};
use crate::native_panel_scene::SceneMascotPose;
use crate::native_panel_ui::visual_plan::{
    resolve_compact_chrome_visibility, NativePanelPaintInput, NativePanelVisualCardBodyRole,
    NativePanelVisualCardInput, NativePanelVisualCardStyle, NativePanelVisualDisplayMode,
};
use reef_core::color::Color;
use reef_core::geometry::Rect;
use reef_draw::primitive::DrawPlan;
use reef_theme::{compact_bar as compact_theme, panel as panel_theme};
use reef_widgets::card::{Badge, BodyLine, BodyRole, Card, CardStyle, SettingsRow};
use reef_widgets::compact_bar::{
    ChromeVisibility, CompactBar, CompactShoulder, CompletionGlow, ShoulderSide,
};
use reef_widgets::island::ExpandedShell;
use reef_widgets::island_widget::{
    render_island_widget, DisplayMode, IslandWidget, IslandWidgetLayout,
};
use reef_widgets::mascot::{CompletionBadge, MascotPose, MascotWidget};

pub fn native_panel_island_widget_from_paint_input(input: &NativePanelPaintInput) -> IslandWidget {
    let mode = display_mode(input);
    let layout = IslandWidgetLayout::new(
        input.panel_frame.width.max(input.compact_bar_frame.width),
        input.compact_bar_frame.height,
        input.panel_frame.height.max(input.shell_frame.height),
    );
    let expanded_display_mode = input.display_mode == NativePanelVisualDisplayMode::Expanded;
    let chrome_spec = resolve_compact_chrome_visibility(input, expanded_display_mode);
    let chrome = chrome_visibility_from_spec(
        chrome_spec,
        input.shoulder_progress,
        input.separator_visibility,
    );

    let mut compact_bar = CompactBar::new()
        .headline(input.headline_text.clone())
        .headline_emphasized(input.headline_emphasized)
        .counts(input.active_count.clone(), input.total_count.clone())
        .show_actions(input.action_buttons_visible)
        .debug_mode(input.mascot_debug_mode_enabled)
        .chrome(chrome)
        .height(layout.compact_height);
    compact_bar.active_count_scroll = active_count_scroll(input.active_count_elapsed_ms);
    compact_bar.completion_count = input.completion_count;

    let mascot = mascot_from_input(input, chrome_spec);
    compact_bar.mascot = mascot.clone();

    let glow = input.glow_visible.then(|| CompletionGlow {
        frame: rect_from_panel(input.compact_bar_frame),
        base_opacity: input.glow_opacity,
        elapsed_ms: input.mascot_elapsed_ms,
    });

    compact_bar.glow = glow.clone();

    let shoulder_left = non_zero_panel_rect(input.left_shoulder_frame)
        .map(|frame| compact_shoulder(frame, ShoulderSide::Left, input.shoulder_progress));
    let shoulder_right = non_zero_panel_rect(input.right_shoulder_frame)
        .map(|frame| compact_shoulder(frame, ShoulderSide::Right, input.shoulder_progress));
    compact_bar.shoulder_left = shoulder_left.clone();
    compact_bar.shoulder_right = shoulder_right.clone();

    IslandWidget::new()
        .mode(mode)
        .layout(layout)
        .compact_bar(compact_bar)
        .expanded_shell(expanded_shell_from_input(input))
        .cards(cards_from_input(input))
        .chrome(chrome)
        .reveal(
            input.chrome_transition_progress.clamp(0.0, 1.0),
            input.cards_visible,
        )
        .maybe_mascot(mascot)
        .maybe_glow(glow)
        .maybe_shoulder_left(shoulder_left)
        .maybe_shoulder_right(shoulder_right)
}

pub fn resolve_native_panel_widget_draw_plan(input: &NativePanelPaintInput) -> DrawPlan {
    render_island_widget(&native_panel_island_widget_from_paint_input(input))
}

fn display_mode(input: &NativePanelPaintInput) -> DisplayMode {
    if !input.window_state.visible {
        return DisplayMode::Hidden;
    }
    match input.display_mode {
        NativePanelVisualDisplayMode::Hidden => DisplayMode::Hidden,
        NativePanelVisualDisplayMode::Compact => DisplayMode::Compact,
        NativePanelVisualDisplayMode::Expanded => DisplayMode::Expanded,
    }
}

fn chrome_visibility_from_spec(
    spec: PanelChromeVisibilitySpec,
    shoulder_progress: f64,
    separator_visibility: f64,
) -> ChromeVisibility {
    ChromeVisibility {
        separator_visibility: separator_visibility.clamp(0.0, 1.0),
        shoulder_progress: shoulder_progress.clamp(0.0, 1.0),
        collapsed_alpha: 1.0 - spec.collapsed_exit_progress.clamp(0.0, 1.0),
        action_button_visibility: spec.action_buttons.opacity,
    }
}

fn cards_from_input(input: &NativePanelPaintInput) -> Vec<Card> {
    if !input.cards_visible && input.cards.is_empty() {
        return Vec::new();
    }

    input.cards.iter().map(card_from_input).collect()
}

fn card_from_input(input: &NativePanelVisualCardInput) -> Card {
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

fn mascot_from_input(
    input: &NativePanelPaintInput,
    chrome: PanelChromeVisibilitySpec,
) -> Option<MascotWidget> {
    if input.mascot_pose == SceneMascotPose::Hidden || !chrome.collapsed_mascot_visible {
        return None;
    }

    let compact_frame = non_zero_panel_rect(input.compact_bar_frame).unwrap_or(input.panel_frame);
    let center = mascot_center(compact_frame);
    let mut mascot = MascotWidget::new(center.x, center.y, 11.0)
        .pose(mascot_pose(input.mascot_pose))
        .alpha(if chrome.collapsed_mascot_visible {
            1.0
        } else {
            0.0
        });
    mascot.elapsed_ms = input.mascot_elapsed_ms;

    if let Some(frame) = input.mascot_motion_frame {
        mascot.offset_x = frame.offset_x;
        mascot.offset_y = frame.offset_y;
        mascot.scale_x = frame.scale_x;
        mascot.scale_y = frame.scale_y;
        mascot.alpha = frame.shell_alpha;
        mascot.shadow_opacity = frame.shadow_opacity;
        mascot.shadow_radius = frame.shadow_radius;
    }

    if input.completion_count > 0 {
        mascot.completion_badge = Some(CompletionBadge::new(
            center.x,
            center.y - 14.0,
            input.completion_count,
        ));
    }

    Some(mascot)
}

fn mascot_pose(pose: SceneMascotPose) -> MascotPose {
    match pose {
        SceneMascotPose::Hidden => MascotPose::Hidden,
        SceneMascotPose::Idle => MascotPose::Idle,
        SceneMascotPose::Running => MascotPose::Running,
        SceneMascotPose::Approval => MascotPose::Approval,
        SceneMascotPose::Question => MascotPose::Question,
        SceneMascotPose::MessageBubble => MascotPose::MessageBubble,
        SceneMascotPose::Complete => MascotPose::Complete,
        SceneMascotPose::Sleepy => MascotPose::Sleepy,
        SceneMascotPose::WakeAngry => MascotPose::WakeAngry,
    }
}

fn mascot_center(compact_frame: PanelRect) -> PanelPoint {
    PanelPoint {
        x: compact_frame.x + 26.0,
        y: compact_frame.y + compact_frame.height / 2.0,
    }
}

fn expanded_shell_from_input(input: &NativePanelPaintInput) -> ExpandedShell {
    let mut shell = ExpandedShell::new().radius(EXPANDED_PANEL_RADIUS);
    shell.fill_color = Color::from(panel_theme::SHELL_FILL);
    shell.border_color = Color::from(panel_theme::SHELL_BORDER);
    shell.separator_color = Color::from(panel_theme::SHELL_SEPARATOR);
    if input.separator_visibility > 0.01 {
        shell.separator_y = Some(input.compact_bar_frame.height + 8.0);
    }
    shell
}

fn compact_shoulder(frame: PanelRect, side: ShoulderSide, progress: f64) -> CompactShoulder {
    CompactShoulder {
        frame: rect_from_panel(frame),
        side,
        progress: progress.clamp(0.0, 1.0),
        fill_color: Color::from(compact_theme::FILL),
        border_color: Color::from(compact_theme::BORDER),
    }
}

fn non_zero_panel_rect(rect: PanelRect) -> Option<PanelRect> {
    (rect.width > 0.0 && rect.height > 0.0).then_some(rect)
}

fn rect_from_panel(rect: PanelRect) -> Rect {
    Rect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}

fn active_count_scroll(elapsed_ms: u128) -> f64 {
    if elapsed_ms == 0 {
        0.0
    } else {
        ((elapsed_ms as f64) / 180.0).clamp(0.0, 1.0)
    }
}

trait IslandWidgetOptionExt {
    fn maybe_mascot(self, mascot: Option<MascotWidget>) -> Self;
    fn maybe_glow(self, glow: Option<CompletionGlow>) -> Self;
    fn maybe_shoulder_left(self, shoulder: Option<CompactShoulder>) -> Self;
    fn maybe_shoulder_right(self, shoulder: Option<CompactShoulder>) -> Self;
}

impl IslandWidgetOptionExt for IslandWidget {
    fn maybe_mascot(mut self, mascot: Option<MascotWidget>) -> Self {
        self.mascot = mascot;
        self
    }

    fn maybe_glow(mut self, glow: Option<CompletionGlow>) -> Self {
        self.glow = glow;
        self
    }

    fn maybe_shoulder_left(mut self, shoulder: Option<CompactShoulder>) -> Self {
        self.shoulder_left = shoulder;
        self
    }

    fn maybe_shoulder_right(mut self, shoulder: Option<CompactShoulder>) -> Self {
        self.shoulder_right = shoulder;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_panel_core::ExpandedSurface;
    use crate::native_panel_ui::render::NativePanelHostWindowState;

    fn input() -> NativePanelPaintInput {
        NativePanelPaintInput {
            window_state: NativePanelHostWindowState {
                visible: true,
                ..NativePanelHostWindowState::default()
            },
            display_mode: NativePanelVisualDisplayMode::Compact,
            surface: ExpandedSurface::Default,
            panel_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 253.0,
                height: 48.0,
            },
            compact_bar_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 253.0,
                height: 48.0,
            },
            left_shoulder_frame: PanelRect {
                x: -6.0,
                y: 12.0,
                width: 6.0,
                height: 24.0,
            },
            right_shoulder_frame: PanelRect {
                x: 253.0,
                y: 12.0,
                width: 6.0,
                height: 24.0,
            },
            shoulder_progress: 0.4,
            content_frame: PanelRect {
                x: 0.0,
                y: 48.0,
                width: 253.0,
                height: 120.0,
            },
            card_stack_frame: PanelRect {
                x: 0.0,
                y: 48.0,
                width: 253.0,
                height: 120.0,
            },
            card_stack_content_height: 120.0,
            shell_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 253.0,
                height: 180.0,
            },
            headline_text: "Sessions".into(),
            headline_emphasized: true,
            active_count: "2".into(),
            active_count_elapsed_ms: 0,
            total_count: "5".into(),
            separator_visibility: 1.0,
            chrome_transition_progress: 1.0,
            cards_visible: false,
            card_count: 0,
            cards: vec![],
            glow_visible: false,
            glow_opacity: 0.0,
            action_buttons_visible: false,
            action_buttons: vec![],
            completion_count: 0,
            mascot_elapsed_ms: 0,
            mascot_motion_frame: None,
            mascot_pose: SceneMascotPose::Hidden,
            mascot_debug_mode_enabled: false,
        }
    }

    #[test]
    fn hidden_input_outputs_hidden_draw_plan() {
        let mut input = input();
        input.display_mode = NativePanelVisualDisplayMode::Hidden;

        let plan = resolve_native_panel_widget_draw_plan(&input);

        assert!(plan.hidden);
        assert!(plan.primitives.is_empty());
    }

    #[test]
    fn compact_input_maps_layout_and_bar_fields() {
        let input = input();

        let widget = native_panel_island_widget_from_paint_input(&input);

        assert_eq!(widget.mode, DisplayMode::Compact);
        assert_eq!(widget.width, input.panel_frame.width);
        assert_eq!(widget.compact_height, input.compact_bar_frame.height);
        assert_eq!(widget.compact_bar.headline, input.headline_text);
        assert_eq!(widget.compact_bar.active_count, input.active_count);
        assert_eq!(widget.compact_bar.total_count, input.total_count);
        assert!(widget.shoulder_left.is_some());
        assert!(widget.shoulder_right.is_some());
    }

    #[test]
    fn expanded_single_card_maps_card_fields() {
        let mut input = input();
        input.display_mode = NativePanelVisualDisplayMode::Expanded;
        input.cards_visible = true;
        input.card_count = 1;
        input.cards = vec![NativePanelVisualCardInput {
            style: NativePanelVisualCardStyle::PendingApproval,
            title: "Allow command?".into(),
            subtitle: Some("codex".into()),
            body: Some("cargo test".into()),
            badge: Some(
                crate::native_panel_ui::render::NativePanelVisualCardBadgeInput {
                    text: "approval".into(),
                    emphasized: true,
                },
            ),
            source_badge: None,
            body_prefix: Some("$".into()),
            body_lines: vec![],
            action_hint: Some("Approve in terminal".into()),
            rows: vec![],
            height: 120.0,
            collapsed_height: 40.0,
            compact: false,
            removing: false,
        }];

        let widget = native_panel_island_widget_from_paint_input(&input);

        assert_eq!(widget.mode, DisplayMode::Expanded);
        assert_eq!(widget.cards.len(), 1);
        assert_eq!(widget.cards[0].style, CardStyle::PendingApproval);
        assert_eq!(widget.cards[0].title, "Allow command?");
        assert_eq!(widget.cards[0].badges.len(), 1);
        assert_eq!(widget.cards[0].body_lines.len(), 1);
    }

    #[test]
    fn glow_and_mascot_visibility_are_mapped() {
        let mut input = input();
        input.glow_visible = true;
        input.glow_opacity = 0.5;
        input.mascot_pose = SceneMascotPose::Idle;

        let compact = native_panel_island_widget_from_paint_input(&input);
        assert!(compact.glow.is_some());
        assert!(compact.mascot.is_some());

        input.mascot_pose = SceneMascotPose::Hidden;
        let hidden = native_panel_island_widget_from_paint_input(&input);
        assert!(hidden.mascot.is_none());
    }
}
