use crate::core::{
    color::Color,
    geometry::{Rect, Size},
};
use crate::draw::primitive::DrawPrimitive;
use crate::layout::Constraints;
use crate::theme::mascot as theme;
use crate::view::widget_host::{PaintContext, Widget};

/// Bubble dots layer.
#[derive(Clone)]
pub struct MessageBubbleDots {
    pub center_x: f64,
    pub bottom_y: f64,
    pub bubble_height: f64,
    pub dot_radius: f64,
    pub alpha: f64,
}

impl Widget for MessageBubbleDots {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.dot_radius * 6.0 + 20.0,
            height: self.bubble_height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        let dot_y = self.bottom_y - self.bubble_height / 2.0;
        let dot_spacing = 10.0;
        let start_x = self.center_x - dot_spacing;

        for i in 0..3 {
            let dx = start_x + i as f64 * dot_spacing;
            ctx.primitives.push(DrawPrimitive::Ellipse {
                frame: Rect {
                    x: dx - self.dot_radius,
                    y: dot_y - self.dot_radius,
                    width: self.dot_radius * 2.0,
                    height: self.dot_radius * 2.0,
                },
                color: Color::from(theme::BUBBLE_DOT),
                alpha: self.alpha * (0.3 + 0.7 * ((i as f64 / 2.0).min(1.0))),
            });
        }
    }
}
