use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_draw::primitive::{DrawPrimitive, TextAlignment, TextWeight};
use reef_layout::Constraints;
use reef_theme::compact_bar as theme;
use reef_view::widget_host::{PaintContext, Widget};

/// Headline text on the compact bar.
#[derive(Clone)]
pub struct CompactBarHeadline {
    pub text: String,
    pub emphasized: bool,
    pub text_color: Color,
    pub leading_reserve: f64,
    pub trailing_reserve: f64,
}

impl Widget for CompactBarHeadline {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: constraints.max_height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        if self.text.is_empty() {
            return;
        }

        let cy = rect.y + rect.height / 2.0;
        let max_width = (rect.width
            - self.leading_reserve
            - self.trailing_reserve
            - theme::HEADLINE_SIDE_RESERVE)
            .max(0.0);
        if max_width <= 0.0 {
            return;
        }

        ctx.primitives.push(DrawPrimitive::Text {
            frame: Rect {
                x: rect.x + theme::HEADLINE_LEFT_INSET + self.leading_reserve,
                y: cy - 8.0,
                width: max_width,
                height: 24.0,
            },
            text: self.text.clone(),
            color: if self.emphasized {
                Color::WHITE
            } else {
                self.text_color
            },
            size: 14,
            weight: if self.emphasized {
                TextWeight::Semibold
            } else {
                TextWeight::Normal
            },
            alignment: TextAlignment::Left,
            alpha: 1.0,
        });
    }
}
