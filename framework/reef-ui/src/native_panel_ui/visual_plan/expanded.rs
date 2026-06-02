use crate::native_panel_core::{
    compact_title, default_panel_card_metric_constants, resolve_estimated_chat_body_height,
    resolve_next_stacked_card_frame, PanelPoint, PanelRect, CARD_RADIUS, EXPANDED_CARD_GAP,
    EXPANDED_CARD_OVERHANG,
};

use super::super::card_visual_spec::{
    card_visual_action_hint_layout, card_visual_badge_layout, card_visual_body_layout,
    card_visual_body_line_paint_spec, card_visual_content_layout,
    card_visual_content_transition_frame, card_visual_header_text_paint_spec,
    card_visual_settings_row_layout, card_visual_shell_border_color, card_visual_shell_fill_color,
    card_visual_shell_reveal_frame, card_visual_stack_reveal_frame, card_visual_tool_pill_layout,
    CardVisualBadgeRole, CardVisualBadgeSpec, CardVisualRowSpec,
};
use super::super::visual_primitives::{
    NativePanelVisualColor, NativePanelVisualPrimitive, NativePanelVisualTextAlignment,
    NativePanelVisualTextRole, NativePanelVisualTextWeight,
};
use super::card_input::{
    card_visual_body_role_from_visual_role, card_visual_style_from_visual_style,
    visual_color_from_card_spec,
};
use super::input::{
    NativePanelVisualCardBadgeInput, NativePanelVisualCardBodyRole, NativePanelVisualCardInput,
    NativePanelVisualCardStyle,
};
use super::utils::{
    apply_card_content_reveal_to_primitives, clip_rect_vertically,
    extend_visible_content_primitives, fit_text_to_lines, fit_text_to_width, inset_rect,
};

pub(super) fn push_expanded_card_shells(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    input: &super::input::NativePanelVisualPlanInput,
    shell_frame: PanelRect,
) {
    if !input.cards_visible || input.cards.is_empty() || input.separator_visibility <= 0.01 {
        return;
    }

    let mut cursor_y = input.card_stack_content_height;
    let stack_overflow_y =
        (input.card_stack_content_height - input.card_stack_frame.height).max(0.0);
    let clip_bounds = PanelRect {
        x: shell_frame.x + input.card_stack_frame.x,
        y: shell_frame.y + input.card_stack_frame.y,
        width: input.card_stack_frame.width,
        height: input.card_stack_frame.height,
    };
    for (index, card) in input.cards.iter().enumerate() {
        let single_empty_card =
            input.cards.len() == 1 && card.style == NativePanelVisualCardStyle::Empty;
        let card_height = if single_empty_card {
            card.height
                .min(cursor_y.max(input.card_stack_frame.height))
                .max(card.collapsed_height)
        } else {
            card.height
        };
        let Some(frame) = resolve_next_stacked_card_frame(
            &mut cursor_y,
            index > 0,
            card_height,
            input.card_stack_frame.width,
            EXPANDED_CARD_GAP,
            EXPANDED_CARD_OVERHANG,
        ) else {
            break;
        };
        let stable_card_frame = PanelRect {
            x: shell_frame.x + input.card_stack_frame.x + frame.x,
            y: shell_frame.y + input.card_stack_frame.y + frame.y - stack_overflow_y,
            width: frame.width,
            height: frame.height,
        };
        let stable_visible_height =
            clip_rect_vertically(stable_card_frame, clip_bounds).map(|frame| frame.height);
        if stable_visible_height.is_none_or(|height| height <= 6.0) && !single_empty_card {
            continue;
        }
        let phase =
            card_visual_stack_reveal_frame(input.separator_visibility, input.cards.len(), index)
                .card_phase;
        if phase <= 0.001 {
            continue;
        }
        let expanded_frame = PanelRect {
            x: shell_frame.x + input.card_stack_frame.x + frame.x,
            y: shell_frame.y + input.card_stack_frame.y + frame.y - stack_overflow_y,
            width: frame.width,
            height: frame.height,
        };
        let unclipped_card_frame =
            card_visual_shell_reveal_frame(expanded_frame, card.collapsed_height, phase);
        let Some(card_frame) = clip_rect_vertically(unclipped_card_frame, clip_bounds) else {
            continue;
        };
        if card_frame.height <= 6.0 {
            continue;
        }

        let content_visible =
            single_empty_card || card_frame.height >= card.collapsed_height.min(48.0);
        push_card_shell(primitives, card, card_frame);
        if content_visible {
            let content_layout_frame = if single_empty_card {
                card_frame
            } else {
                unclipped_card_frame
            };
            push_expanded_card_content(primitives, card, content_layout_frame, card_frame, phase);
        }
    }
}

