use crate::native_panel_core::{
    resolve_active_count_marquee_frame, resolve_compact_action_button_layout,
    resolve_compact_bar_content_layout, ActiveCountMarqueeInput, CompactBarContentLayout,
    CompactBarContentLayoutInput, PanelChromeVisibilitySpec, PanelPoint, PanelRect,
    ACTIVE_COUNT_SCROLL_TRAVEL, ACTIVE_COUNT_TEXT_OFFSET_X, ACTIVE_COUNT_TEXT_WIDTH,
    COMPACT_PILL_RADIUS,
};
use reef_draw::primitive::{DrawPlan, DrawPrimitive, PathSegment};

use super::super::action_button_visual_spec::{
    action_button_visual_frame_for_phase, resolve_action_button_visual_specs,
    ActionButtonVisibilitySpec, ActionButtonVisualSpec, ActionButtonVisualSpecInput,
};
use super::super::completion_glow_visual_spec::{
    resolve_completion_glow_visual_spec, CompletionGlowVisualSpecInput,
};
use super::super::descriptors::NativePanelEdgeAction;
use super::super::visual_primitives::{
    draw_point, draw_rect, native_panel_visual_text_box_height, native_panel_visual_text_frame,
    NativePanelVisualShoulderSide, NativePanelVisualTextAlignment, NativePanelVisualTextRole,
    NativePanelVisualTextWeight,
};

use super::input::{NativePanelPaintInput, NativePanelVisualDisplayMode};

use super::utils::{
    compact_collapsed_alpha, compact_digit_y, compact_headline_y, fit_text_to_width, non_zero_rect,
    visual_panel_frame,
};
use reef_theme::{compact_bar as compact_theme, panel as theme};

pub fn resolve_native_panel_compact_bar_visual_plan(input: &NativePanelPaintInput) -> DrawPlan {
    if input.display_mode == NativePanelVisualDisplayMode::Hidden || !input.window_state.visible {
        return DrawPlan {
            hidden: true,
            viewport: reef_core::geometry::Size {
                width: 0.0,
                height: 0.0,
            },
            primitives: Vec::new(),
        };
    }

    let mut primitives = Vec::new();
    let panel_frame = visual_panel_frame(input);
    let compact_frame = non_zero_rect(input.compact_bar_frame).unwrap_or(panel_frame);
    let expanded_display_mode = super::native_panel_visual_expanded_display_mode(input);
    let chrome_visibility = super::resolve_compact_chrome_visibility(input, expanded_display_mode);
    let action_button_visibility = chrome_visibility.action_buttons;

    if input.display_mode == NativePanelVisualDisplayMode::Compact {
        push_compact_island_background(&mut primitives, input, compact_frame);
    }

    let compact_content = compact_content_layout(compact_frame, false);
    let collapsed_alpha = compact_collapsed_alpha(chrome_visibility);
    push_compact_headline_primitive(
        &mut primitives,
        input,
        compact_frame,
        compact_content,
        collapsed_alpha,
        expanded_display_mode,
    );
    push_compact_metrics_primitives(
        &mut primitives,
        input,
        compact_frame,
        compact_content,
        chrome_visibility,
        collapsed_alpha,
    );
    push_compact_action_button_primitives(
        &mut primitives,
        input,
        compact_frame,
        action_button_visibility,
    );

    DrawPlan {
        hidden: false,
        viewport: reef_core::geometry::Size {
            width: panel_frame.width,
            height: panel_frame.height,
        },
        primitives,
    }
}

pub(super) fn compact_content_layout(
    compact_frame: PanelRect,
    actions_active: bool,
) -> CompactBarContentLayout {
    let mut layout = resolve_compact_bar_content_layout(CompactBarContentLayoutInput {
        bar_width: compact_frame.width,
        bar_height: compact_frame.height,
    });
    if actions_active {
        let action_layout = resolve_compact_action_button_layout(PanelRect {
            x: 0.0,
            y: 0.0,
            width: compact_frame.width,
            height: compact_frame.height,
        });
        let side_gap = 4.0;
        let safe_left = action_layout.settings.x + action_layout.settings.width + side_gap;
        let safe_right = action_layout.quit.x - side_gap;
        let center_x = compact_frame.width / 2.0;
        let centered_safe_width = ((center_x - safe_left).min(safe_right - center_x) * 2.0)
            .clamp(0.0, layout.headline_width);
        if centered_safe_width > 0.0 {
            layout.headline_width = centered_safe_width;
            layout.headline_x = (compact_frame.width / 2.0) - centered_safe_width / 2.0;
            layout.headline_center_x = compact_frame.width / 2.0;
        }
    }
    layout
}

