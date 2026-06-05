use crate::native_panel_core::{lerp, resolve_estimated_text_width, PanelRect};
use reef_core::color::Color;
use reef_draw::primitive::DrawPrimitive;

use crate::native_panel_core::PanelChromeVisibilitySpec;

use super::super::visual_primitives::{draw_rect, NativePanelVisualColor};
use super::input::NativePanelPaintInput;

// ---- geometry utilities ----

pub(super) fn non_zero_rect(rect: PanelRect) -> Option<PanelRect> {
    (rect.width > 0.0 && rect.height > 0.0).then_some(rect)
}

pub(super) fn inset_rect(rect: PanelRect, inset: f64) -> PanelRect {
    PanelRect {
        x: rect.x + inset,
        y: rect.y + inset,
        width: (rect.width - inset * 2.0).max(0.0),
        height: (rect.height - inset * 2.0).max(0.0),
    }
}

pub(super) fn clip_rect_vertically(rect: PanelRect, bounds: PanelRect) -> Option<PanelRect> {
    let bottom = rect.y.max(bounds.y);
    let top = (rect.y + rect.height).min(bounds.y + bounds.height);
    (top > bottom).then_some(PanelRect {
        x: rect.x,
        y: bottom,
        width: rect.width,
        height: top - bottom,
    })
}

pub(super) fn primitive_intersects_vertical_bounds(
    primitive: &DrawPrimitive,
    bounds: PanelRect,
) -> bool {
    let Some((bottom, top)) = primitive_vertical_bounds(primitive) else {
        return true;
    };
    top > bounds.y && bottom < bounds.y + bounds.height
}

pub(super) fn primitive_vertical_bounds(primitive: &DrawPrimitive) -> Option<(f64, f64)> {
    match primitive {
        DrawPrimitive::RoundRect { frame, .. }
        | DrawPrimitive::Rect { frame, .. }
        | DrawPrimitive::Ellipse { frame, .. }
        | DrawPrimitive::Image { frame, .. }
        | DrawPrimitive::StrokedRoundRect { frame, .. }
        | DrawPrimitive::NineSliceImage { frame, .. }
        | DrawPrimitive::SpriteImage { frame, .. }
        | DrawPrimitive::Text { frame, .. }
        | DrawPrimitive::ClipStart { frame } => Some((frame.y, frame.y + frame.height)),
        DrawPrimitive::StrokeLine { from, to, .. } => Some((from.y.min(to.y), from.y.max(to.y))),
        DrawPrimitive::Path { segments, .. } => segments
            .iter()
            .map(|segment| match segment {
                reef_draw::primitive::PathSegment::LineTo(point) => point.y,
                reef_draw::primitive::PathSegment::CubicBezier {
                    control1,
                    control2,
                    end,
                } => control1.y.min(control2.y).min(end.y),
            })
            .fold(None, |bounds: Option<(f64, f64)>, y| {
                Some(match bounds {
                    Some((min, max)) => (min.min(y), max.max(y)),
                    None => (y, y),
                })
            }),
        DrawPrimitive::ClipEnd => None,
    }
}

// ---- animation / reveal utilities ----

pub(super) fn apply_card_content_reveal_to_primitives(
    primitives: &mut [DrawPrimitive],
    translate_y: f64,
    progress: f64,
    fade_base: NativePanelVisualColor,
) {
    for primitive in primitives {
        translate_primitive_y(primitive, translate_y);
        fade_primitive_color(primitive, fade_base, progress);
    }
}

fn translate_primitive_y(primitive: &mut DrawPrimitive, translate_y: f64) {
    match primitive {
        DrawPrimitive::RoundRect { frame, .. }
        | DrawPrimitive::Rect { frame, .. }
        | DrawPrimitive::Ellipse { frame, .. }
        | DrawPrimitive::Image { frame, .. }
        | DrawPrimitive::StrokedRoundRect { frame, .. }
        | DrawPrimitive::NineSliceImage { frame, .. }
        | DrawPrimitive::SpriteImage { frame, .. }
        | DrawPrimitive::Text { frame, .. }
        | DrawPrimitive::ClipStart { frame } => {
            frame.y += translate_y;
        }
        DrawPrimitive::StrokeLine { from, to, .. } => {
            from.y += translate_y;
            to.y += translate_y;
        }
        DrawPrimitive::Path { segments, .. } => {
            for segment in segments {
                match segment {
                    reef_draw::primitive::PathSegment::LineTo(point) => point.y += translate_y,
                    reef_draw::primitive::PathSegment::CubicBezier {
                        control1,
                        control2,
                        end,
                    } => {
                        control1.y += translate_y;
                        control2.y += translate_y;
                        end.y += translate_y;
                    }
                }
            }
        }
        DrawPrimitive::ClipEnd => {}
    }
}

fn fade_primitive_color(
    primitive: &mut DrawPrimitive,
    fade_base: NativePanelVisualColor,
    progress: f64,
) {
    let fade_base = Color::from(fade_base);
    match primitive {
        DrawPrimitive::RoundRect { color, .. }
        | DrawPrimitive::Rect { color, .. }
        | DrawPrimitive::Ellipse { color, .. }
        | DrawPrimitive::StrokeLine { color, .. }
        | DrawPrimitive::Text { color, .. }
        | DrawPrimitive::Path { fill: color, .. } => {
            *color = blend_visual_color(fade_base, *color, progress);
        }
        DrawPrimitive::StrokedRoundRect { fill, stroke, .. } => {
            *fill = blend_visual_color(fade_base, *fill, progress);
            *stroke = blend_visual_color(fade_base, *stroke, progress);
        }
        DrawPrimitive::Image { opacity, .. }
        | DrawPrimitive::NineSliceImage { opacity, .. }
        | DrawPrimitive::SpriteImage { opacity, .. } => {
            *opacity *= progress.clamp(0.0, 1.0);
        }
        DrawPrimitive::ClipStart { .. } | DrawPrimitive::ClipEnd => {}
    }
}

