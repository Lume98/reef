use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

use crate::mascot::MascotWidget;

/// Collapsed mini-pill bar.
pub struct CompactBar {
    pub headline: String,
    pub headline_emphasized: bool,
    pub active_count: String,
    pub total_count: String,
    pub completion_count: usize,
    pub mascot: Option<MascotWidget>,
    pub show_actions: bool,
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
            total_count: String::new(),
            completion_count: 0,
            mascot: None,
            show_actions: false,
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
        // Background pill
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

        let cy = rect.y + rect.height / 2.0;

        // Headline
        if !self.headline.is_empty() {
            ctx.primitives.push(VisualPrimitive::Text {
                origin: reef_core::geometry::Point { x: rect.x + 16.0, y: cy - 8.0 },
                max_width: rect.width * 0.5,
                text: self.headline.clone(),
                color: if self.headline_emphasized { Color::WHITE } else { self.text_color },
                size: 14,
                weight: if self.headline_emphasized { FontWeight::Semibold } else { FontWeight::Normal },
                alignment: TextAlignment::Left,
                alpha: 1.0,
            });
        }

        // Active / Total counts on the right
        let mut right_x = rect.x + rect.width - 16.0;
        if !self.total_count.is_empty() {
            let tw = self.total_count.chars().count() as f64 * 8.0;
            right_x -= tw;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: reef_core::geometry::Point { x: right_x, y: cy - 8.0 },
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
                origin: reef_core::geometry::Point { x: right_x - 4.0, y: cy - 8.0 },
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
            right_x -= aw;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: reef_core::geometry::Point { x: right_x, y: cy - 8.0 },
                max_width: aw + 4.0,
                text: self.active_count.clone(),
                color: self.text_color,
                size: 14,
                weight: FontWeight::Semibold,
                alignment: TextAlignment::Right,
                alpha: 1.0,
            });
        }

        // Mascot
        if let Some(mascot) = &self.mascot {
            mascot.paint(rect, ctx);
        }
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
}
