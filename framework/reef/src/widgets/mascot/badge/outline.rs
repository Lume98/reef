use crate::core::{
    color::Color,
    geometry::{Rect, Size},
};
use crate::draw::primitive::DrawPrimitive;
use crate::layout::Constraints;
use crate::theme::mascot as theme;
use crate::view::widget_host::{PaintContext, Widget};

/// Badge outline layer.
#[derive(Clone)]
pub struct CompletionBadgeOutline {
    pub frame: Rect,
    pub radius: f64,
    pub alpha: f64,
}

impl Widget for CompletionBadgeOutline {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.frame.width,
            height: self.frame.height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        ctx.primitives.push(DrawPrimitive::RoundRect {
            frame: self.frame,
            radius: self.radius,
            color: Color::from(theme::BADGE_OUTLINE),
            alpha: self.alpha,
        });
    }
}
