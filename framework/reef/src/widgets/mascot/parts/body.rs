use crate::core::geometry::{Rect, Size};
use crate::layout::Constraints;
use crate::view::widget_host::{PaintContext, Widget};

use super::dot::MascotDot;

/// Mascot body layer.
#[derive(Clone)]
pub struct MascotBody {
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub fill_color: crate::core::color::Color,
    pub stroke_color: crate::core::color::Color,
    pub alpha: f64,
}

impl Widget for MascotBody {
    fn measure(&self, constraints: Constraints) -> Size {
        let d = self.radius * 2.0;
        constraints.constrain(Size {
            width: d,
            height: d,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        let body = MascotDot {
            center_x: self.center_x + self.offset_x,
            center_y: self.center_y + self.offset_y,
            radius: self.radius,
            corner_radius: self.radius * 0.6,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: 2.0,
            alpha: self.alpha,
        };
        body.paint(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            ctx,
        );
    }
}
