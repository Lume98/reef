//! Migration gate for the native panel `reef-widgets` candidate renderer.

use reef_draw::primitive::DrawPlan;

use super::visual_plan::{resolve_native_panel_visual_plan, NativePanelPaintInput};
use super::widget_bridge::resolve_native_panel_widget_draw_plan;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveDiff {
    pub index: usize,
    pub expected: String,
    pub actual: String,
    pub field: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawPlanComparison {
    pub total_primitives_expected: usize,
    pub total_primitives_actual: usize,
    pub diffs: Vec<PrimitiveDiff>,
}

impl DrawPlanComparison {
    pub fn is_identical(&self) -> bool {
        self.diffs.is_empty() && self.total_primitives_expected == self.total_primitives_actual
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativePanelWidgetMigrationResult {
    pub reference: DrawPlan,
    pub candidate: DrawPlan,
    pub comparison: DrawPlanComparison,
    pub selected: DrawPlan,
    pub used_candidate: bool,
}

pub fn compare_draw_plans(expected: &DrawPlan, actual: &DrawPlan) -> DrawPlanComparison {
    let mut diffs = Vec::new();

    let max_len = expected.primitives.len().max(actual.primitives.len());
    for index in 0..max_len {
        match (expected.primitives.get(index), actual.primitives.get(index)) {
            (Some(expected), Some(actual)) => {
                if expected != actual {
                    diffs.push(PrimitiveDiff {
                        index,
                        expected: format!("{:?}", expected),
                        actual: format!("{:?}", actual),
                        field: "primitive".into(),
                    });
                }
            }
            (Some(expected), None) => diffs.push(PrimitiveDiff {
                index,
                expected: format!("{:?}", expected),
                actual: "<missing>".into(),
                field: "existence".into(),
            }),
            (None, Some(actual)) => diffs.push(PrimitiveDiff {
                index,
                expected: "<missing>".into(),
                actual: format!("{:?}", actual),
                field: "existence".into(),
            }),
            (None, None) => {}
        }
    }

    DrawPlanComparison {
        total_primitives_expected: expected.primitives.len(),
        total_primitives_actual: actual.primitives.len(),
        diffs,
    }
}

pub fn resolve_native_panel_widget_migration_result(
    input: &NativePanelPaintInput,
) -> NativePanelWidgetMigrationResult {
    let reference = resolve_native_panel_visual_plan(input);
    let candidate = resolve_native_panel_widget_draw_plan(input);
    let comparison = compare_draw_plans(&reference, &candidate);
    let used_candidate = comparison.is_identical();
    let selected = if used_candidate {
        candidate.clone()
    } else {
        reference.clone()
    };

    NativePanelWidgetMigrationResult {
        reference,
        candidate,
        comparison,
        selected,
        used_candidate,
    }
}

pub fn resolve_native_panel_widget_migration_draw_plan(input: &NativePanelPaintInput) -> DrawPlan {
    let result = resolve_native_panel_widget_migration_result(input);
    if !result.used_candidate {
        log::warn!(
            "[NativePanelWidgetMigration] {} primitive 差异，回退到旧 visual plan",
            result.comparison.diffs.len()
        );
    }
    result.selected
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_panel_core::{ExpandedSurface, PanelRect};
    use crate::native_panel_scene::SceneMascotPose;
    use crate::native_panel_ui::render::{
        NativePanelHostWindowState, NativePanelVisualCardBadgeInput, NativePanelVisualCardInput,
        NativePanelVisualCardStyle, NativePanelVisualDisplayMode,
    };

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
                width: 200.0,
                height: 48.0,
            },
            compact_bar_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 48.0,
            },
            left_shoulder_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            right_shoulder_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            shoulder_progress: 0.0,
            content_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            card_stack_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            card_stack_content_height: 0.0,
            shell_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 48.0,
            },
            headline_text: "Sessions".into(),
            headline_emphasized: false,
            active_count: "3".into(),
            active_count_elapsed_ms: 0,
            total_count: "10".into(),
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
    fn hidden_input_uses_identical_candidate() {
        let mut input = input();
        input.display_mode = NativePanelVisualDisplayMode::Hidden;
        input.window_state.visible = false;

        let result = resolve_native_panel_widget_migration_result(&input);

        assert!(result.comparison.is_identical());
        assert!(result.used_candidate);
        assert_eq!(result.selected, result.candidate);
    }

    #[test]
    fn compact_visible_input_falls_back_to_reference_when_candidate_differs() {
        let input = input();

        let result = resolve_native_panel_widget_migration_result(&input);

        assert!(!result.comparison.is_identical());
        assert!(!result.used_candidate);
        assert_eq!(result.selected, result.reference);
    }

    #[test]
    fn expanded_card_input_falls_back_to_reference_when_candidate_differs() {
        let mut input = input();
        input.display_mode = NativePanelVisualDisplayMode::Expanded;
        input.cards_visible = true;
        input.card_count = 1;
        input.cards = vec![NativePanelVisualCardInput {
            style: NativePanelVisualCardStyle::Default,
            title: "Test Card".into(),
            subtitle: Some("sub".into()),
            body: None,
            badge: Some(NativePanelVisualCardBadgeInput {
                text: "new".into(),
                emphasized: false,
            }),
            source_badge: None,
            body_prefix: None,
            body_lines: vec![],
            action_hint: Some("click".into()),
            rows: vec![],
            height: 120.0,
            collapsed_height: 40.0,
            compact: false,
            removing: false,
        }];
        input.compact_bar_frame = PanelRect {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 48.0,
        };
        input.shell_frame = PanelRect {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 300.0,
        };
        input.card_stack_frame = PanelRect {
            x: 0.0,
            y: 48.0,
            width: 300.0,
            height: 200.0,
        };
        input.card_stack_content_height = 200.0;
        input.content_frame = PanelRect {
            x: 0.0,
            y: 48.0,
            width: 300.0,
            height: 200.0,
        };
        input.panel_frame = PanelRect {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 300.0,
        };
        input.separator_visibility = 1.0;

        let result = resolve_native_panel_widget_migration_result(&input);

        assert!(!result.comparison.is_identical());
        assert!(!result.used_candidate);
        assert_eq!(result.selected, result.reference);
    }

    #[test]
    fn compare_draw_plans_reports_count_and_content_differences() {
        let input = input();
        let mut expected = resolve_native_panel_visual_plan(&input);
        let mut actual = expected.clone();
        actual.primitives.pop();

        let count_comparison = compare_draw_plans(&expected, &actual);
        assert_eq!(
            count_comparison.total_primitives_expected,
            expected.primitives.len()
        );
        assert_eq!(
            count_comparison.total_primitives_actual,
            actual.primitives.len()
        );
        assert!(count_comparison
            .diffs
            .iter()
            .any(|diff| diff.field == "existence"));

        actual = expected.clone();
        expected.primitives.swap(0, 1);
        let content_comparison = compare_draw_plans(&expected, &actual);
        assert!(content_comparison
            .diffs
            .iter()
            .any(|diff| diff.field == "primitive"));
    }
}
