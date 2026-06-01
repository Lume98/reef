use crate::native_panel_core::{PanelPoint, PanelRect};

use super::visual_primitives::NativePanelVisualColor;
use super::visual_plan::{
    NativePanelVisualCardBodyRole, NativePanelVisualCardInput, NativePanelVisualCardStyle,
    NativePanelVisualDisplayMode, NativePanelVisualPlanInput,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativePanelPanelColors {
    pub shell_fill: NativePanelVisualColor,
    pub shell_border: NativePanelVisualColor,
    pub separator: NativePanelVisualColor,
    pub text_primary: NativePanelVisualColor,
    pub text_secondary: NativePanelVisualColor,
}

impl Default for NativePanelPanelColors {
    fn default() -> Self {
        Self {
            shell_fill: NativePanelVisualColor::rgb(12, 12, 15),
            shell_border: NativePanelVisualColor::rgb(44, 44, 50),
            separator: NativePanelVisualColor::rgb(62, 62, 70),
            text_primary: NativePanelVisualColor::rgb(245, 247, 252),
            text_secondary: NativePanelVisualColor::rgb(230, 235, 245),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelContainerComponent {
    pub frame: PanelRect,
    pub radius: f64,
    pub fill: NativePanelVisualColor,
    pub border: Option<NativePanelVisualColor>,
    pub separator: Option<PanelRect>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelCompactBarComponent {
    pub frame: PanelRect,
    pub headline_origin: PanelPoint,
    pub headline_width: f64,
    pub active_origin: PanelPoint,
    pub total_origin: PanelPoint,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelStackComponent {
    pub frame: PanelRect,
    pub content_height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelSettingRowComponent {
    pub frame: PanelRect,
    pub title_frame: PanelRect,
    pub value_frame: PanelRect,
    pub active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelSessionCardComponent {
    pub frame: PanelRect,
    pub title_frame: PanelRect,
    pub body_frame: PanelRect,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelMastheadComponent {
    pub anchor: PanelPoint,
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NativePanelComponent {
    Container(NativePanelContainerComponent),
    CompactBar(NativePanelCompactBarComponent),
    Stack(NativePanelStackComponent),
    SettingRow(NativePanelSettingRowComponent),
    SessionCard(NativePanelSessionCardComponent),
    Masthead(NativePanelMastheadComponent),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NativePanelComponentTree {
    pub components: Vec<NativePanelComponent>,
}

impl NativePanelComponentTree {
    pub fn push(&mut self, component: NativePanelComponent) {
        self.components.push(component);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelLayoutSpacing {
    pub card_gap: f64,
    pub row_gap: f64,
    pub card_title_height: f64,
    pub card_body_inset: f64,
}

impl Default for NativePanelLayoutSpacing {
    fn default() -> Self {
        Self {
            card_gap: 8.0,
            row_gap: 6.0,
            card_title_height: 24.0,
            card_body_inset: 10.0,
        }
    }
}

pub fn build_native_panel_component_tree(
    input: &NativePanelVisualPlanInput,
) -> NativePanelComponentTree {
    let mut tree = NativePanelComponentTree::default();
    let spacing = NativePanelLayoutSpacing::default();

    tree.push(NativePanelComponent::Container(
        NativePanelContainerComponent {
            frame: input.panel_frame,
            radius: if input.display_mode == NativePanelVisualDisplayMode::Expanded {
                crate::native_panel_core::EXPANDED_PANEL_RADIUS
            } else {
                crate::native_panel_core::COMPACT_PILL_RADIUS
            },
            fill: NativePanelVisualColor::rgb(12, 12, 15),
            border: Some(NativePanelVisualColor::rgb(44, 44, 50)),
            separator: (input.separator_visibility > 0.01).then_some(PanelRect {
                x: input.shell_frame.x + 20.0,
                y: input.compact_bar_frame.y + input.compact_bar_frame.height + 8.0,
                width: (input.shell_frame.width - 40.0).max(0.0),
                height: 1.0,
            }),
        },
    ));

    tree.push(NativePanelComponent::CompactBar(
        NativePanelCompactBarComponent {
            frame: input.compact_bar_frame,
            headline_origin: PanelPoint {
                x: input.compact_bar_frame.x + input.compact_bar_frame.width / 2.0,
                y: input.compact_bar_frame.y + 8.0,
            },
            headline_width: input.compact_bar_frame.width * 0.6,
            active_origin: PanelPoint {
                x: input.compact_bar_frame.x + 16.0,
                y: input.compact_bar_frame.y + 8.0,
            },
            total_origin: PanelPoint {
                x: input.compact_bar_frame.x + input.compact_bar_frame.width - 36.0,
                y: input.compact_bar_frame.y + 8.0,
            },
        },
    ));

    tree.push(NativePanelComponent::Stack(NativePanelStackComponent {
        frame: input.card_stack_frame,
        content_height: input.card_stack_content_height,
    }));

    let mut cursor_y = input.card_stack_frame.y;
    for card in &input.cards {
        let card_frame = PanelRect {
            x: input.card_stack_frame.x,
            y: cursor_y,
            width: input.card_stack_frame.width,
            height: card.height.max(card.collapsed_height).max(1.0),
        };
        push_card_components(&mut tree, card_frame, card, &spacing);
        cursor_y += card_frame.height + spacing.card_gap;
    }

    if input.display_mode != NativePanelVisualDisplayMode::Hidden {
        tree.push(NativePanelComponent::Masthead(NativePanelMastheadComponent {
            anchor: PanelPoint {
                x: input.compact_bar_frame.x + input.compact_bar_frame.width / 2.0,
                y: input.compact_bar_frame.y + input.compact_bar_frame.height / 2.0,
            },
            radius: 11.0,
        }));
    }

    tree
}

fn push_card_components(
    tree: &mut NativePanelComponentTree,
    frame: PanelRect,
    card: &NativePanelVisualCardInput,
    spacing: &NativePanelLayoutSpacing,
) {
    match card.style {
        NativePanelVisualCardStyle::Settings => {
            push_setting_row_components(tree, frame, card, spacing);
        }
        _ => {
            tree.push(NativePanelComponent::SessionCard(
                NativePanelSessionCardComponent {
                    frame,
                    title_frame: PanelRect {
                        x: frame.x + spacing.card_body_inset,
                        y: frame.y + spacing.card_body_inset,
                        width: (frame.width - spacing.card_body_inset * 2.0).max(0.0),
                        height: spacing.card_title_height,
                    },
                    body_frame: PanelRect {
                        x: frame.x + spacing.card_body_inset,
                        y: frame.y + spacing.card_title_height + spacing.card_body_inset,
                        width: (frame.width - spacing.card_body_inset * 2.0).max(0.0),
                        height: (frame.height - spacing.card_title_height - spacing.card_body_inset * 2.0)
                            .max(0.0),
                    },
                },
            ));
        }
    }
}

fn push_setting_row_components(
    tree: &mut NativePanelComponentTree,
    frame: PanelRect,
    card: &NativePanelVisualCardInput,
    spacing: &NativePanelLayoutSpacing,
) {
    let row_count = card.rows.len().max(1) as f64;
    let row_height = ((frame.height - spacing.row_gap * (row_count - 1.0)) / row_count).max(1.0);
    let mut row_y = frame.y;
    for row in &card.rows {
        let row_frame = PanelRect {
            x: frame.x,
            y: row_y,
            width: frame.width,
            height: row_height,
        };
        tree.push(NativePanelComponent::SettingRow(
            NativePanelSettingRowComponent {
                frame: row_frame,
                title_frame: PanelRect {
                    x: row_frame.x + spacing.card_body_inset,
                    y: row_frame.y + 8.0,
                    width: (row_frame.width * 0.62).max(0.0),
                    height: 18.0,
                },
                value_frame: PanelRect {
                    x: row_frame.x + (row_frame.width * 0.72),
                    y: row_frame.y + 6.0,
                    width: (row_frame.width * 0.22).max(0.0),
                    height: 20.0,
                },
                active: row.active,
            },
        ));
        row_y += row_height + spacing.row_gap;
    }
}

pub fn build_native_panel_component_tree_from_visual_plan(
    input: &NativePanelVisualPlanInput,
) -> NativePanelComponentTree {
    build_native_panel_component_tree(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        native_panel_core::{ExpandedSurface, PanelRect},
        native_panel_scene::SceneMascotPose,
    };

    fn plan_input() -> NativePanelVisualPlanInput {
        NativePanelVisualPlanInput {
            window_state: super::super::descriptors::NativePanelHostWindowState {
                frame: Some(PanelRect {
                    x: 0.0,
                    y: 0.0,
                    width: 400.0,
                    height: 200.0,
                }),
                visible: true,
                preferred_display_index: 0,
            },
            display_mode: NativePanelVisualDisplayMode::Expanded,
            surface: ExpandedSurface::Default,
            panel_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 400.0,
                height: 200.0,
            },
            compact_bar_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 400.0,
                height: 36.0,
            },
            left_shoulder_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 20.0,
                height: 36.0,
            },
            right_shoulder_frame: PanelRect {
                x: 380.0,
                y: 0.0,
                width: 20.0,
                height: 36.0,
            },
            shoulder_progress: 0.0,
            content_frame: PanelRect {
                x: 0.0,
                y: 36.0,
                width: 400.0,
                height: 164.0,
            },
            card_stack_frame: PanelRect {
                x: 12.0,
                y: 44.0,
                width: 376.0,
                height: 146.0,
            },
            card_stack_content_height: 88.0,
            shell_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 400.0,
                height: 200.0,
            },
            headline_text: "Idle".to_string(),
            headline_emphasized: false,
            active_count: "0".to_string(),
            active_count_elapsed_ms: 0,
            total_count: "1".to_string(),
            separator_visibility: 1.0,
            chrome_transition_progress: 1.0,
            cards_visible: true,
            card_count: 1,
            cards: vec![super::super::visual_plan::NativePanelVisualCardInput {
                style: NativePanelVisualCardStyle::Default,
                title: "Session".to_string(),
                subtitle: None,
                body: Some("hello".to_string()),
                badge: None,
                source_badge: None,
                body_prefix: None,
                body_lines: vec![],
                action_hint: None,
                rows: vec![],
                height: 72.0,
                collapsed_height: 52.0,
                compact: false,
                removing: false,
            }],
            glow_visible: false,
            glow_opacity: 0.0,
            action_buttons_visible: false,
            action_buttons: vec![],
            completion_count: 0,
            mascot_elapsed_ms: 0,
            mascot_motion_frame: None,
            mascot_pose: SceneMascotPose::Idle,
            mascot_debug_mode_enabled: false,
        }
    }

    #[test]
    fn component_tree_contains_core_layout_nodes() {
        let tree = build_native_panel_component_tree(&plan_input());

        assert!(tree
            .components
            .iter()
            .any(|component| matches!(component, NativePanelComponent::Container(_))));
        assert!(tree
            .components
            .iter()
            .any(|component| matches!(component, NativePanelComponent::CompactBar(_))));
        assert!(tree
            .components
            .iter()
            .any(|component| matches!(component, NativePanelComponent::Stack(_))));
        assert!(tree
            .components
            .iter()
            .any(|component| matches!(component, NativePanelComponent::SessionCard(_))));
    }
}