pub(super) fn push_compact_island_background(
    primitives: &mut Vec<DrawPrimitive>,
    input: &NativePanelPaintInput,
    compact_frame: PanelRect,
) {
    push_compact_shoulder_primitive(
        primitives,
        input.left_shoulder_frame,
        NativePanelVisualShoulderSide::Left,
        input.shoulder_progress,
    );
    push_compact_shoulder_primitive(
        primitives,
        input.right_shoulder_frame,
        NativePanelVisualShoulderSide::Right,
        input.shoulder_progress,
    );
    primitives.push(DrawPrimitive::Path {
        segments: compact_pill_path_segments(compact_frame, COMPACT_PILL_RADIUS),
        fill: theme::SHELL_FILL.into(),
        alpha: 1.0,
    });
}

pub(super) fn push_compact_headline_primitive(
    primitives: &mut Vec<DrawPrimitive>,
    input: &NativePanelPaintInput,
    compact_frame: PanelRect,
    compact_content: CompactBarContentLayout,
    collapsed_alpha: f64,
    expanded_display_mode: bool,
) {
    let headline_alpha = if expanded_display_mode {
        1.0
    } else {
        collapsed_alpha
    };
    let headline_text = fit_text_to_width(
        &input.headline_text,
        compact_content.headline_width,
        13.0,
        1,
    );
    let headline_width =
        crate::native_panel_core::resolve_estimated_text_width(&headline_text, 13.0)
            .min(compact_content.headline_width);
    let headline_center_x = compact_frame.x + compact_content.headline_center_x;
    let role = NativePanelVisualTextRole::CompactHeadline;
    let origin = PanelPoint {
        x: headline_center_x - headline_width / 2.0,
        y: compact_frame.y + compact_headline_y(compact_frame.height),
    };
    primitives.push(DrawPrimitive::Text {
        frame: native_panel_visual_text_frame(role, origin, headline_width, &headline_text, 13),
        text: headline_text,
        color: if input.headline_emphasized {
            compact_theme::HEADLINE_EMPHASIZED.into()
        } else {
            compact_theme::HEADLINE.into()
        },
        size: 13,
        weight: NativePanelVisualTextWeight::Semibold.into(),
        alignment: NativePanelVisualTextAlignment::Center.into(),
        alpha: headline_alpha,
    });
}

pub(super) fn push_compact_metrics_primitives(
    primitives: &mut Vec<DrawPrimitive>,
    input: &NativePanelPaintInput,
    compact_frame: PanelRect,
    compact_content: CompactBarContentLayout,
    chrome_visibility: PanelChromeVisibilitySpec,
    collapsed_alpha: f64,
) {
    if !chrome_visibility.collapsed_metrics_visible
        || (input.active_count.is_empty() && input.total_count.is_empty())
    {
        return;
    }

    let active_count_marquee = resolve_active_count_marquee_frame(ActiveCountMarqueeInput {
        text: &input.active_count,
        elapsed_ms: input.active_count_elapsed_ms,
    });
    let active_count_y = compact_frame.y + compact_digit_y(compact_frame.height);
    let active_count_x = compact_frame.x + compact_content.active_x + ACTIVE_COUNT_TEXT_OFFSET_X;
    let active_count_color = if input.active_count.parse::<usize>().unwrap_or_default() > 0 {
        compact_theme::COUNT_ACTIVE.into()
    } else {
        compact_theme::COUNT_INACTIVE.into()
    };
    let role = NativePanelVisualTextRole::CompactActiveCount;
    let origin = PanelPoint {
        x: active_count_x,
        y: active_count_y - active_count_marquee.scroll_offset,
    };
    primitives.push(DrawPrimitive::Text {
        frame: native_panel_visual_text_frame(
            role,
            origin,
            ACTIVE_COUNT_TEXT_WIDTH,
            &active_count_marquee.current,
            15,
        ),
        text: active_count_marquee.current.clone(),
        color: active_count_color,
        size: 15,
        weight: NativePanelVisualTextWeight::Semibold.into(),
        alignment: NativePanelVisualTextAlignment::Right.into(),
        alpha: collapsed_alpha,
    });
    if active_count_marquee.show_next {
        let role = NativePanelVisualTextRole::CompactActiveCountNext;
        let origin = PanelPoint {
            x: active_count_x,
            y: active_count_y + ACTIVE_COUNT_SCROLL_TRAVEL - active_count_marquee.scroll_offset,
        };
        primitives.push(DrawPrimitive::Text {
            frame: native_panel_visual_text_frame(
                role,
                origin,
                ACTIVE_COUNT_TEXT_WIDTH,
                &active_count_marquee.next,
                15,
            ),
            text: active_count_marquee.next.clone(),
            color: compact_theme::COUNT_ACTIVE.into(),
            size: 15,
            weight: NativePanelVisualTextWeight::Semibold.into(),
            alignment: NativePanelVisualTextAlignment::Right.into(),
            alpha: collapsed_alpha,
        });
    }
    if !input.total_count.is_empty() {
        let role = NativePanelVisualTextRole::CompactSlash;
        let origin = PanelPoint {
            x: compact_frame.x + compact_content.slash_x,
            y: compact_frame.y + compact_digit_y(compact_frame.height),
        };
        primitives.push(DrawPrimitive::Text {
            frame: native_panel_visual_text_frame(role, origin, 10.0, "/", 15),
            text: "/".to_string(),
            color: compact_theme::COUNT_TOTAL.into(),
            size: 15,
            weight: NativePanelVisualTextWeight::Semibold.into(),
            alignment: NativePanelVisualTextAlignment::Center.into(),
            alpha: collapsed_alpha,
        });
        let role = NativePanelVisualTextRole::CompactTotalCount;
        let origin = PanelPoint {
            x: compact_frame.x + compact_content.total_x,
            y: compact_frame.y + compact_digit_y(compact_frame.height),
        };
        primitives.push(DrawPrimitive::Text {
            frame: native_panel_visual_text_frame(role, origin, 24.0, &input.total_count, 15),
            text: input.total_count.clone(),
            color: compact_theme::COUNT_TOTAL.into(),
            size: 15,
            weight: NativePanelVisualTextWeight::Semibold.into(),
            alignment: NativePanelVisualTextAlignment::Left.into(),
            alpha: collapsed_alpha,
        });
    }
}

