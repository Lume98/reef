use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_draw::primitive::DrawPrimitive;
use reef_layout::Constraints;
use reef_theme::mascot as theme;
use reef_view::widget_host::{PaintContext, Widget};

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
