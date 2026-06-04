use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;
use reef_view::widget_host::{PaintContext, Widget};

/// Mascot body blob with fill and stroke.
pub struct MascotDot {
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub corner_radius: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub fill_color: Color,
    pub stroke_color: Color,
    pub stroke_width: f64,
    pub alpha: f64,
}

impl MascotDot {
    pub fn frame(&self) -> Rect {
        let w = self.radius * 2.0 * self.scale_x;
        let h = self.radius * 2.0 * self.scale_y;
        Rect {
            x: self.center_x - w / 2.0,
            y: self.center_y - h / 2.0,
            width: w,
            height: h,
        }
    }
}

impl Widget for MascotDot {
    fn measure(&self, constraints: Constraints) -> Size {
        let frame = self.frame();
        constraints.constrain(Size {
            width: frame.width,
            height: frame.height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        ctx.primitives.push(VisualPrimitive::StrokedRoundRect {
            frame: rect,
            radius: self.corner_radius,
            fill: self.fill_color,
            stroke: self.stroke_color,
            stroke_width: self.stroke_width,
            alpha: self.alpha,
        });
    }
}
