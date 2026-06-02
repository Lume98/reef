use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

use crate::compact_shoulder::CompactShoulder;
use crate::completion_glow::CompletionGlow;
use crate::mascot::MascotWidget;

const SETTINGS_ICON: &str = "\u{E713}";
const QUIT_ICON: &str = "\u{E7E8}";

/// Chrome visibility spec for transitions between compact and expanded modes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChromeVisibility {
    pub separator_visibility: f64,
    pub shoulder_progress: f64,
    pub collapsed_alpha: f64,
    pub action_button_visibility: f64,
}

impl ChromeVisibility {
    pub fn compact() -> Self {
        Self {
            separator_visibility: 0.0,
            shoulder_progress: 0.0,
            collapsed_alpha: 1.0,
            action_button_visibility: 0.0,
        }
    }

    pub fn expanded() -> Self {
        Self {
            separator_visibility: 1.0,
            shoulder_progress: 1.0,
            collapsed_alpha: 0.0,
            action_button_visibility: 1.0,
        }
    }

    pub fn interpolate(from: &Self, to: &Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            separator_visibility: from.separator_visibility + (to.separator_visibility - from.separator_visibility) * t,
            shoulder_progress: from.shoulder_progress + (to.shoulder_progress - from.shoulder_progress) * t,
            collapsed_alpha: from.collapsed_alpha + (to.collapsed_alpha - from.collapsed_alpha) * t,
            action_button_visibility: from.action_button_visibility + (to.action_button_visibility - from.action_button_visibility) * t,
        }
    }
}

/// Collapsed mini-pill bar with shoulders, marquee, action buttons, and glow.
#[derive(Clone)]
pub struct CompactBar {
    pub headline: String,
    pub headline_emphasized: bool,
    pub active_count: String,
    pub active_count_next: Option<String>,
    pub active_count_scroll: f64,
    pub total_count: String,
    pub completion_count: usize,
    pub mascot: Option<MascotWidget>,
    pub show_actions: bool,
    pub debug_mode: bool,
    pub shoulder_left: Option<CompactShoulder>,
    pub shoulder_right: Option<CompactShoulder>,
    pub glow: Option<CompletionGlow>,
    pub chrome: ChromeVisibility,
    pub action_button_scale: f64,
    pub action_button_opacity: f64,
    pub action_button_offset_y: f64,
    pub fill_color: Color,
    pub border_color: Color,
    pub text_color: Color,
    pub dim_text_color: Color,
    pub radius: f64,
    pub height: f64,
}

impl CompactBar {
    pub fn new() -> Self {
        Self {
            headline: String::new(),
            headline_emphasized: false,
            active_count: String::new(),
            active_count_next: None,
            active_count_scroll: 0.0,
            total_count: String::new(),
            completion_count: 0,
            mascot: None,
            show_actions: false,
            debug_mode: false,
            shoulder_left: None,
            shoulder_right: None,
            glow: None,
            chrome: ChromeVisibility::compact(),
            action_button_scale: 0.82,
            action_button_opacity: 0.0,
            action_button_offset_y: -4.0,
            fill_color: Color::rgb(18, 20, 26),
            border_color: Color::rgb(44, 48, 58),
            text_color: Color::rgb(200, 210, 225),
            dim_text_color: Color::rgb(100, 108, 125),
            radius: 24.0,
            height: 48.0,
        }
    }
}