pub(super) fn push_compact_action_button_primitives(
    primitives: &mut Vec<DrawPrimitive>,
    input: &NativePanelPaintInput,
    compact_frame: PanelRect,
    action_button_visibility: ActionButtonVisibilitySpec,
) {
    if !action_button_visibility.reserves_headline_space {
        return;
    }

    let button_frames = input
        .action_buttons
        .iter()
        .map(|button| (button.action, button.frame))
        .collect::<Vec<_>>();
    for spec in resolve_action_button_visual_specs(ActionButtonVisualSpecInput {
        visible: true,
        compact_frame,
        buttons: &button_frames,
        debug_mode_enabled: input
            .action_buttons
            .iter()
            .any(|button| button.debug_mode_enabled),
    }) {
        push_action_button_icon(primitives, &spec, action_button_visibility);
    }
}

pub(super) fn push_completion_glow_if_visible(
    primitives: &mut Vec<DrawPrimitive>,
    input: &NativePanelPaintInput,
    compact_frame: PanelRect,
) {
    let Some(spec) = resolve_completion_glow_visual_spec(CompletionGlowVisualSpecInput {
        visible: input.glow_visible,
        frame: compact_frame,
        base_opacity: input.glow_opacity,
        elapsed_ms: input.mascot_elapsed_ms,
    }) else {
        return;
    };
    primitives.push(DrawPrimitive::NineSliceImage {
        key: "island-completion-inner-glow-9slice.png".to_string(),
        frame: draw_rect(spec.frame),
        slice_left: 24.0,
        slice_right: 24.0,
        slice_top: 24.0,
        slice_bottom: 24.0,
        opacity: spec.opacity,
    });
}

fn push_action_button_icon(
    primitives: &mut Vec<DrawPrimitive>,
    spec: &ActionButtonVisualSpec,
    visibility: ActionButtonVisibilitySpec,
) {
    let frame = action_button_visual_frame_for_phase(
        spec.frame,
        visibility,
        action_button_outward_direction(spec.action),
    );
    let size = ((spec.size as f64) * visibility.scale).round().max(1.0) as i32;
    let text_height = native_panel_visual_text_box_height(&spec.text, size);
    let role = action_button_text_role(spec.action);
    let origin = PanelPoint {
        x: frame.x,
        y: frame.y + ((frame.height - text_height) / 2.0).round() - 1.5,
    };
    primitives.push(DrawPrimitive::Text {
        frame: native_panel_visual_text_frame(role, origin, frame.width, &spec.text, size),
        text: spec.text.clone(),
        color: spec.color.into(),
        size,
        weight: spec.weight.into(),
        alignment: NativePanelVisualTextAlignment::Center.into(),
        alpha: visibility.opacity.clamp(0.0, 1.0),
    });
}

fn action_button_outward_direction(action: NativePanelEdgeAction) -> f64 {
    match action {
        NativePanelEdgeAction::Settings => -1.0,
        NativePanelEdgeAction::Quit => 1.0,
    }
}

fn action_button_text_role(action: NativePanelEdgeAction) -> NativePanelVisualTextRole {
    match action {
        NativePanelEdgeAction::Settings => NativePanelVisualTextRole::ActionButtonSettings,
        NativePanelEdgeAction::Quit => NativePanelVisualTextRole::ActionButtonQuit,
    }
}

