use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_draw::primitive::{DrawPrimitive, TextAlignment, TextWeight};
use reef_layout::Constraints;
use reef_view::widget_host::{PaintContext, Widget};

use crate::card::{
    settings_row_border_color, settings_row_fill_color, settings_value_badge_bg,
    settings_value_badge_fg, SettingsRow,
};
use reef_theme::card as theme;

/// Settings card surface: title, version badge, and setting rows.
#[derive(Clone)]
pub struct CardSettingsPanel {
    pub title: String,
    pub subtitle: Option<String>,
    pub settings_rows: Vec<SettingsRow>,
    pub content_translate_y: f64,
    pub content_alpha: f64,
}

impl Widget for CardSettingsPanel {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: constraints.max_height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        if !self.title.is_empty() {
            let y = rect.y + rect.height - 24.0 + self.content_translate_y;
            ctx.primitives.push(DrawPrimitive::Text {
                origin: Point {
                    x: rect.x + theme::HEADER_PAD_X,
                    y,
                },
                max_width: rect.width - theme::HEADER_PAD_X * 2.0,
                text: self.title.clone(),
                color: Color::from(theme::TEXT_TITLE),
                size: 12,
                weight: TextWeight::Semibold,
                alignment: TextAlignment::Left,
                alpha: self.content_alpha,
            });
        }

        if let Some(version) = &self.subtitle {
            let title_y = rect.y + rect.height - 24.0 + self.content_translate_y;
            let w = theme::BADGE_WIDTH;
            let bx = rect.x + rect.width - theme::HEADER_PAD_X - w;
            let by = title_y - 3.0;
            ctx.primitives.push(DrawPrimitive::RoundRect {
                frame: Rect {
                    x: bx,
                    y: by,
                    width: w,
                    height: 22.0,
                },
                radius: theme::BADGE_RADIUS,
                color: Color::from(theme::BADGE_BG_DEFAULT),
                alpha: self.content_alpha,
            });
            ctx.primitives.push(DrawPrimitive::Text {
                origin: Point {
                    x: bx + 7.0,
                    y: by + 2.0,
                },
                max_width: w - 14.0,
                text: version.clone(),
                color: Color::from(theme::BADGE_FG_DEFAULT),
                size: 10,
                weight: TextWeight::Normal,
                alignment: TextAlignment::Center,
                alpha: self.content_alpha,
            });
        }

        let row_h = 32.0;
        let row_gap = 2.0;
        let pad_x = theme::HEADER_PAD_X;
        for (i, row) in self.settings_rows.iter().enumerate() {
            let ry = rect.y + 8.0 + (self.settings_rows.len() - 1 - i) as f64 * (row_h + row_gap);
            let row_frame = Rect {
                x: rect.x + pad_x,
                y: ry,
                width: rect.width - pad_x * 2.0,
                height: row_h,
            };

            ctx.primitives.push(DrawPrimitive::RoundRect {
                frame: row_frame,
                radius: theme::SETTINGS_ROW_RADIUS,
                color: settings_row_border_color(row.active),
                alpha: self.content_alpha,
            });
            let inner = Rect {
                x: row_frame.x + 1.0,
                y: row_frame.y + 1.0,
                width: row_frame.width - 2.0,
                height: row_frame.height - 2.0,
            };
            ctx.primitives.push(DrawPrimitive::RoundRect {
                frame: inner,
                radius: theme::SETTINGS_ROW_RADIUS - 1.0,
                color: settings_row_fill_color(row.active),
                alpha: self.content_alpha,
            });
            ctx.primitives.push(DrawPrimitive::Text {
                origin: Point {
                    x: inner.x + 11.0,
                    y: inner.y + (inner.height - 16.0) / 2.0,
                },
                max_width: inner.width - 70.0,
                text: row.title.clone(),
                color: Color::from(theme::TEXT_TITLE),
                size: 11,
                weight: TextWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: self.content_alpha,
            });

            let badge_w = 44.0;
            let badge_h = 18.0;
            let badge_x = inner.x + inner.width - badge_w - 9.0;
            let badge_y = inner.y + (inner.height - badge_h) / 2.0;
            ctx.primitives.push(DrawPrimitive::RoundRect {
                frame: Rect {
                    x: badge_x,
                    y: badge_y,
                    width: badge_w,
                    height: badge_h,
                },
                radius: theme::SETTINGS_VALUE_BADGE_RADIUS,
                color: settings_value_badge_bg(row.active),
                alpha: self.content_alpha,
            });
            ctx.primitives.push(DrawPrimitive::Text {
                origin: Point {
                    x: badge_x + 9.0,
                    y: badge_y + 2.0,
                },
                max_width: badge_w - 18.0,
                text: row.value.clone(),
                color: settings_value_badge_fg(row.active),
                size: 10,
                weight: TextWeight::Normal,
                alignment: TextAlignment::Center,
                alpha: self.content_alpha,
            });
        }
    }
}
