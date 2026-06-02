use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

/// Bubble background layer.
#[derive(Clone)]
pub struct MessageBubbleBackground {
    pub frame: Rect,
    pub fill_color: Color,
    pub alpha: f64,
}

impl Widget for MessageBubbleBackground {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.frame.width,
            height: self.frame.height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: self.frame,
            radius: 8.0,
            color: self.fill_color,
            alpha: self.alpha,
        });
    }
}
