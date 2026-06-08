use super::{
    directwrite_text_requests_from_paint_plan, Direct2DWindowsPanelPainter,
    PlanOnlyWindowsPanelPainter, WindowsDirect2DCoordinateSpace, WindowsPanelPainter,
};
use crate::{
    platform_windows_host::window_shell::WindowsPanelShellPaintJob,
    state::{ExpandedSurface, PanelRect},
};
use reef::core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef::draw::primitive::{DrawPlan, DrawPrimitive, PathSegment, TextAlignment, TextWeight};

fn paint_job() -> WindowsPanelShellPaintJob {
    let zero = PanelRect {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };
    WindowsPanelShellPaintJob {
        window_state: crate::presentation::descriptor::NativePanelHostWindowState {
            frame: Some(PanelRect {
                x: 0.0,
                y: 0.0,
                width: 320.0,
                height: 160.0,
            }),
            visible: true,
            preferred_display_index: 0,
        },
        display_mode: crate::presentation::presentation::NativePanelVisualDisplayMode::Compact,
        surface: ExpandedSurface::Default,
        panel_frame: PanelRect {
            x: 0.0,
            y: 0.0,
            width: 320.0,
            height: 160.0,
        },
        compact_bar_frame: PanelRect {
            x: 20.0,
            y: 120.0,
            width: 240.0,
            height: 44.0,
        },
        card_stack_frame: PanelRect {
            x: 20.0,
            y: 20.0,
            width: 280.0,
            height: 80.0,
        },
        card_stack_content_height: 0.0,
        shell_frame: PanelRect {
            x: 0.0,
            y: 0.0,
            width: 320.0,
            height: 160.0,
        },
        content_frame: PanelRect {
            x: 0.0,
            y: 0.0,
            width: 320.0,
            height: 160.0,
        },
        left_shoulder_frame: zero,
        right_shoulder_frame: zero,
        shoulder_progress: 1.0,
        headline_text: "Reef UI".to_string(),
        headline_emphasized: false,
        active_count: String::new(),
        active_count_elapsed_ms: 0,
        total_count: String::new(),
        separator_visibility: 0.0,
        chrome_transition_progress: 0.0,
        cards_visible: false,
        card_count: 0,
        cards: Vec::new(),
        glow_visible: false,
        glow_opacity: 0.0,
        action_buttons_visible: false,
        action_buttons: Vec::new(),
        completion_count: 0,
        mascot_elapsed_ms: 0,
        mascot_motion_frame: None,
        mascot_pose: crate::scene::SceneMascotPose::Idle,
        mascot_debug_mode_enabled: false,
    }
}

#[test]
fn plan_only_painter_returns_draw_plan() {
    let mut painter = PlanOnlyWindowsPanelPainter;
    let plan = painter.paint(&paint_job()).expect("paint plan");
    assert!(!plan.hidden);
    assert!(plan
        .primitives
        .iter()
        .any(|primitive| matches!(primitive, DrawPrimitive::Text { .. })));
}

#[test]
fn directwrite_requests_use_text_frame_width_and_draw_plan_text_style() {
    let plan = DrawPlan {
        hidden: false,
        viewport: Size {
            width: 100.0,
            height: 50.0,
        },
        primitives: vec![DrawPrimitive::Text {
            frame: Rect {
                x: 1.0,
                y: 2.0,
                width: 42.0,
                height: 18.0,
            },
            text: "Codex".to_string(),
            color: Color::WHITE,
            size: 13,
            weight: TextWeight::Semibold,
            alignment: TextAlignment::Center,
            alpha: 1.0,
        }],
    };

    let requests = directwrite_text_requests_from_paint_plan(&plan);

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].max_width, 42.0);
    assert_eq!(requests[0].weight, TextWeight::Semibold);
    assert_eq!(requests[0].alignment, TextAlignment::Center);
}

#[test]
fn coordinate_space_converts_rect_point_and_path_points_once() {
    let coordinates = WindowsDirect2DCoordinateSpace::new(100.0);
    assert_eq!(
        coordinates
            .rect(Rect {
                x: 1.0,
                y: 2.0,
                width: 3.0,
                height: 4.0
            })
            .y,
        94.0
    );
    assert_eq!(coordinates.point(Point { x: 1.0, y: 2.0 }).y, 98.0);
    let path = [PathSegment::LineTo(Point { x: 0.0, y: 10.0 })];
    if let PathSegment::LineTo(point) = path[0] {
        assert_eq!(coordinates.point(point).y, 90.0);
    }
}

#[test]
fn direct2d_painter_constructs_without_window() {
    let painter = Direct2DWindowsPanelPainter::new(None).expect("painter");
    assert!(!painter.is_per_pixel_alpha_ready() || painter.resource_rebuild_count() == 0);
}
