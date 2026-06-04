use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};
use reef_theme::card as card_theme;
use reef_theme::compact_bar as theme;
use reef_view::widget_host::{PaintContext, Widget};

fn estimate_text_width(text: &str) -> f64 {
    text.chars().count() as f64 * 8.0
}

/// Right-aligned counters on the compact bar.
#[derive(Clone)]
pub struct CompactBarCounts {
    pub active_count: String,
    pub active_count_next: Option<String>,
    pub active_count_scroll: f64,
    pub total_count: String,
    pub text_color: Color,
    pub dim_text_color: Color,
}

impl Widget for CompactBarCounts {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: constraints.max_height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        let cy = rect.y + rect.height / 2.0;
        let active_count_positive = self.active_count.parse::<usize>().unwrap_or(0) > 0;
        let active_color = if active_count_positive {
            Color::from(card_theme::TEXT_BODY_EMPHASIZED)
        } else {
            self.text_color
        };

        let mut right_x = rect.x + rect.width - theme::COUNTS_RIGHT_INSET;
        if !self.total_count.is_empty() {
            let tw = estimate_text_width(&self.total_count);
            right_x -= tw;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point {
                    x: right_x,
                    y: cy - 8.0,
                },
                max_width: tw + 4.0,
                text: self.total_count.clone(),
                color: self.dim_text_color,
                size: 14,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Right,
                alpha: 1.0,
            });
        }

        if !self.active_count.is_empty() && !self.total_count.is_empty() {
            right_x -= 8.0;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point {
                    x: right_x - 4.0,
                    y: cy - 8.0,
                },
                max_width: 12.0,
                text: "/".to_string(),
                color: self.dim_text_color,
                size: 14,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Right,
                alpha: 0.5,
            });
            right_x -= 8.0;
        }

        if self.active_count.is_empty() {
            return;
        }

        let aw = estimate_text_width(&self.active_count);
        ctx.primitives.push(VisualPrimitive::ClipStart {
            frame: Rect {
                x: right_x - aw,
                y: cy - 10.0,
                width: aw + 4.0,
                height: 20.0,
            },
        });

        let scroll_offset = self.active_count_scroll * 20.0;
        ctx.primitives.push(VisualPrimitive::Text {
            origin: Point {
                x: right_x,
                y: cy - 8.0 - scroll_offset,
            },
            max_width: aw + 4.0,
            text: self.active_count.clone(),
            color: active_color,
            size: 14,
            weight: FontWeight::Semibold,
            alignment: TextAlignment::Right,
            alpha: 1.0 - self.active_count_scroll,
        });

        if let Some(next) = &self.active_count_next {
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point {
                    x: right_x,
                    y: cy - 8.0 + 20.0 - scroll_offset,
                },
                max_width: aw + 4.0,
                text: next.clone(),
                color: active_color,
                size: 14,
                weight: FontWeight::Semibold,
                alignment: TextAlignment::Right,
                alpha: self.active_count_scroll,
            });
        }

        ctx.primitives.push(VisualPrimitive::ClipEnd);
    }
}
