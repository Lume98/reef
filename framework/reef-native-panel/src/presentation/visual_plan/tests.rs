use reef::draw::primitive::{DrawPrimitive, PathSegment};

use super::{resolve_native_panel_draw_plan, NativePanelPaintInput, NativePanelVisualDisplayMode};
use crate::{
    presentation::descriptors::NativePanelHostWindowState,
    scene::SceneMascotPose,
    state::{ExpandedSurface, PanelRect},
};

fn rect(x: f64, y: f64, width: f64, height: f64) -> PanelRect {
    PanelRect {
        x,
        y,
        width,
        height,
    }
}

fn input(display_mode: NativePanelVisualDisplayMode) -> NativePanelPaintInput {
    NativePanelPaintInput {
        window_state: NativePanelHostWindowState {
            frame: Some(rect(0.0, 0.0, 320.0, 160.0)),
            visible: display_mode != NativePanelVisualDisplayMode::Hidden,
            preferred_display_index: 0,
        },
        display_mode,
        surface: ExpandedSurface::Default,
        panel_frame: rect(0.0, 0.0, 320.0, 160.0),
        compact_bar_frame: rect(20.0, 120.0, 240.0, 44.0),
        left_shoulder_frame: rect(14.0, 132.0, 6.0, 10.0),
        right_shoulder_frame: rect(260.0, 132.0, 6.0, 10.0),
        shoulder_progress: 0.0,
        content_frame: rect(0.0, 0.0, 320.0, 160.0),
        card_stack_frame: rect(20.0, 20.0, 280.0, 80.0),
        card_stack_content_height: 0.0,
        shell_frame: rect(0.0, 0.0, 320.0, 160.0),
        headline_text: "Reef UI".to_string(),
        headline_emphasized: false,
        active_count: "1".to_string(),
        active_count_elapsed_ms: 0,
        total_count: "2".to_string(),
        separator_visibility: 1.0,
        chrome_transition_progress: 1.0,
        cards_visible: false,
        card_count: 0,
        cards: Vec::new(),
        glow_visible: true,
        glow_opacity: 0.8,
        action_buttons_visible: false,
        action_buttons: Vec::new(),
        completion_count: 1,
        mascot_elapsed_ms: 0,
        mascot_motion_frame: None,
        mascot_pose: SceneMascotPose::Complete,
        mascot_debug_mode_enabled: false,
    }
}

#[test]
fn draw_plan_outputs_only_draw_plan_primitives() {
    let plan = resolve_native_panel_draw_plan(&input(NativePanelVisualDisplayMode::Compact));

    assert!(!plan.hidden);
    assert_eq!(plan.viewport.width, 320.0);
    assert!(plan.primitives.iter().any(|primitive| matches!(
        primitive,
        DrawPrimitive::Text { frame, text, .. } if text == "Reef UI" && frame.height > 0.0
    )));
    assert!(plan
        .primitives
        .iter()
        .any(|primitive| matches!(primitive, DrawPrimitive::NineSliceImage { .. })));
    assert!(plan
        .primitives
        .iter()
        .any(|primitive| matches!(primitive, DrawPrimitive::Path { .. })));
    assert!(plan.primitives.iter().any(|primitive| matches!(
        primitive,
        DrawPrimitive::StrokedRoundRect { .. }
            | DrawPrimitive::SpriteImage { .. }
            | DrawPrimitive::RoundRect { .. }
    )));
}

#[test]
fn hidden_input_returns_hidden_empty_draw_plan() {
    let plan = resolve_native_panel_draw_plan(&input(NativePanelVisualDisplayMode::Hidden));

    assert!(plan.hidden);
    assert!(plan.primitives.is_empty());
}

#[test]
fn compact_background_path_preserves_draw_coordinate_top_and_bottom() {
    let plan = resolve_native_panel_draw_plan(&input(NativePanelVisualDisplayMode::Compact));
    let compact_background = plan
        .primitives
        .iter()
        .find_map(|primitive| match primitive {
            DrawPrimitive::Path { segments, .. } if segments.len() == 6 => Some(segments),
            _ => None,
        })
        .expect("compact background path");

    assert!(matches!(
        compact_background[0],
        PathSegment::LineTo(point) if point.x == 20.0 && point.y == 164.0
    ));
    assert!(matches!(
        compact_background[4],
        PathSegment::LineTo(point) if point.x == 32.5 && point.y == 120.0
    ));
}
