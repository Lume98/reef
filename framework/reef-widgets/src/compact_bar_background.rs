use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

/// Compact bar background layer: fill plus border.
#[derive(Clone)]
pub struct CompactBarBackground {
    pub fill_color: Color,
    pub border_color: Color,
    pub radius: f64,
}

impl Widget for CompactBarBackground {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: constraints.max_height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
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
    }
}
