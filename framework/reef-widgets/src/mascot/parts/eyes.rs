use reef_core::geometry::{Rect, Size};
use reef_layout::Constraints;
use reef_view::widget_host::{PaintContext, Widget};

use super::eye::MascotEye;

/// Mascot eye pair.
#[derive(Clone)]
pub struct MascotEyes {
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub eye_color: reef_core::color::Color,
    pub alpha: f64,
}

impl Widget for MascotEyes {
    fn measure(&self, constraints: Constraints) -> Size {
        let d = self.radius * 2.0;
        constraints.constrain(Size {
            width: d,
            height: d,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        let cx = self.center_x + self.offset_x;
        let cy = self.center_y + self.offset_y;
        let r = self.radius;
        let eye_r = r * 0.12;
        let eye_y = cy - r * 0.2 * self.scale_y;

        MascotEye {
            frame: Rect {
                x: cx - r * 0.3 * self.scale_x - eye_r,
                y: eye_y - eye_r,
                width: eye_r * 2.0,
                height: eye_r * 2.0,
            },
            color: self.eye_color,
            alpha: self.alpha,
        }
        .paint(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            ctx,
        );

        MascotEye {
            frame: Rect {
                x: cx + r * 0.3 * self.scale_x - eye_r,
                y: eye_y - eye_r,
                width: eye_r * 2.0,
                height: eye_r * 2.0,
            },
            color: self.eye_color,
            alpha: self.alpha,
        }
        .paint(
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
