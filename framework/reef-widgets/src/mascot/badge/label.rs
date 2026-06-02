use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

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
        ctx.primitives.push(VisualPrimitive::Text {
            origin: Point {
                x: self.frame.x + 4.0,
                y: self.frame.y + 3.0,
            },
            max_width: self.frame.width - 8.0,
            text: label,
            color: Color::rgb(102, 222, 145),
            size: 11,
            weight: FontWeight::Semibold,
            alignment: TextAlignment::Center,
            alpha: self.alpha,
        });
    }
}
