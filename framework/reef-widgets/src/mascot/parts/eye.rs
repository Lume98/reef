use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_draw::primitive::DrawPrimitive;
use reef_layout::Constraints;
use reef_view::widget_host::{PaintContext, Widget};

/// Mascot eye ellipse.
pub struct MascotEye {
    pub frame: Rect,
    pub color: Color,
    pub alpha: f64,
}

impl Widget for MascotEye {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.frame.width,
            height: self.frame.height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        ctx.primitives.push(DrawPrimitive::Ellipse {
            frame: self.frame,
            color: self.color,
            alpha: self.alpha,
        });
    }
}
