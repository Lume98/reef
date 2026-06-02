use reef_app::widget_host::{PaintContext, Widget};
use reef_core::geometry::{Rect, Size};
use reef_layout::Constraints;

use crate::mascot_badge::CompletionBadge;
use crate::mascot_body::MascotBody;
use crate::mascot_bubble::MessageBubble;
use crate::mascot_expression::MascotExpression;
use crate::mascot_eyes::MascotEyes;
use crate::mascot_shadow::MascotShadow;

/// Mascot pose enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MascotPose {
    Hidden,
    Idle,
    Running,
    Approval,
    Question,
    MessageBubble,
    Complete,
    Sleepy,
    WakeAngry,
}

/// Animated mascot widget with body, eyes, shadow, and optional decorations.
#[derive(Clone)]
pub struct MascotWidget {
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub pose: MascotPose,
    pub elapsed_ms: u128,
    pub fill_color: reef_core::color::Color,
    pub stroke_color: reef_core::color::Color,
    pub eye_color: reef_core::color::Color,
    pub alpha: f64,
    /// Motion offsets for animation
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    /// Shadow
    pub shadow_opacity: f64,
    pub shadow_radius: f64,
    /// Optional completion badge
    pub completion_badge: Option<CompletionBadge>,
    /// Optional message bubble
    pub message_bubble: Option<MessageBubble>,
}

impl MascotWidget {
    pub fn new(center_x: f64, center_y: f64, radius: f64) -> Self {
        Self {
            center_x,
            center_y,
            radius,
            pose: MascotPose::Idle,
            elapsed_ms: 0,
            fill_color: reef_core::color::Color::rgb(60, 65, 80),
            stroke_color: reef_core::color::Color::rgb(220, 160, 60),
            eye_color: reef_core::color::Color::rgb(220, 225, 240),
            alpha: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            shadow_opacity: 0.0,
            shadow_radius: 0.0,
            completion_badge: None,
            message_bubble: None,
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
        constraints.constrain(Size {
            width: d,
            height: d,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        if self.pose == MascotPose::Hidden {
            return;
        }

        MascotShadow {
            center_x: self.center_x,
            center_y: self.center_y,
            radius: self.radius,
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            scale_x: self.scale_x,
            shadow_opacity: self.shadow_opacity,
            shadow_radius: self.shadow_radius,
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

        MascotBody {
            center_x: self.center_x,
            center_y: self.center_y,
            radius: self.radius,
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
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

        MascotEyes {
            center_x: self.center_x,
            center_y: self.center_y,
            radius: self.radius,
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            eye_color: self.eye_color,
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

        MascotExpression {
            pose: self.pose,
            center_x: self.center_x,
            center_y: self.center_y,
            radius: self.radius,
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
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

        if let Some(bubble) = &self.message_bubble {
            bubble.paint(
                Rect {
                    x: self.center_x - 24.0,
                    y: self.center_y + self.offset_y - self.radius * self.scale_y - 30.0,
                    width: 48.0,
                    height: 22.0,
                },
                ctx,
            );
        }

        if let Some(badge) = &self.completion_badge {
            badge.paint(
                Rect {
                    x: self.center_x - 18.0,
                    y: self.center_y + self.offset_y - self.radius * self.scale_y - 10.0,
                    width: 36.0,
                    height: 18.0,
                },
                ctx,
            );
        }
    }
}
