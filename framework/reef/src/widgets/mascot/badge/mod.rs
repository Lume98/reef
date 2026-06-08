use crate::core::geometry::{Rect, Size};
use crate::layout::Constraints;
use crate::theme::mascot as theme;
use crate::view::widget_host::{PaintContext, Widget};

mod label;
mod outline;

pub use label::CompletionBadgeLabel;
pub use outline::CompletionBadgeOutline;

/// Completion badge shown on the mascot (outline + fill + label).
#[derive(Clone)]
pub struct CompletionBadge {
    pub center_x: f64,
    pub center_y: f64,
    pub count: usize,
    pub badge_width: f64,
    pub badge_height: f64,
    pub alpha: f64,
}

impl CompletionBadge {
    pub fn new(center_x: f64, center_y: f64, count: usize) -> Self {
        Self {
            center_x,
            center_y,
            count,
            badge_width: 36.0,
            badge_height: 18.0,
            alpha: 1.0,
        }
    }
}

impl Widget for CompletionBadge {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.badge_width,
            height: self.badge_height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        let bx = self.center_x - self.badge_width / 2.0;
        let by = self.center_y - self.badge_height / 2.0;
        let frame = Rect {
            x: bx,
            y: by,
            width: self.badge_width,
            height: self.badge_height,
        };

        CompletionBadgeOutline {
            frame,
            radius: self.badge_height / 2.0,
            alpha: self.alpha,
        }
        .paint(frame, ctx);

        ctx.primitives
            .push(crate::draw::primitive::DrawPrimitive::RoundRect {
                frame: Rect {
                    x: frame.x + 1.0,
                    y: frame.y + 1.0,
                    width: frame.width - 2.0,
                    height: frame.height - 2.0,
                },
                radius: (self.badge_height / 2.0) - 1.0,
                color: crate::core::color::Color::from(theme::BADGE_FILL),
                alpha: self.alpha,
            });

        CompletionBadgeLabel {
            frame,
            count: self.count,
            alpha: self.alpha,
        }
        .paint(frame, ctx);
    }
}
