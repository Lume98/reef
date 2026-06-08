//! Bridge from native panel paint input into `reef::widgets`.
//!
//! The legacy visual plan remains the reference renderer. This module maps the
//! native panel paint input into the candidate widget tree only.

mod adornments;
mod cards;
mod chrome;

use reef::draw::primitive::DrawPlan;
use reef::widgets::island_widget::{
    render_island_widget, IslandRevealSpec, IslandWidget, IslandWidgetSpec,
};

use crate::presentation::visual_plan::NativePanelPaintInput;

pub fn native_panel_island_widget_from_paint_input(input: &NativePanelPaintInput) -> IslandWidget {
    IslandWidget::from_spec(island_widget_spec_from_paint_input(input))
}

pub fn resolve_native_panel_widget_draw_plan(input: &NativePanelPaintInput) -> DrawPlan {
    render_island_widget(&native_panel_island_widget_from_paint_input(input))
}

fn island_widget_spec_from_paint_input(input: &NativePanelPaintInput) -> IslandWidgetSpec {
    let mode = chrome::display_mode(input);
    let layout = chrome::layout(input);
    let chrome_spec = chrome::chrome_spec(input);
    let chrome = chrome::chrome_visibility(input, chrome_spec);
    let mascot = adornments::mascot(input, chrome_spec);
    let glow = adornments::glow(input);
    let shoulder_left = adornments::shoulder_left(input);
    let shoulder_right = adornments::shoulder_right(input);

    IslandWidgetSpec {
        mode,
        layout,
        compact_bar: chrome::compact_bar(
            input,
            layout,
            chrome,
            mascot.clone(),
            glow.clone(),
            shoulder_left.clone(),
            shoulder_right.clone(),
        ),
        expanded_shell: chrome::expanded_shell(input),
        cards: cards::cards(input),
        mascot,
        glow,
        shoulder_left,
        shoulder_right,
        chrome,
        reveal: IslandRevealSpec::new(
            input.chrome_transition_progress.clamp(0.0, 1.0),
            input.cards_visible,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::render::{
        NativePanelHostWindowState, NativePanelVisualCardBadgeInput, NativePanelVisualCardInput,
        NativePanelVisualCardStyle, NativePanelVisualDisplayMode,
    };
    use crate::scene::SceneMascotPose;
    use crate::state::{ExpandedSurface, PanelRect};
    use reef::widgets::card::CardStyle;
    use reef::widgets::island_widget::DisplayMode;

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
            badge: Some(NativePanelVisualCardBadgeInput {
                text: "approval".into(),
                emphasized: true,
            }),
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