fn push_compact_shoulder_primitive(
    primitives: &mut Vec<DrawPrimitive>,
    frame: PanelRect,
    side: NativePanelVisualShoulderSide,
    progress: f64,
) {
    if frame.width <= 0.0 || frame.height <= 0.0 {
        return;
    }
    let progress = progress.clamp(0.0, 1.0);
    if progress >= 0.999 {
        return;
    }
    primitives.push(DrawPrimitive::Path {
        segments: compact_shoulder_path_segments(frame, side, progress),
        fill: theme::SHELL_FILL.into(),
        alpha: 1.0,
    });
}

fn compact_shoulder_path_segments(
    frame: PanelRect,
    side: NativePanelVisualShoulderSide,
    progress: f64,
) -> Vec<PathSegment> {
    let scale_x = (1.0 - progress.clamp(0.0, 1.0)).clamp(0.0, 1.0);
    let top = frame.y + frame.height;
    let bottom = frame.y;
    let control_y =
        frame.y + frame.height * crate::native_panel_core::COMPACT_SHOULDER_CURVE_FACTOR;
    match side {
        NativePanelVisualShoulderSide::Left => {
            let left = frame.x + frame.width * (1.0 - scale_x);
            let right = frame.x + frame.width;
            let control_x = right
                - frame.width
                    * (1.0 - crate::native_panel_core::COMPACT_SHOULDER_CURVE_FACTOR)
                    * scale_x;
            vec![
                PathSegment::LineTo(draw_point(PanelPoint { x: left, y: top })),
                PathSegment::LineTo(draw_point(PanelPoint { x: right, y: top })),
                PathSegment::LineTo(draw_point(PanelPoint {
                    x: right,
                    y: bottom,
                })),
                PathSegment::CubicBezier {
                    control1: draw_point(PanelPoint {
                        x: right,
                        y: control_y,
                    }),
                    control2: draw_point(PanelPoint {
                        x: control_x,
                        y: top,
                    }),
                    end: draw_point(PanelPoint { x: left, y: top }),
                },
            ]
        }
        NativePanelVisualShoulderSide::Right => {
            let left = frame.x;
            let right = frame.x + frame.width * scale_x;
            let control_x = frame.x
                + frame.width * crate::native_panel_core::COMPACT_SHOULDER_CURVE_FACTOR * scale_x;
            vec![
                PathSegment::LineTo(draw_point(PanelPoint { x: right, y: top })),
                PathSegment::LineTo(draw_point(PanelPoint { x: left, y: top })),
                PathSegment::LineTo(draw_point(PanelPoint { x: left, y: bottom })),
                PathSegment::CubicBezier {
                    control1: draw_point(PanelPoint {
                        x: left,
                        y: control_y,
                    }),
                    control2: draw_point(PanelPoint {
                        x: control_x,
                        y: top,
                    }),
                    end: draw_point(PanelPoint { x: right, y: top }),
                },
            ]
        }
    }
}

fn compact_pill_path_segments(frame: PanelRect, radius: f64) -> Vec<PathSegment> {
    const ARC_CONTROL_FACTOR: f64 = 0.552_284_749_830_793_6;

    let radius = radius
        .max(0.0)
        .min(frame.width / 2.0)
        .min(frame.height.max(0.0));
    let control = radius * ARC_CONTROL_FACTOR;
    let left = frame.x;
    let right = frame.x + frame.width;
    let top = frame.y + frame.height;
    let bottom = frame.y;

    vec![
        PathSegment::LineTo(draw_point(PanelPoint { x: left, y: top })),
        PathSegment::LineTo(draw_point(PanelPoint { x: right, y: top })),
        PathSegment::LineTo(draw_point(PanelPoint {
            x: right,
            y: bottom + radius,
        })),
        PathSegment::CubicBezier {
            control1: draw_point(PanelPoint {
                x: right,
                y: bottom + radius - control,
            }),
            control2: draw_point(PanelPoint {
                x: right - radius + control,
                y: bottom,
            }),
            end: draw_point(PanelPoint {
                x: right - radius,
                y: bottom,
            }),
        },
        PathSegment::LineTo(draw_point(PanelPoint {
            x: left + radius,
            y: bottom,
        })),
        PathSegment::CubicBezier {
            control1: draw_point(PanelPoint {
                x: left + radius - control,
                y: bottom,
            }),
            control2: draw_point(PanelPoint {
                x: left,
                y: bottom + radius - control,
            }),
            end: draw_point(PanelPoint {
                x: left,
                y: bottom + radius,
            }),
        },
    ]
}
