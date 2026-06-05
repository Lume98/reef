use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_draw::primitive::{DrawPrimitive, TextAlignment, TextWeight};
use reef_layout::Constraints;
use reef_view::widget_host::{PaintContext, Widget};

use crate::mascot::MascotPose;
use reef_theme::mascot as theme;

/// Pose-driven mascot expression layer.
#[derive(Clone)]
pub struct MascotExpression {
    pub pose: MascotPose,
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub alpha: f64,
}

impl Widget for MascotExpression {
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

        match self.pose {
            MascotPose::Approval => {
                let mouth_y = cy + r * 0.1;
                ctx.primitives.push(DrawPrimitive::Ellipse {
                    frame: Rect {
                        x: cx - r * 0.2,
                        y: mouth_y - eye_r,
                        width: r * 0.4,
                        height: eye_r * 1.5,
                    },
                    color: Color::from(theme::SHADOW),
                    alpha: self.alpha,
                });
            }
            MascotPose::WakeAngry => {
                let mouth_w = r * 0.3;
                ctx.primitives.push(DrawPrimitive::Rect {
                    frame: Rect {
                        x: cx - mouth_w / 2.0,
                        y: cy + r * 0.15,
                        width: mouth_w,
                        height: 2.0,
                    },
                    color: Color::from(theme::MOUTH),
                    alpha: self.alpha,
                });
            }
            _ => {}
        }

        if self.pose == MascotPose::Sleepy {
            ctx.primitives.push(DrawPrimitive::Text {
                origin: Point {
                    x: cx - 3.0 * self.scale_x,
                    y: cy - r * self.scale_y - 14.0,
                },
                max_width: 20.0,
                text: "Z".to_string(),
                color: Color::from(theme::EYE_LID),
                size: 12,
                weight: TextWeight::Bold,
                alignment: TextAlignment::Center,
                alpha: 0.7,
            });
        }
    }
}