fn blend_visual_color(from: Color, to: Color, progress: f64) -> Color {
    let progress = progress.clamp(0.0, 1.0);
    Color::rgb(
        lerp(from.r as f64, to.r as f64, progress).round() as u8,
        lerp(from.g as f64, to.g as f64, progress).round() as u8,
        lerp(from.b as f64, to.b as f64, progress).round() as u8,
    )
}

// ---- visibility / clipping ----

pub fn extend_visible_content_primitives(
    output: &mut Vec<DrawPrimitive>,
    primitives: Vec<DrawPrimitive>,
    visible_frame: PanelRect,
) {
    let visible_primitives = primitives
        .into_iter()
        .filter(|primitive| primitive_intersects_vertical_bounds(primitive, visible_frame))
        .collect::<Vec<_>>();
    if visible_primitives.is_empty() {
        return;
    }

    output.push(DrawPrimitive::ClipStart {
        frame: draw_rect(visible_frame),
    });
    output.extend(visible_primitives);
    output.push(DrawPrimitive::ClipEnd);
}

// ---- text fitting utilities ----

pub(super) fn fit_text_to_width(
    text: &str,
    width: f64,
    font_size: f64,
    max_lines: usize,
) -> String {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.is_empty() {
        return String::new();
    }
    let max_width = width.max(font_size) * max_lines.max(1) as f64;
    if resolve_estimated_text_width(&normalized, font_size) <= max_width {
        return normalized;
    }

    let mut clipped = String::new();
    for ch in normalized.chars() {
        let candidate = format!("{clipped}{ch}...");
        if resolve_estimated_text_width(&candidate, font_size) > max_width {
            break;
        }
        clipped.push(ch);
    }
    if clipped.is_empty() {
        "...".to_string()
    } else {
        format!("{}...", clipped.trim_end())
    }
}

pub(super) fn fit_text_to_lines(
    text: &str,
    width: f64,
    font_size: f64,
    max_lines: usize,
) -> Vec<String> {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.is_empty() {
        return Vec::new();
    }

    let max_lines = max_lines.max(1);
    let mut lines = Vec::new();
    let mut current = String::new();
    for ch in normalized.chars() {
        let candidate = format!("{current}{ch}");
        if !current.is_empty() && resolve_estimated_text_width(&candidate, font_size) > width {
            lines.push(current.trim_end().to_string());
            current.clear();
            if lines.len() == max_lines {
                break;
            }
        }
        current.push(ch);
    }
    if lines.len() < max_lines && !current.is_empty() {
        lines.push(current.trim_end().to_string());
    }

    if lines.len() > max_lines {
        lines.truncate(max_lines);
    }
    if !text_fits_in_lines(&normalized, &lines) {
        if let Some(last) = lines.last_mut() {
            *last = ellipsize_text_to_width(last, width, font_size);
        }
    }
    lines
}

fn text_fits_in_lines(original: &str, lines: &[String]) -> bool {
    lines.join("").chars().count() >= original.chars().count()
}

fn ellipsize_text_to_width(text: &str, width: f64, font_size: f64) -> String {
    let ellipsis = "...";
    if resolve_estimated_text_width(text, font_size) <= width
        && !text.ends_with(ellipsis)
        && resolve_estimated_text_width(&format!("{text}{ellipsis}"), font_size) <= width
    {
        return text.to_string();
    }

    let mut clipped = String::new();
    for ch in text.chars() {
        let candidate = format!("{clipped}{ch}{ellipsis}");
        if resolve_estimated_text_width(&candidate, font_size) > width {
            break;
        }
        clipped.push(ch);
    }
    if clipped.is_empty() {
        ellipsis.to_string()
    } else {
        format!("{}{}", clipped.trim_end(), ellipsis)
    }
}

// ---- position helpers (used by compact module) ----

pub(super) fn compact_headline_y(bar_height: f64) -> f64 {
    ((bar_height - 24.0) / 2.0).round() - 1.5
}

pub fn compact_digit_y(bar_height: f64) -> f64 {
    ((bar_height - crate::native_panel_core::ACTIVE_COUNT_LABEL_HEIGHT) / 2.0).round() - 1.5
}

// ---- frame / visibility helpers ----

pub(super) fn visual_panel_frame(input: &NativePanelPaintInput) -> PanelRect {
    non_zero_rect(input.content_frame)
        .or_else(|| {
            input.window_state.frame.map(|frame| PanelRect {
                x: 0.0,
                y: 0.0,
                width: frame.width,
                height: frame.height,
            })
        })
        .unwrap_or(input.panel_frame)
}

pub(super) fn compact_collapsed_alpha(chrome_visibility: PanelChromeVisibilitySpec) -> f64 {
    1.0 - chrome_visibility.collapsed_exit_progress.clamp(0.0, 1.0)
}
