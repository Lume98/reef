use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_draw::primitive::{DrawPrimitive, TextAlignment, TextWeight};
use reef_layout::Constraints;
use reef_theme::mascot as theme;
use reef_view::widget_host::{PaintContext, Widget};

/// Badge count label layer.
#[derive(Clone)]
pub struct CompletionBadgeLabel {
    pub frame: Rect,
    pub count: usize,
    pub alpha: f64,
}

impl Widget for CompletionBadgeLabel {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.frame.width,
            height: self.frame.height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        let label = self.count.to_string();
        ctx.primitives.push(DrawPrimitive::Text {
            origin: Point {
                x: self.frame.x + 4.0,
                y: self.frame.y + 3.0,
            },
            max_width: self.frame.width - 8.0,
            text: label,
            color: Color::from(theme::BADGE_LABEL),
            size: 11,
            weight: TextWeight::Semibold,
            alignment: TextAlignment::Center,
            alpha: self.alpha,
        });
    }
}
