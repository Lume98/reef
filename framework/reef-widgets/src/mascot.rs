use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

/// Mascot pose enum.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MascotPose {
    Hidden,
    Idle,
    Running,
    Approval,
    Question,
    Complete,
    Sleepy,
}

/// Animated mascot widget. Composes body dot, eyes, mouth, and optional decorations.
pub struct MascotWidget {
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub pose: MascotPose,
    pub elapsed_ms: u128,
    pub fill_color: Color,
    pub stroke_color: Color,
    pub eye_color: Color,
    pub alpha: f64,
}

impl MascotWidget {
    pub fn new(center_x: f64, center_y: f64, radius: f64) -> Self {
        Self {
            center_x,
            center_y,
            radius,
            pose: MascotPose::Idle,
            elapsed_ms: 0,
            fill_color: Color::rgb(60, 65, 80),
            stroke_color: Color::rgb(220, 160, 60),
            eye_color: Color::rgb(220, 225, 240),
            alpha: 1.0,
        }
    }

    pub fn pose(mut self, pose: MascotPose) -> Self {
        self.pose = pose;
        self
    }
}

impl Widget for MascotWidget {
    fn measure(&self, constraints: Constraints) -> Size {
        let d = self.radius * 2.0;
        constraints.constrain(Size { width: d, height: d })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        if self.pose == MascotPose::Hidden {
            return;
        }

        let r = self.radius;
        let cx = self.center_x;
        let cy = self.center_y;

        // Body
        ctx.primitives.push(VisualPrimitive::StrokedRoundRect {
            frame: Rect {
                x: cx - r,
                y: cy - r,
                width: r * 2.0,
                height: r * 2.0,
            },
            radius: r * 0.6,
            fill: self.fill_color,
            stroke: self.stroke_color,
            stroke_width: 2.0,
            alpha: self.alpha,
        });

        // Eyes
        let eye_y = cy - r * 0.2;
        let eye_r = r * 0.12;
        ctx.primitives.push(VisualPrimitive::Ellipse {
            frame: Rect {
                x: cx - r * 0.3 - eye_r,
                y: eye_y - eye_r,
                width: eye_r * 2.0,
                height: eye_r * 2.0,
            },
            color: self.eye_color,
            alpha: self.alpha,
        });
        ctx.primitives.push(VisualPrimitive::Ellipse {
            frame: Rect {
                x: cx + r * 0.3 - eye_r,
                y: eye_y - eye_r,
                width: eye_r * 2.0,
                height: eye_r * 2.0,
            },
            color: self.eye_color,
            alpha: self.alpha,
        });

        // Sleepy label
        if self.pose == MascotPose::Sleepy {
            ctx.primitives.push(VisualPrimitive::Text {
                origin: reef_core::geometry::Point { x: cx - 3.0, y: cy - r - 14.0 },
                max_width: 20.0,
                text: "Z".to_string(),
                color: Color::rgb(160, 170, 190),
                size: 12,
                weight: FontWeight::Bold,
                alignment: TextAlignment::Center,
                alpha: 0.7,
            });
        }
    }
}
