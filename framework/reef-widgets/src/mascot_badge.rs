use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

/// Completion badge shown on the mascot (outline + fill + label).
#[derive(Clone)]
pub struct CompletionBadge {
    pub center_x: f64,
    pub center_y: f64,
    pub count: usize,
    pub badge_width: f64,
    pub badge_height: f64,
    pub alpha: f64,
}

impl CompletionBadge {
    pub fn new(center_x: f64, center_y: f64, count: usize) -> Self {
        Self {
            center_x,
            center_y,
            count,
            badge_width: 36.0,
            badge_height: 18.0,
            alpha: 1.0,
        }
    }
}

impl Widget for CompletionBadge {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.badge_width,
            height: self.badge_height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        let bx = self.center_x - self.badge_width / 2.0;
        let by = self.center_y - self.badge_height / 2.0;
        let frame = Rect {
            x: bx,
            y: by,
            width: self.badge_width,
            height: self.badge_height,
        };

        // Outline
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame,
            radius: self.badge_height / 2.0,
            color: Color::rgb(46, 79, 61),
            alpha: self.alpha,
        });

        // Fill
        let inset = 1.0;
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: Rect {
                x: frame.x + inset,
                y: frame.y + inset,
                width: frame.width - inset * 2.0,
                height: frame.height - inset * 2.0,
            },
            radius: (self.badge_height / 2.0) - inset,
            color: Color::rgb(37, 37, 41),
            alpha: self.alpha,
        });

        // Label
        let label = self.count.to_string();
        ctx.primitives.push(VisualPrimitive::Text {
            origin: Point {
                x: bx + 4.0,
                y: by + 3.0,
            },
            max_width: self.badge_width - 8.0,
            text: label,
            color: Color::rgb(102, 222, 145),
            size: 11,
            weight: FontWeight::Semibold,
            alignment: TextAlignment::Center,
            alpha: self.alpha,
        });
    }
}
