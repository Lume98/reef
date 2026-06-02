mod card_input;
mod compact;
mod expanded;
mod input;
mod mascot;
mod utils;

use crate::native_panel_core::{
    PanelChromeVisibilitySpec, PanelPoint, PanelRect, EXPANDED_PANEL_RADIUS,
};

use super::visual_primitives::{
    NativePanelVisualColor, NativePanelVisualPlan, NativePanelVisualPrimitive,
};

pub use card_input::{
    native_panel_visual_card_input_from_scene_card,
    native_panel_visual_card_input_from_scene_card_with_height,
};
use compact::{
    compact_content_layout, push_compact_action_button_primitives, push_compact_headline_primitive,
    push_compact_island_background, push_compact_metrics_primitives,
    push_completion_glow_if_visible,
};
use expanded::push_expanded_card_shells;
pub use input::{
    NativePanelVisualActionButtonInput, NativePanelVisualCardBadgeInput,
    NativePanelVisualCardBodyLineInput, NativePanelVisualCardBodyRole, NativePanelVisualCardInput,
    NativePanelVisualCardRowInput, NativePanelVisualCardStyle, NativePanelVisualDisplayMode,
    NativePanelVisualPlanInput,
};
use mascot::{apply_mascot_chrome_alpha, push_mascot_primitives};
use utils::{compact_collapsed_alpha, non_zero_rect, visual_panel_frame};

pub fn resolve_native_panel_visual_plan(
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
    let shell_frame = non_zero_rect(input.shell_frame).unwrap_or(panel_frame);
    let content_frame = non_zero_rect(input.content_frame).unwrap_or(panel_frame);
    let expanded_display_mode = native_panel_visual_expanded_display_mode(input);
    let chrome_visibility = resolve_compact_chrome_visibility(input, expanded_display_mode);
    let action_button_visibility = chrome_visibility.action_buttons;

    if input.display_mode == NativePanelVisualDisplayMode::Compact {
        push_compact_island_background(&mut primitives, input, compact_frame);
    }
    if chrome_visibility.collapsed_mascot_visible {
        push_completion_glow_if_visible(&mut primitives, input, compact_frame);
    }

    if input.display_mode == NativePanelVisualDisplayMode::Expanded {
        primitives.push(NativePanelVisualPrimitive::RoundRect {
            frame: shell_frame,
            radius: EXPANDED_PANEL_RADIUS,
            color: NativePanelVisualColor::rgb(12, 12, 15),
        });

        if input.separator_visibility > 0.01 {
            primitives.push(NativePanelVisualPrimitive::Rect {
                frame: PanelRect {
                    x: shell_frame.x + 20.0,
                    y: compact_frame.y + compact_frame.height + 8.0,
                    width: (shell_frame.width - 40.0).max(0.0),
                    height: 1.0,
                },
                color: NativePanelVisualColor::rgb(62, 62, 70),
            });
        }

        push_expanded_card_shells(&mut primitives, input, shell_frame);
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

    let _ = (content_frame, input.cards_visible);

    if chrome_visibility.collapsed_mascot_visible {
        let mascot_spec = super::mascot_visual_spec::resolve_mascot_visual_spec(
            super::mascot_visual_spec::MascotVisualSpecInput {
                body_center: PanelPoint {
                    x: compact_frame.x + compact_content.mascot_center_x,
                    y: compact_frame.y + compact_frame.height / 2.0,
                },
                completion_badge_anchor: PanelPoint {
                    x: compact_frame.x + compact_content.mascot_center_x,
                    y: compact_frame.y + compact_frame.height / 2.0,
                },
                radius: 11.0,
                pose: input.mascot_pose,
                debug_mode_enabled: input.mascot_debug_mode_enabled,
                completion_count: input.completion_count,
                elapsed_ms: input.mascot_elapsed_ms,
                motion_frame: input.mascot_motion_frame,
            },
        );
        let mascot_start_index = primitives.len();
        push_mascot_primitives(&mut primitives, &mascot_spec);
        apply_mascot_chrome_alpha(&mut primitives[mascot_start_index..], collapsed_alpha);
    }

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

pub use compact::resolve_native_panel_compact_bar_visual_plan;

// Re-export utils items needed by tests
pub(crate) use utils::{compact_digit_y, extend_visible_content_primitives};

pub(super) fn native_panel_visual_expanded_display_mode(
    input: &NativePanelVisualPlanInput,
) -> bool {
    input.display_mode == NativePanelVisualDisplayMode::Expanded
}

pub(super) fn resolve_compact_chrome_visibility(
    input: &NativePanelVisualPlanInput,
    expanded_display_mode: bool,
) -> PanelChromeVisibilitySpec {
    let chrome_transition_progress = if expanded_display_mode {
        input.chrome_transition_progress.clamp(0.0, 1.0)
    } else {
        0.0
    };
    crate::native_panel_core::resolve_panel_chrome_visibility_spec(
        crate::native_panel_core::PanelChromeVisibilitySpecInput {
            edge_actions_visible: input.action_buttons_visible,
            expanded_display_mode,
            surface: input.surface,
            transition_visibility_progress: chrome_transition_progress,
        },
    )
}

#[cfg(test)]
mod tests;
