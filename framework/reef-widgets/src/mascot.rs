use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

use crate::mascot_badge::CompletionBadge;
use crate::mascot_bubble::MessageBubble;

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
    pub fill_color: Color,
    pub stroke_color: Color,
    pub eye_color: Color,
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
            fill_color: Color::rgb(60, 65, 80),
            stroke_color: Color::rgb(220, 160, 60),
            eye_color: Color::rgb(220, 225, 240),
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
        constraints.constrain(Size { width: d, height: d })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        if self.pose == MascotPose::Hidden {
            return;
        }

        let r = self.radius;
        let cx = self.center_x + self.offset_x;
        let cy = self.center_y + self.offset_y;
        let sx = self.scale_x;
        let sy = self.scale_y;

        // ── Shadow ───────────────────────────────────────────────────────
        if self.shadow_opacity > 0.0 && self.shadow_radius > 0.0 {
            let sr = self.shadow_radius + r;
            ctx.primitives.push(VisualPrimitive::Ellipse {
                frame: Rect {
                    x: cx - sr * sx,
                    y: cy + r * 0.3,
                    width: sr * 2.0 * sx,
                    height: sr * 0.6,
                },
                color: Color::rgba(0, 0, 0, (self.shadow_opacity * 100.0) as u8),
                alpha: self.shadow_opacity * 0.3,
            });
        }

        // ── Body ─────────────────────────────────────────────────────────
        let body_frame = Rect {
            x: cx - r * sx,
            y: cy - r * sy,
            width: r * 2.0 * sx,
            height: r * 2.0 * sy,
        };

        ctx.primitives.push(VisualPrimitive::StrokedRoundRect {
            frame: body_frame,
            radius: r * 0.6,
            fill: self.fill_color,
            stroke: self.stroke_color,
            stroke_width: 2.0,
            alpha: self.alpha,
        });

        // ── Eyes ─────────────────────────────────────────────────────────
        let eye_y = cy - r * 0.2 * sy;
        let eye_r = r * 0.12;

        ctx.primitives.push(VisualPrimitive::Ellipse {
            frame: Rect {
                x: cx - r * 0.3 * sx - eye_r,
                y: eye_y - eye_r,
                width: eye_r * 2.0,
                height: eye_r * 2.0,
            },
            color: self.eye_color,
            alpha: self.alpha,
        });
        ctx.primitives.push(VisualPrimitive::Ellipse {
            frame: Rect {
                x: cx + r * 0.3 * sx - eye_r,
                y: eye_y - eye_r,
                width: eye_r * 2.0,
                height: eye_r * 2.0,
            },
            color: self.eye_color,
            alpha: self.alpha,
        });

        // ── Mouth ────────────────────────────────────────────────────────
        match self.pose {
            MascotPose::Approval => {
                // Open mouth
                let mouth_y = cy + r * 0.1;
                ctx.primitives.push(VisualPrimitive::Ellipse {
                    frame: Rect {
                        x: cx - r * 0.2,
                        y: mouth_y - eye_r,
                        width: r * 0.4,
                        height: eye_r * 1.5,
                    },
                    color: Color::rgb(30, 30, 35),
                    alpha: self.alpha,
                });
            }
            MascotPose::WakeAngry => {
                // Angry mouth (horizontal line)
                let mouth_w = r * 0.3;
                ctx.primitives.push(VisualPrimitive::Rect {
                    frame: Rect {
                        x: cx - mouth_w / 2.0,
                        y: cy + r * 0.15,
                        width: mouth_w,
                        height: 2.0,
                    },
                    color: Color::rgb(255, 130, 100),
                    alpha: self.alpha,
                });
            }
            _ => {}
        }

        // ── Sleepy label ─────────────────────────────────────────────────
        if self.pose == MascotPose::Sleepy {
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: cx - 3.0, y: cy - r * sy - 14.0 },
                max_width: 20.0,
                text: "Z".to_string(),
                color: Color::rgb(160, 170, 190),
                size: 12,
                weight: FontWeight::Bold,
                alignment: TextAlignment::Center,
                alpha: 0.7,
            });
        }

        // ── Message bubble ───────────────────────────────────────────────
        if let Some(bubble) = &self.message_bubble {
            bubble.paint(Rect { x: cx - 24.0, y: cy - r * sy - 30.0, width: 48.0, height: 22.0 }, ctx);
        }

        // ── Completion badge ─────────────────────────────────────────────
        if let Some(badge) = &self.completion_badge {
            badge.paint(Rect { x: cx - 18.0, y: cy - r * sy - 10.0, width: 36.0, height: 18.0 }, ctx);
        }
    }
}
