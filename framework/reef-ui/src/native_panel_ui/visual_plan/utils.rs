use crate::native_panel_core::{lerp, resolve_estimated_text_width, PanelRect};

use crate::native_panel_core::PanelChromeVisibilitySpec;

use super::super::visual_primitives::{
    native_panel_visual_text_box_height, native_panel_visual_text_box_height_for_role,
    NativePanelDrawPrimitive, NativePanelVisualColor,
};
use super::input::NativePanelDrawPlanInput;

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
    primitive: &NativePanelDrawPrimitive,
    bounds: PanelRect,
) -> bool {
    let Some((bottom, top)) = primitive_vertical_bounds(primitive) else {
        return true;
    };
    top > bounds.y && bottom < bounds.y + bounds.height
}

pub(super) fn primitive_vertical_bounds(
    primitive: &NativePanelDrawPrimitive,
) -> Option<(f64, f64)> {
    match primitive {
        NativePanelDrawPrimitive::RoundRect { frame, .. }
        | NativePanelDrawPrimitive::Rect { frame, .. }
        | NativePanelDrawPrimitive::Ellipse { frame, .. }
        | NativePanelDrawPrimitive::MascotRoundRect { frame, .. }
        | NativePanelDrawPrimitive::MascotEllipse { frame, .. }
        | NativePanelDrawPrimitive::MascotSprite { frame, .. }
        | NativePanelDrawPrimitive::CompactShoulder { frame, .. }
        | NativePanelDrawPrimitive::CompletionGlow { frame, .. }
        | NativePanelDrawPrimitive::ClipStart { frame } => Some((frame.y, frame.y + frame.height)),
        NativePanelDrawPrimitive::StrokeLine { from, to, .. } => {
            Some((from.y.min(to.y), from.y.max(to.y)))
        }
        NativePanelDrawPrimitive::Text {
            origin,
            text,
            size,
            role,
            ..
        } => {
            let height = native_panel_visual_text_box_height_for_role(*role, text, *size);
            Some((origin.y, origin.y + height))
        }
        NativePanelDrawPrimitive::MascotText {
            origin, text, size, ..
        } => {
            let height = native_panel_visual_text_box_height(text, *size);
            Some((origin.y, origin.y + height))
        }
        NativePanelDrawPrimitive::MascotDot { frame, .. } => {
            Some((frame.y, frame.y + frame.height))
        }
        NativePanelDrawPrimitive::ClipEnd => None,
    }
}

// ---- animation / reveal utilities ----

pub(super) fn apply_card_content_reveal_to_primitives(
    primitives: &mut [NativePanelDrawPrimitive],
    translate_y: f64,
    progress: f64,
    fade_base: NativePanelVisualColor,
) {
    for primitive in primitives {
        translate_primitive_y(primitive, translate_y);
        fade_primitive_color(primitive, fade_base, progress);
    }
}

fn translate_primitive_y(primitive: &mut NativePanelDrawPrimitive, translate_y: f64) {
    match primitive {
        NativePanelDrawPrimitive::RoundRect { frame, .. }
        | NativePanelDrawPrimitive::Rect { frame, .. }
        | NativePanelDrawPrimitive::Ellipse { frame, .. }
        | NativePanelDrawPrimitive::MascotRoundRect { frame, .. }
        | NativePanelDrawPrimitive::MascotEllipse { frame, .. }
        | NativePanelDrawPrimitive::MascotSprite { frame, .. }
        | NativePanelDrawPrimitive::CompactShoulder { frame, .. }
        | NativePanelDrawPrimitive::CompletionGlow { frame, .. }
        | NativePanelDrawPrimitive::ClipStart { frame } => {
            frame.y += translate_y;
        }
        NativePanelDrawPrimitive::StrokeLine { from, to, .. } => {
            from.y += translate_y;
            to.y += translate_y;
        }
        NativePanelDrawPrimitive::Text { origin, .. } => {
            origin.y += translate_y;
        }
        NativePanelDrawPrimitive::MascotText { origin, .. } => {
            origin.y += translate_y;
        }
        NativePanelDrawPrimitive::MascotDot { center, frame, .. } => {
            center.y += translate_y;
            frame.y += translate_y;
        }
        NativePanelDrawPrimitive::ClipEnd => {}
    }
}

fn fade_primitive_color(
    primitive: &mut NativePanelDrawPrimitive,
    fade_base: NativePanelVisualColor,
    progress: f64,
) {
    match primitive {
        NativePanelDrawPrimitive::RoundRect { color, .. }
        | NativePanelDrawPrimitive::Rect { color, .. }
        | NativePanelDrawPrimitive::Ellipse { color, .. }
        | NativePanelDrawPrimitive::MascotRoundRect { color, .. }
        | NativePanelDrawPrimitive::MascotEllipse { color, .. }
        | NativePanelDrawPrimitive::StrokeLine { color, .. }
        | NativePanelDrawPrimitive::Text { color, .. } => {
            *color = blend_visual_color(fade_base, *color, progress);
        }
        NativePanelDrawPrimitive::MascotText { color, alpha, .. } => {
            *color = blend_visual_color(fade_base, *color, progress);
            *alpha *= progress.clamp(0.0, 1.0);
        }
        NativePanelDrawPrimitive::CompactShoulder { fill, border, .. } => {
            *fill = blend_visual_color(fade_base, *fill, progress);
            *border = blend_visual_color(fade_base, *border, progress);
        }
        NativePanelDrawPrimitive::CompletionGlow { opacity, .. } => {
            *opacity *= progress.clamp(0.0, 1.0);
        }
        NativePanelDrawPrimitive::MascotSprite { opacity, .. } => {
            *opacity *= progress.clamp(0.0, 1.0);
        }
        NativePanelDrawPrimitive::MascotDot { .. }
        | NativePanelDrawPrimitive::ClipStart { .. }
        | NativePanelDrawPrimitive::ClipEnd => {}
    }
}

fn blend_visual_color(
    from: NativePanelVisualColor,
    to: NativePanelVisualColor,
    progress: f64,
) -> NativePanelVisualColor {
    let progress = progress.clamp(0.0, 1.0);
    NativePanelVisualColor::rgb(
        lerp(from.r as f64, to.r as f64, progress).round() as u8,
        lerp(from.g as f64, to.g as f64, progress).round() as u8,
        lerp(from.b as f64, to.b as f64, progress).round() as u8,
    )
}

// ---- visibility / clipping ----

pub fn extend_visible_content_primitives(
    output: &mut Vec<NativePanelDrawPrimitive>,
    primitives: Vec<NativePanelDrawPrimitive>,
    visible_frame: PanelRect,
) {
    let visible_primitives = primitives
        .into_iter()
        .filter(|primitive| primitive_intersects_vertical_bounds(primitive, visible_frame))
        .collect::<Vec<_>>();
    if visible_primitives.is_empty() {
        return;
    }

    output.push(NativePanelDrawPrimitive::ClipStart {
        frame: visible_frame,
    });
    output.extend(visible_primitives);
    output.push(NativePanelDrawPrimitive::ClipEnd);
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

pub(super) fn visual_panel_frame(input: &NativePanelDrawPlanInput) -> PanelRect {
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
