use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

/// Message bubble shown above the mascot (3 dots + rounded rect).
#[derive(Clone)]
pub struct MessageBubble {
    pub center_x: f64,
    pub bottom_y: f64,
    pub bubble_width: f64,
    pub bubble_height: f64,
    pub dot_radius: f64,
    pub fill_color: Color,
    pub alpha: f64,
}

impl MessageBubble {
    pub fn new(center_x: f64, bottom_y: f64) -> Self {
        Self {
            center_x,
            bottom_y,
            bubble_width: 48.0,
            bubble_height: 22.0,
            dot_radius: 3.0,
            fill_color: Color::rgb(47, 47, 52),
            alpha: 1.0,
        }
    }
}

impl Widget for MessageBubble {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.bubble_width,
            height: self.bubble_height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        let bx = self.center_x - self.bubble_width / 2.0;
        let by = self.bottom_y - self.bubble_height;

        // Bubble background
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: Rect {
                x: bx,
                y: by,
                width: self.bubble_width,
                height: self.bubble_height,
            },
            radius: 8.0,
            color: self.fill_color,
            alpha: self.alpha,
        });

        // Three dots
        let dot_y = by + self.bubble_height / 2.0;
        let dot_spacing = 10.0;
        let start_x = self.center_x - dot_spacing;

        for i in 0..3 {
            let dx = start_x + i as f64 * dot_spacing;
            ctx.primitives.push(VisualPrimitive::Ellipse {
                frame: Rect {
                    x: dx - self.dot_radius,
                    y: dot_y - self.dot_radius,
                    width: self.dot_radius * 2.0,
                    height: self.dot_radius * 2.0,
                },
                color: Color::rgb(140, 150, 170),
                alpha: self.alpha * (0.3 + 0.7 * ((i as f64 / 2.0).min(1.0))),
            });
        }
    }
}