pub(super) fn push_card_shell(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    card: &NativePanelVisualCardInput,
    frame: PanelRect,
) {
    let radius = CARD_RADIUS.min(frame.height / 2.0);
    primitives.push(NativePanelVisualPrimitive::RoundRect {
        frame,
        radius,
        color: card_shell_border_color(card.style),
    });

    let inner = inset_rect(frame, 1.0);
    if inner.width <= 0.0 || inner.height <= 0.0 {
        return;
    }
    primitives.push(NativePanelVisualPrimitive::RoundRect {
        frame: inner,
        radius: (radius - 1.0).max(0.0).min(inner.height / 2.0),
        color: card_shell_fill_color(card.style),
    });
}

fn card_shell_border_color(style: NativePanelVisualCardStyle) -> NativePanelVisualColor {
    visual_color_from_card_spec(card_visual_shell_border_color(
        card_visual_style_from_visual_style(style),
    ))
}

fn card_shell_fill_color(style: NativePanelVisualCardStyle) -> NativePanelVisualColor {
    visual_color_from_card_spec(card_visual_shell_fill_color(
        card_visual_style_from_visual_style(style),
    ))
}

pub(super) fn push_expanded_card_content(
    output: &mut Vec<NativePanelVisualPrimitive>,
    card: &NativePanelVisualCardInput,
    frame: PanelRect,
    visible_frame: PanelRect,
    phase: f64,
) {
    let content_reveal = card_visual_content_transition_frame(phase, card.removing);
    if content_reveal.visibility_progress <= 0.001 || frame.height < card.collapsed_height.min(48.0)
    {
        return;
    }

    let fade_base = card_shell_fill_color(card.style);
    let content_layout = card_visual_content_layout(frame);
    if content_layout.content_width <= 8.0 {
        return;
    }

    let mut content_primitives = Vec::new();
    let primitives = &mut content_primitives;
    let header_text_spec =
        card_visual_header_text_paint_spec(card_visual_style_from_visual_style(card.style));
    if card.style == NativePanelVisualCardStyle::Empty {
        primitives.push(NativePanelVisualPrimitive::Text {
            role: NativePanelVisualTextRole::CardTitle,
            origin: PanelPoint {
                x: frame.x,
                y: content_layout.empty_title_y,
            },
            max_width: frame.width,
            text: card.title.clone(),
            color: visual_color_from_card_spec(header_text_spec.title.color),
            size: header_text_spec.title.size,
            weight: NativePanelVisualTextWeight::Semibold,
            alignment: NativePanelVisualTextAlignment::Center,
            alpha: 1.0,
        });
        apply_card_content_reveal_to_primitives(
            &mut content_primitives,
            content_reveal.translate_y,
            content_reveal.visibility_progress,
            fade_base,
        );
        extend_visible_content_primitives(output, content_primitives, visible_frame);
        return;
    }

    let mut badge_right = frame.x + frame.width - 12.0;
    let status_left = if let Some(badge) = &card.badge {
        push_expanded_card_badge(
            primitives,
            badge,
            badge_right,
            content_layout.title_y,
            card.style,
            CardVisualBadgeRole::Status,
        )
    } else {
        badge_right
    };
    badge_right = status_left;
    let source_left = if let Some(badge) = &card.source_badge {
        push_expanded_card_badge(
            primitives,
            badge,
            badge_right - 6.0,
            content_layout.title_y,
            card.style,
            CardVisualBadgeRole::Source,
        )
    } else {
        status_left
    };
    let title_width = (source_left - content_layout.content_x - 8.0).max(32.0);
    let title_text = fit_text_to_width(
        &compact_title(&card.title, header_text_spec.title_max_chars),
        title_width,
        header_text_spec.title.size as f64,
        1,
    );
    primitives.push(NativePanelVisualPrimitive::Text {
        role: NativePanelVisualTextRole::CardTitle,
        origin: PanelPoint {
            x: content_layout.content_x,
            y: content_layout.title_y,
        },
        max_width: title_width,
        text: title_text,
        color: visual_color_from_card_spec(header_text_spec.title.color),
        size: header_text_spec.title.size,
        weight: NativePanelVisualTextWeight::Semibold,
        alignment: NativePanelVisualTextAlignment::Left,
        alpha: 1.0,
    });

    if let Some(subtitle) = &card.subtitle {
        primitives.push(NativePanelVisualPrimitive::Text {
            role: NativePanelVisualTextRole::CardSubtitle,
            origin: PanelPoint {
                x: content_layout.content_x,
                y: content_layout.subtitle_y,
            },
            max_width: content_layout.content_width,
            text: fit_text_to_width(
                subtitle,
                content_layout.content_width,
                header_text_spec.subtitle.size as f64,
                1,
            ),
            color: visual_color_from_card_spec(header_text_spec.subtitle.color),
            size: header_text_spec.subtitle.size,
            weight: NativePanelVisualTextWeight::Normal,
            alignment: NativePanelVisualTextAlignment::Left,
            alpha: 1.0,
        });
    }

    if !card.rows.is_empty() {
        push_expanded_settings_rows(
            primitives,
            card,
            frame,
            content_layout.content_x,
            content_layout.content_width,
        );
        apply_card_content_reveal_to_primitives(
            &mut content_primitives,
            content_reveal.translate_y,
            content_reveal.visibility_progress,
            fade_base,
        );
        extend_visible_content_primitives(output, content_primitives, visible_frame);
        return;
    }

    if card.body.is_some() || !card.body_lines.is_empty() {
        push_expanded_card_body_line(primitives, card, frame, card.body.as_deref().unwrap_or(""));
    }

    if let Some(action_hint) = &card.action_hint {
        push_pending_action_hint_pill(primitives, frame, action_hint);
    }

    apply_card_content_reveal_to_primitives(
        &mut content_primitives,
        content_reveal.translate_y,
        content_reveal.visibility_progress,
        fade_base,
    );
    extend_visible_content_primitives(output, content_primitives, visible_frame);
}