impl Widget for CompactBar {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size { width: constraints.max_width, height: self.height })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        // ── Glow (behind background) ──────────────────────────────────────
        if let Some(glow) = &self.glow {
            glow.paint(rect, ctx);
        }

        // ── Background pill ──────────────────────────────────────────────
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: rect,
            radius: self.radius,
            color: self.fill_color,
            alpha: 1.0,
        });
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: rect,
            radius: self.radius,
            color: self.border_color,
            alpha: 0.4,
        });

        // ── Shoulders ────────────────────────────────────────────────────
        if let Some(shoulder) = &self.shoulder_left {
            shoulder.paint(rect, ctx);
        }
        if let Some(shoulder) = &self.shoulder_right {
            shoulder.paint(rect, ctx);
        }

        let cy = rect.y + rect.height / 2.0;

        // ── Action buttons ───────────────────────────────────────────────
        let action_reserves_space = self.show_actions && self.chrome.action_button_visibility > 0.0;
        let left_safe = if action_reserves_space { 44.0 } else { 0.0 };
        let right_safe = if action_reserves_space { rect.width - 44.0 } else { rect.width };

        // ── Headline ─────────────────────────────────────────────────────
        if !self.headline.is_empty() {
            let head_max = (right_safe - left_safe - 32.0).max(0.0);
            if head_max > 0.0 {
                let head_x = rect.x + 16.0 + left_safe;
                ctx.primitives.push(VisualPrimitive::Text {
                    origin: Point { x: head_x, y: cy - 8.0 },
                    max_width: head_max,
                    text: self.headline.clone(),
                    color: if self.headline_emphasized { Color::WHITE } else { self.text_color },
                    size: 14,
                    weight: if self.headline_emphasized { FontWeight::Semibold } else { FontWeight::Normal },
                    alignment: TextAlignment::Left,
                    alpha: 1.0,
                });
            }
        }

        // ── Active count with marquee ────────────────────────────────────
        let active_count_positive = self.active_count.parse::<usize>().unwrap_or(0) > 0;
        let active_color = if active_count_positive {
            Color::rgb(104, 222, 145)
        } else {
            self.text_color
        };

        let mut right_x = rect.x + rect.width - 16.0;
        if !self.total_count.is_empty() {
            let tw = self.total_count.chars().count() as f64 * 8.0;
            right_x -= tw;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: right_x, y: cy - 8.0 },
                max_width: tw + 4.0,
                text: self.total_count.clone(),
                color: self.dim_text_color,
                size: 14,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Right,
                alpha: 1.0,
            });
        }
        if !self.active_count.is_empty() && !self.total_count.is_empty() {
            right_x -= 8.0;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: right_x - 4.0, y: cy - 8.0 },
                max_width: 12.0,
                text: "/".to_string(),
                color: self.dim_text_color,
                size: 14,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Right,
                alpha: 0.5,
            });
            right_x -= 8.0;
        }
        if !self.active_count.is_empty() {
            let aw = self.active_count.chars().count() as f64 * 8.0;

            // Marquee: draw current count (scrolls up) and next count (scrolls in from below)
            ctx.primitives.push(VisualPrimitive::ClipStart { frame: Rect { x: right_x - aw, y: cy - 10.0, width: aw + 4.0, height: 20.0 } });

            let scroll_offset = self.active_count_scroll * 20.0;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: right_x, y: cy - 8.0 - scroll_offset },
                max_width: aw + 4.0,
                text: self.active_count.clone(),
                color: active_color,
                size: 14,
                weight: FontWeight::Semibold,
                alignment: TextAlignment::Right,
                alpha: 1.0 - self.active_count_scroll,
            });

            if let Some(next) = &self.active_count_next {
                ctx.primitives.push(VisualPrimitive::Text {
                    origin: Point { x: right_x, y: cy - 8.0 + 20.0 - scroll_offset },
                    max_width: aw + 4.0,
                    text: next.clone(),
                    color: active_color,
                    size: 14,
                    weight: FontWeight::Semibold,
                    alignment: TextAlignment::Right,
                    alpha: self.active_count_scroll,
                });
            }

            ctx.primitives.push(VisualPrimitive::ClipEnd);
        }

        // ── Action button icons ──────────────────────────────────────────
        if self.show_actions {
            let av = self.chrome.action_button_visibility;
            let scale = 0.82 + (1.0 - 0.82) * av;
            let opacity = av;
            let offset_y = -4.0 * (1.0 - av);

            // Settings button (left side)
            let sx = rect.x + 12.0;
            let sy = cy - 10.0 + offset_y;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: sx, y: sy },
                max_width: 24.0,
                text: SETTINGS_ICON.to_string(),
                color: if self.debug_mode { Color::rgb(102, 222, 145) } else { Color::rgb(245, 247, 252) },
                size: (16.0 * scale) as i32,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: opacity,
            });

            // Quit button (right side)
            let qx = rect.x + rect.width - 28.0;
            let qy = cy - 10.0 + offset_y;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: qx, y: qy },
                max_width: 24.0,
                text: QUIT_ICON.to_string(),
                color: Color::rgb(255, 82, 82),
                size: (16.0 * scale) as i32,
                weight: FontWeight::Bold,
                alignment: TextAlignment::Left,
                alpha: opacity,
            });
        }

        // ── Mascot ───────────────────────────────────────────────────────
        if let Some(mascot) = &self.mascot {
            mascot.paint(rect, ctx);
        }
    }
}

impl Default for CompactBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_bar_paints_background_and_text() {
        let bar = CompactBar::new();
        let rect = Rect { x: 0.0, y: 0.0, width: 400.0, height: 48.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        bar.paint(rect, &mut ctx);
        assert!(primitives.len() >= 2); // fill + border
    }

    #[test]
    fn compact_bar_with_headline_and_counts() {
        let mut bar = CompactBar::new();
        bar.headline = "Reef".to_string();
        bar.headline_emphasized = true;
        bar.active_count = "3".to_string();
        bar.total_count = "5".to_string();
        let rect = Rect { x: 0.0, y: 0.0, width: 400.0, height: 48.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        bar.paint(rect, &mut ctx);
        assert!(primitives.len() > 4);
    }

    #[test]
    fn compact_bar_marquee_renders_current_and_next() {
        let mut bar = CompactBar::new();
        bar.active_count = "3".to_string();
        bar.active_count_next = Some("4".to_string());
        bar.active_count_scroll = 0.5;
        bar.total_count = "5".to_string();
        let rect = Rect { x: 0.0, y: 0.0, width: 400.0, height: 48.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        bar.paint(rect, &mut ctx);
        assert!(primitives.len() > 6);
    }

    #[test]
    fn compact_bar_with_action_buttons() {
        let mut bar = CompactBar::new();
        bar.show_actions = true;
        bar.chrome = ChromeVisibility::expanded();
        let rect = Rect { x: 0.0, y: 0.0, width: 400.0, height: 48.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        bar.paint(rect, &mut ctx);
        // background (2) + settings icon + quit icon = 4
        assert!(primitives.len() >= 4);
    }

    #[test]
    fn chrome_visibility_interpolation() {
        let compact = ChromeVisibility::compact();
        let expanded = ChromeVisibility::expanded();
        let mid = ChromeVisibility::interpolate(&compact, &expanded, 0.5);
        assert!(mid.separator_visibility > 0.4 && mid.separator_visibility < 0.6);
        assert!(mid.shoulder_progress > 0.4 && mid.shoulder_progress < 0.6);
        assert!(mid.action_button_visibility > 0.4 && mid.action_button_visibility < 0.6);
    }
}
