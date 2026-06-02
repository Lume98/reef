use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, VisualPrimitive};

/// Pill-shaped text badge (status, source, action hint).
pub struct Badge {
    pub text: String,
    pub background_color: Color,
    pub text_color: Color,
    pub font_size: i32,
    pub font_weight: FontWeight,
    pub radius: f64,
    pub padding_h: f64,
    pub padding_v: f64,
    pub alpha: f64,
}

impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            background_color: Color::rgb(40, 40, 50),
            text_color: Color::WHITE,
            font_size: 11,
            font_weight: FontWeight::Normal,
            radius: 8.0,
            padding_h: 6.0,
            padding_v: 2.0,
            alpha: 1.0,
        }
    }

    pub fn colors(mut self, bg: Color, fg: Color) -> Self {
        self.background_color = bg;
        self.text_color = fg;
        self
    }

    pub fn font_size(mut self, size: i32) -> Self {
        self.font_size = size;
        self
    }

    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha;
        self
    }

    fn text_width(&self) -> f64 {
        self.text.chars().count() as f64 * self.font_size as f64 * 0.6
    }
}

impl Widget for Badge {
    fn measure(&self, constraints: Constraints) -> Size {
        let width = self.text_width() + self.padding_h * 2.0;
        let height = self.font_size as f64 + self.padding_v * 2.0;
        constraints.constrain(Size { width, height })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: rect,
            radius: self.radius,
            color: self.background_color,
            alpha: self.alpha,
        });
        ctx.primitives.push(VisualPrimitive::Text {
            origin: reef_core::geometry::Point {
                x: rect.x + self.padding_h,
                y: rect.y + self.padding_v,
            },
            max_width: rect.width,
            text: self.text.clone(),
            color: self.text_color,
            size: self.font_size,
            weight: self.font_weight,
            alignment: reef_render::primitive::TextAlignment::Left,
            alpha: self.alpha,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_measures_pill_size() {
        let badge = Badge::new("Running");
        let size = badge.measure(Constraints::loose(Size { width: 800.0, height: 600.0 }));
        assert!(size.width > 12.0);
        assert!(size.height > 11.0);
    }

    #[test]
    fn badge_paints_background_and_text() {
        let badge = Badge::new("OK");
        let rect = Rect { x: 0.0, y: 0.0, width: 40.0, height: 20.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        badge.paint(rect, &mut ctx);
        assert_eq!(primitives.len(), 2);
    }
}
