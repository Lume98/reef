use reef_view::widget_host::{PaintContext, Widget};
use reef_core::geometry::{Point, Rect, Size};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

use crate::card::{badge_background_color, badge_foreground_color, Badge, CardStyle};

/// Badge strip on the card header.
#[derive(Clone)]
pub struct CardBadges {
    pub style: CardStyle,
    pub badges: Vec<Badge>,
    pub content_translate_y: f64,
    pub content_alpha: f64,
    pub pad_x: f64,
}

impl Widget for CardBadges {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: constraints.max_height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        if self.badges.is_empty() {
            return;
        }

        let title_y = rect.y + rect.height - 24.0 + self.content_translate_y;
        let mut right = rect.x + rect.width - self.pad_x;
        for badge in self.badges.iter().rev() {
            let bg = badge_background_color(self.style, badge);
            let fg = badge_foreground_color(self.style, badge);
            let w = 64.0;
            let bx = right - w;
            let by = title_y - 3.0;
            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: Rect {
                    x: bx,
                    y: by,
                    width: w,
                    height: 22.0,
                },
                radius: 11.0,
                color: bg,
                alpha: self.content_alpha,
            });
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point {
                    x: bx + 7.0,
                    y: by + 2.0,
                },
                max_width: w - 14.0,
                text: badge.text.clone(),
                color: fg,
                size: 10,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Center,
                alpha: self.content_alpha,
            });
            right = bx - 6.0;
        }
    }
}
