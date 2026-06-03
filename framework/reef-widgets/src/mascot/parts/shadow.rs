use reef_view::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

/// Mascot shadow layer.
#[derive(Clone)]
pub struct MascotShadow {
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale_x: f64,
    pub shadow_opacity: f64,
    pub shadow_radius: f64,
}

impl Widget for MascotShadow {
    fn measure(&self, constraints: Constraints) -> Size {
        let d = self.radius * 2.0;
        constraints.constrain(Size {
            width: d,
            height: d,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        if self.shadow_opacity <= 0.0 || self.shadow_radius <= 0.0 {
            return;
        }

        let r = self.radius;
        let cx = self.center_x + self.offset_x;
        let cy = self.center_y + self.offset_y;
        let sr = self.shadow_radius + r;

        ctx.primitives.push(VisualPrimitive::Ellipse {
            frame: Rect {
                x: cx - sr * self.scale_x,
                y: cy + r * 0.3,
                width: sr * 2.0 * self.scale_x,
                height: sr * 0.6,
            },
            color: Color::rgba(0, 0, 0, (self.shadow_opacity * 100.0) as u8),
            alpha: self.shadow_opacity * 0.3,
        });
    }
}