fn push_expanded_card_body_line(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    card: &NativePanelVisualCardInput,
    frame: PanelRect,
    body: &str,
) {
    let metrics = default_panel_card_metric_constants();
    let body_layout = card_visual_body_layout(frame, card.action_hint.is_some());
    let mut cursor_y = body_layout.initial_y;
    let body_lines = expanded_card_body_lines(card, body);
    for (index, line) in body_lines.iter().enumerate() {
        if line.role == NativePanelVisualCardBodyRole::Tool {
            push_expanded_tool_pill_line(primitives, frame, cursor_y, &line.text);
            cursor_y += 22.0;
            if index + 1 < body_lines.len() {
                cursor_y += metrics.tool_gap;
            }
            continue;
        }

        let body_text =
            fit_text_to_lines(&line.text, body_layout.body_width, 10.0, line.max_lines).join("\n");
        let body_height = resolve_estimated_chat_body_height(
            &body_text,
            body_layout.body_width,
            line.max_lines as isize,
            metrics,
        );
        if let Some(prefix) = &line.prefix {
            let line_spec = card_visual_body_line_paint_spec(
                card_visual_style_from_visual_style(card.style),
                card_visual_body_role_from_visual_role(line.role),
                Some(prefix),
            );
            primitives.push(NativePanelVisualPrimitive::Text {
                role: NativePanelVisualTextRole::CardBodyPrefix,
                origin: PanelPoint {
                    x: body_layout.prefix_x,
                    y: cursor_y,
                },
                max_width: 10.0,
                text: prefix.clone(),
                color: visual_color_from_card_spec(line_spec.prefix_color),
                size: line_spec.prefix_size,
                weight: NativePanelVisualTextWeight::Bold,
                alignment: NativePanelVisualTextAlignment::Center,
                alpha: 1.0,
            });
        }
        let line_spec = card_visual_body_line_paint_spec(
            card_visual_style_from_visual_style(card.style),
            card_visual_body_role_from_visual_role(line.role),
            line.prefix.as_deref(),
        );
        primitives.push(NativePanelVisualPrimitive::Text {
            role: NativePanelVisualTextRole::CardBodyText,
            origin: PanelPoint {
                x: body_layout.text_x,
                y: cursor_y,
            },
            max_width: body_layout.body_width,
            text: body_text,
            color: visual_color_from_card_spec(line_spec.text_color),
            size: line_spec.text_size,
            weight: NativePanelVisualTextWeight::Normal,
            alignment: NativePanelVisualTextAlignment::Left,
            alpha: 1.0,
        });
        cursor_y += body_height;
        if index + 1 < body_lines.len() {
            cursor_y += metrics.chat_gap;
        }
    }
}

