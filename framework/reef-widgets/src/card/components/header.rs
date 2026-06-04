use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};
use reef_view::widget_host::{PaintContext, Widget};

/// Standard card title and subtitle block.
#[derive(Clone)]
pub struct CardHeader {
    pub title: String,
    pub subtitle: Option<String>,
    pub title_color: Color,
    pub compact: bool,
    pub content_translate_y: f64,
    pub content_alpha: f64,
    pub pad_x: f64,
}

impl Widget for CardHeader {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: constraints.max_height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        if !self.title.is_empty() {
            let base_y = if self.compact {
                rect.y + (rect.height - 20.0) / 2.0
            } else {
                rect.y + rect.height - 24.0
            };
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point {
                    x: rect.x + self.pad_x,
                    y: base_y + self.content_translate_y,
                },
                max_width: rect.width - self.pad_x * 2.0,
                text: self.title.clone(),
                color: self.title_color,
                size: 12,
                weight: FontWeight::Semibold,
                alignment: TextAlignment::Left,
                alpha: self.content_alpha,
            });
        }

        if let Some(sub) = &self.subtitle {
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point {
                    x: rect.x + self.pad_x,
                    y: rect.y + rect.height - 40.0 + self.content_translate_y,
                },
                max_width: rect.width - self.pad_x * 2.0,
                text: sub.clone(),
                color: Color::rgb(171, 179, 194),
                size: 9,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: self.content_alpha,
            });
        }
    }
}
