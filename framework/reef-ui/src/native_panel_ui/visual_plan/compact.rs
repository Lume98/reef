use crate::native_panel_core::{
    resolve_active_count_marquee_frame, resolve_compact_action_button_layout,
    resolve_compact_bar_content_layout, ActiveCountMarqueeInput, CompactBarContentLayout,
    CompactBarContentLayoutInput, PanelChromeVisibilitySpec, PanelPoint, PanelRect,
    ACTIVE_COUNT_SCROLL_TRAVEL, ACTIVE_COUNT_TEXT_OFFSET_X, ACTIVE_COUNT_TEXT_WIDTH,
    COMPACT_PILL_RADIUS,
};

use super::super::action_button_visual_spec::{
    action_button_visual_frame_for_phase, resolve_action_button_visual_specs,
    ActionButtonVisibilitySpec, ActionButtonVisualSpec, ActionButtonVisualSpecInput,
};
use super::super::completion_glow_visual_spec::{
    resolve_completion_glow_visual_spec, CompletionGlowVisualSpecInput,
};
use super::super::descriptors::NativePanelEdgeAction;
use super::super::visual_primitives::{
    native_panel_visual_text_box_height, NativePanelVisualPlan, NativePanelVisualPrimitive,
    NativePanelVisualShoulderSide, NativePanelVisualTextAlignment, NativePanelVisualTextRole,
    NativePanelVisualTextWeight,
};

use super::input::{NativePanelVisualDisplayMode, NativePanelVisualPlanInput};

use super::utils::{
    compact_collapsed_alpha, compact_digit_y, compact_headline_y, fit_text_to_width, non_zero_rect,
    visual_panel_frame,
};
use reef_theme::{compact_bar as compact_theme, panel as theme};

pub fn resolve_native_panel_compact_bar_visual_plan(
    input: &NativePanelVisualPlanInput,
) -> NativePanelVisualPlan {
    if input.display_mode == NativePanelVisualDisplayMode::Hidden || !input.window_state.visible {
        return NativePanelVisualPlan {
            hidden: true,
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

    NativePanelVisualPlan {
        hidden: false,
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
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    input: &NativePanelVisualPlanInput,
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
    primitives.push(NativePanelVisualPrimitive::RoundRect {
        frame: compact_frame,
        radius: COMPACT_PILL_RADIUS,
        color: theme::SHELL_FILL.into(),
    });
}

pub(super) fn push_compact_headline_primitive(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    input: &NativePanelVisualPlanInput,
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
    primitives.push(NativePanelVisualPrimitive::Text {
        role: NativePanelVisualTextRole::CompactHeadline,
        origin: PanelPoint {
            x: headline_center_x - headline_width / 2.0,
            y: compact_frame.y + compact_headline_y(compact_frame.height),
        },
        max_width: headline_width,
        text: headline_text,
        color: if input.headline_emphasized {
            compact_theme::HEADLINE_EMPHASIZED.into()
        } else {
            compact_theme::HEADLINE.into()
        },
        size: 13,
        weight: NativePanelVisualTextWeight::Semibold,
        alignment: NativePanelVisualTextAlignment::Center,
        alpha: headline_alpha,
    });
}

pub(super) fn push_compact_metrics_primitives(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    input: &NativePanelVisualPlanInput,
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
    primitives.push(NativePanelVisualPrimitive::Text {
        role: NativePanelVisualTextRole::CompactActiveCount,
        origin: PanelPoint {
            x: active_count_x,
            y: active_count_y - active_count_marquee.scroll_offset,
        },
        max_width: ACTIVE_COUNT_TEXT_WIDTH,
        text: active_count_marquee.current.clone(),
        color: active_count_color,
        size: 15,
        weight: NativePanelVisualTextWeight::Semibold,
        alignment: NativePanelVisualTextAlignment::Right,
        alpha: collapsed_alpha,
    });
    if active_count_marquee.show_next {
        primitives.push(NativePanelVisualPrimitive::Text {
            role: NativePanelVisualTextRole::CompactActiveCountNext,
            origin: PanelPoint {
                x: active_count_x,
                y: active_count_y + ACTIVE_COUNT_SCROLL_TRAVEL - active_count_marquee.scroll_offset,
            },
            max_width: ACTIVE_COUNT_TEXT_WIDTH,
            text: active_count_marquee.next.clone(),
            color: compact_theme::COUNT_ACTIVE.into(),
            size: 15,
            weight: NativePanelVisualTextWeight::Semibold,
            alignment: NativePanelVisualTextAlignment::Right,
            alpha: collapsed_alpha,
        });
    }
    if !input.total_count.is_empty() {
        primitives.push(NativePanelVisualPrimitive::Text {
            role: NativePanelVisualTextRole::CompactSlash,
            origin: PanelPoint {
                x: compact_frame.x + compact_content.slash_x,
                y: compact_frame.y + compact_digit_y(compact_frame.height),
            },
            max_width: 10.0,
            text: "/".to_string(),
            color: compact_theme::COUNT_TOTAL.into(),
            size: 15,
            weight: NativePanelVisualTextWeight::Semibold,
            alignment: NativePanelVisualTextAlignment::Center,
            alpha: collapsed_alpha,
        });
        primitives.push(NativePanelVisualPrimitive::Text {
            role: NativePanelVisualTextRole::CompactTotalCount,
            origin: PanelPoint {
                x: compact_frame.x + compact_content.total_x,
                y: compact_frame.y + compact_digit_y(compact_frame.height),
            },
            max_width: 24.0,
            text: input.total_count.clone(),
            color: compact_theme::COUNT_TOTAL.into(),
            size: 15,
            weight: NativePanelVisualTextWeight::Semibold,
            alignment: NativePanelVisualTextAlignment::Left,
            alpha: collapsed_alpha,
        });
    }
}

pub(super) fn push_compact_action_button_primitives(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    input: &NativePanelVisualPlanInput,
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
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    input: &NativePanelVisualPlanInput,
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
    primitives.push(NativePanelVisualPrimitive::CompletionGlow {
        frame: spec.frame,
        opacity: spec.opacity,
    });
}

fn push_action_button_icon(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
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
    primitives.push(NativePanelVisualPrimitive::Text {
        role: action_button_text_role(spec.action),
        origin: PanelPoint {
            x: frame.x,
            y: frame.y + ((frame.height - text_height) / 2.0).round() - 1.5,
        },
        max_width: frame.width,
        text: spec.text.clone(),
        color: spec.color,
        size,
        weight: spec.weight,
        alignment: NativePanelVisualTextAlignment::Center,
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
    primitives: &mut Vec<NativePanelVisualPrimitive>,
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
    primitives.push(NativePanelVisualPrimitive::CompactShoulder {
        frame,
        side,
        progress,
        fill: theme::SHELL_FILL.into(),
        border: theme::SHELL_BORDER.into(),
    });
}