fn push_pending_action_hint_pill(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    frame: PanelRect,
    action_hint: &str,
) {
    let Some(layout) = card_visual_action_hint_layout(frame, action_hint) else {
        return;
    };
    primitives.push(NativePanelVisualPrimitive::RoundRect {
        frame: layout.pill_frame,
        radius: layout.paint.radius,
        color: visual_color_from_card_spec(layout.paint.background_color),
    });
    primitives.push(NativePanelVisualPrimitive::Text {
        role: NativePanelVisualTextRole::CardActionHint,
        origin: layout.text_origin,
        max_width: layout.text_max_width,
        text: fit_text_to_width(
            &layout.paint.text,
            layout.text_max_width,
            layout.paint.text_size as f64,
            1,
        ),
        color: visual_color_from_card_spec(layout.paint.foreground_color),
        size: layout.paint.text_size,
        weight: NativePanelVisualTextWeight::Semibold,
        alignment: NativePanelVisualTextAlignment::Left,
        alpha: 1.0,
    });
}

fn push_expanded_tool_pill_line(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    frame: PanelRect,
    y: f64,
    text: &str,
) {
    let Some(layout) = card_visual_tool_pill_layout(frame, y, text) else {
        return;
    };

    primitives.push(NativePanelVisualPrimitive::RoundRect {
        frame: layout.pill_frame,
        radius: layout.paint.radius,
        color: visual_color_from_card_spec(layout.paint.border_color),
    });
    let fill_frame = inset_rect(layout.pill_frame, 1.0);
    if fill_frame.width > 0.0 && fill_frame.height > 0.0 {
        primitives.push(NativePanelVisualPrimitive::RoundRect {
            frame: fill_frame,
            radius: (layout.paint.radius - 1.0).max(0.0),
            color: visual_color_from_card_spec(layout.paint.background_color),
        });
    }

    primitives.push(NativePanelVisualPrimitive::Text {
        role: NativePanelVisualTextRole::CardToolName,
        origin: layout.tool_name_origin,
        max_width: layout.tool_name_max_width,
        text: layout.paint.tool_name.clone(),
        color: visual_color_from_card_spec(layout.paint.tool_name_color),
        size: layout.paint.text_size,
        weight: NativePanelVisualTextWeight::Bold,
        alignment: NativePanelVisualTextAlignment::Left,
        alpha: 1.0,
    });

    if let Some(description) = layout.description {
        primitives.push(NativePanelVisualPrimitive::Text {
            role: NativePanelVisualTextRole::CardToolDescription,
            origin: description.origin,
            max_width: description.max_width,
            text: fit_text_to_width(
                &description.text,
                description.max_width,
                layout.paint.text_size as f64,
                1,
            ),
            color: visual_color_from_card_spec(layout.paint.description_color),
            size: layout.paint.text_size,
            weight: NativePanelVisualTextWeight::Normal,
            alignment: NativePanelVisualTextAlignment::Left,
            alpha: 1.0,
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExpandedCardBodyLine {
    role: NativePanelVisualCardBodyRole,
    prefix: Option<String>,
    text: String,
    max_lines: usize,
}

fn expanded_card_body_lines(
    card: &NativePanelVisualCardInput,
    body: &str,
) -> Vec<ExpandedCardBodyLine> {
    if !card.body_lines.is_empty() {
        return card
            .body_lines
            .iter()
            .filter_map(|line| {
                let text = line.text.trim();
                (!text.is_empty()).then(|| ExpandedCardBodyLine {
                    role: line.role,
                    prefix: line.prefix.clone(),
                    text: text.to_string(),
                    max_lines: line.max_lines.max(1),
                })
            })
            .collect();
    }

    let raw_lines = body
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let prefixes = card
        .body_prefix
        .as_deref()
        .unwrap_or_default()
        .chars()
        .map(|ch| ch.to_string())
        .collect::<Vec<_>>();
    if raw_lines.len() > 1 && prefixes.len() >= raw_lines.len() {
        return raw_lines
            .into_iter()
            .zip(prefixes)
            .map(|(text, prefix)| ExpandedCardBodyLine {
                role: expanded_card_body_role_for_prefix(Some(prefix.as_str())),
                max_lines: expanded_card_body_max_lines_for_prefix(
                    card.style,
                    Some(prefix.as_str()),
                ),
                prefix: Some(prefix),
                text: text.to_string(),
            })
            .collect();
    }

    let prefix = card.body_prefix.clone();
    vec![ExpandedCardBodyLine {
        role: expanded_card_body_role_for_prefix(prefix.as_deref()),
        max_lines: expanded_card_body_max_lines_for_prefix(card.style, prefix.as_deref()),
        prefix,
        text: body.to_string(),
    }]
}

fn expanded_card_body_role_for_prefix(prefix: Option<&str>) -> NativePanelVisualCardBodyRole {
    match prefix {
        Some("$") => NativePanelVisualCardBodyRole::Assistant,
        Some(">") => NativePanelVisualCardBodyRole::User,
        Some("!") => NativePanelVisualCardBodyRole::Tool,
        _ => NativePanelVisualCardBodyRole::Plain,
    }
}

fn expanded_card_body_max_lines_for_prefix(
    style: NativePanelVisualCardStyle,
    prefix: Option<&str>,
) -> usize {
    match (style, prefix) {
        (NativePanelVisualCardStyle::Default, Some(">")) => 1,
        (NativePanelVisualCardStyle::Default, Some("$"))
        | (NativePanelVisualCardStyle::Completion, _)
        | (NativePanelVisualCardStyle::Pending, _)
        | (NativePanelVisualCardStyle::PendingApproval, _)
        | (NativePanelVisualCardStyle::PendingQuestion, _)
        | (NativePanelVisualCardStyle::PromptAssist, _) => 2,
        _ => 1,
    }
}

fn push_expanded_card_badge(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    badge: &NativePanelVisualCardBadgeInput,
    right: f64,
    title_y: f64,
    style: NativePanelVisualCardStyle,
    role: CardVisualBadgeRole,
) -> f64 {
    let badge_spec = CardVisualBadgeSpec {
        role,
        text: badge.text.clone(),
        emphasized: badge.emphasized,
    };
    let layout = card_visual_badge_layout(
        card_visual_style_from_visual_style(style),
        &badge_spec,
        right,
        title_y,
    );
    primitives.push(NativePanelVisualPrimitive::RoundRect {
        frame: layout.badge_frame,
        radius: layout.paint.radius,
        color: visual_color_from_card_spec(layout.paint.background_color),
    });
    primitives.push(NativePanelVisualPrimitive::Text {
        role: card_badge_text_role(role),
        origin: layout.text_origin,
        max_width: layout.text_max_width,
        text: badge.text.clone(),
        color: visual_color_from_card_spec(layout.paint.foreground_color),
        size: layout.paint.text_size,
        weight: NativePanelVisualTextWeight::Semibold,
        alignment: NativePanelVisualTextAlignment::Center,
        alpha: 1.0,
    });
    layout.badge_frame.x
}

fn card_badge_text_role(role: CardVisualBadgeRole) -> NativePanelVisualTextRole {
    match role {
        CardVisualBadgeRole::Status => NativePanelVisualTextRole::CardStatusBadge,
        CardVisualBadgeRole::Source => NativePanelVisualTextRole::CardSourceBadge,
    }
}

fn push_expanded_settings_rows(
    primitives: &mut Vec<NativePanelVisualPrimitive>,
    card: &NativePanelVisualCardInput,
    frame: PanelRect,
    _content_x: f64,
    _content_width: f64,
) {
    for (index, row) in card.rows.iter().enumerate() {
        let row_spec = CardVisualRowSpec {
            title: row.title.clone(),
            value: row.value.clone(),
            active: row.active,
        };
        let Some(layout) = card_visual_settings_row_layout(frame, index, &row_spec) else {
            break;
        };
        primitives.push(NativePanelVisualPrimitive::RoundRect {
            frame: layout.row_frame,
            radius: layout.paint.border_radius,
            color: visual_color_from_card_spec(layout.paint.border_color),
        });
        primitives.push(NativePanelVisualPrimitive::RoundRect {
            frame: layout.row_inner_frame,
            radius: layout.paint.fill_radius,
            color: visual_color_from_card_spec(layout.paint.fill_color),
        });

        primitives.push(NativePanelVisualPrimitive::Text {
            role: NativePanelVisualTextRole::CardSettingsTitle,
            origin: layout.title_origin,
            max_width: layout.title_max_width,
            text: row.title.clone(),
            color: visual_color_from_card_spec(layout.paint.title_color),
            size: layout.paint.title_size,
            weight: NativePanelVisualTextWeight::Semibold,
            alignment: NativePanelVisualTextAlignment::Left,
            alpha: 1.0,
        });
        primitives.push(NativePanelVisualPrimitive::RoundRect {
            frame: layout.value_badge_frame,
            radius: layout.paint.value_badge.radius,
            color: visual_color_from_card_spec(layout.paint.value_badge.background_color),
        });
        primitives.push(NativePanelVisualPrimitive::Text {
            role: NativePanelVisualTextRole::CardSettingsValue,
            origin: layout.value_origin,
            max_width: layout.value_max_width,
            text: fit_text_to_width(
                &row.value,
                layout.value_max_width,
                layout.paint.value_badge.text_size as f64,
                1,
            ),
            color: visual_color_from_card_spec(layout.paint.value_badge.foreground_color),
            size: layout.paint.value_badge.text_size,
            weight: NativePanelVisualTextWeight::Semibold,
            alignment: NativePanelVisualTextAlignment::Center,
            alpha: 1.0,
        });
    }
}
