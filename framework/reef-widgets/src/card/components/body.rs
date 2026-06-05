use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_draw::primitive::{DrawPrimitive, TextAlignment, TextWeight};
use reef_layout::Constraints;
use reef_view::widget_host::{PaintContext, Widget};

use crate::card::{
    body_prefix_color, body_text_color, estimated_text_width, tool_tone_color, BodyLine, CardStyle,
    ToolPill,
};
use reef_theme::card as theme;

/// Card body content: text lines, tool pill, and action hint.
#[derive(Clone)]
pub struct CardBody {
    pub style: CardStyle,
    pub body_lines: Vec<BodyLine>,
    pub tool: Option<ToolPill>,
    pub action_hint: Option<String>,
    pub content_translate_y: f64,
    pub content_alpha: f64,
    pub pad_x: f64,
}

impl Widget for CardBody {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: constraints.max_height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        let action_hint_present = self.action_hint.is_some();
        let body_bottom = if action_hint_present { 36.0 } else { 10.0 };
        let mut y = rect.y + body_bottom + self.content_translate_y;

        // Preserve the semantic order of body lines as supplied by the card builder.
        for line in self.body_lines.iter() {
            let (prefix_color, text_color) = {
                let p = line.prefix.as_deref().unwrap_or_default();
                (
                    body_prefix_color(self.style, p),
                    body_text_color(self.style, line.role, line.prefix.as_deref()),
                )
            };
            let mut x = rect.x + self.pad_x;
            if let Some(prefix) = &line.prefix {
                let pw = prefix.chars().count() as f64 * 6.0;
                ctx.primitives.push(DrawPrimitive::Text {
                    origin: Point { x, y },
                    max_width: pw + 4.0,
                    text: prefix.clone(),
                    color: prefix_color,
                    size: 10,
                    weight: TextWeight::Normal,
                    alignment: TextAlignment::Left,
                    alpha: self.content_alpha,
                });
                x += 24.0;
            }
            ctx.primitives.push(DrawPrimitive::Text {
                origin: Point { x, y },
                max_width: rect.width - x + rect.x - self.pad_x,
                text: line.text.clone(),
                color: text_color,
                size: 10,
                weight: TextWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: self.content_alpha,
            });
            y += 16.0;
        }

        if let Some(tool) = &self.tool {
            let name_w = estimated_text_width(&tool.name, 9.0);
            let desc_w = tool
                .description
                .as_ref()
                .filter(|d| !d.trim().is_empty())
                .map(|d| estimated_text_width(d, 9.0) + 6.0)
                .unwrap_or(0.0);
            let pill_w = (name_w + desc_w + 14.0).max(36.0);
            let pill_h = 22.0;
            let pill_radius = 5.0;
            let px = rect.x + self.pad_x;
            let py = y;

            ctx.primitives.push(DrawPrimitive::RoundRect {
                frame: Rect {
                    x: px,
                    y: py,
                    width: pill_w,
                    height: pill_h,
                },
                radius: pill_radius,
                color: Color::from(theme::TOOL_PILL_BG),
                alpha: self.content_alpha,
            });
            ctx.primitives.push(DrawPrimitive::RoundRect {
                frame: Rect {
                    x: px,
                    y: py,
                    width: pill_w,
                    height: pill_h,
                },
                radius: pill_radius,
                color: Color::from(theme::TOOL_PILL_SHADOW),
                alpha: 0.4 * self.content_alpha,
            });
            ctx.primitives.push(DrawPrimitive::Text {
                origin: Point {
                    x: px + 7.0,
                    y: py + 5.0,
                },
                max_width: name_w,
                text: tool.name.clone(),
                color: tool_tone_color(&tool.name),
                size: 9,
                weight: TextWeight::Semibold,
                alignment: TextAlignment::Left,
                alpha: self.content_alpha,
            });
            if let Some(desc) = &tool.description {
                if !desc.trim().is_empty() {
                    let desc_x = px + 7.0 + name_w + 6.0;
                    ctx.primitives.push(DrawPrimitive::Text {
                        origin: Point {
                            x: desc_x,
                            y: py + 5.0,
                        },
                        max_width: desc_w,
                        text: desc.clone(),
                        color: Color::from(theme::TEXT_DETAIL),
                        size: 9,
                        weight: TextWeight::Normal,
                        alignment: TextAlignment::Left,
                        alpha: self.content_alpha,
                    });
                }
            }
        }

        if let Some(hint) = &self.action_hint {
            let hint_text = hint.split_whitespace().collect::<Vec<_>>().join(" ");
            if !hint_text.is_empty() {
                let hint_w = (estimated_text_width(&hint_text, 10.0) + 18.0).max(32.0);
                let hint_h = 18.0;
                let hint_radius = hint_h / 2.0;
                let hx = rect.x + self.pad_x;
                let hy = rect.y + 10.0;

                ctx.primitives.push(DrawPrimitive::RoundRect {
                    frame: Rect {
                        x: hx,
                        y: hy,
                        width: hint_w,
                        height: hint_h,
                    },
                    radius: hint_radius,
                    color: Color::from(theme::ACTION_HINT_BG),
                    alpha: self.content_alpha,
                });
                ctx.primitives.push(DrawPrimitive::Text {
                    origin: Point {
                        x: hx + 9.0,
                        y: hy + 4.0,
                    },
                    max_width: hint_w - 18.0,
                    text: hint_text,
                    color: Color::from(theme::ACTION_HINT_FG),
                    size: 10,
                    weight: TextWeight::Normal,
                    alignment: TextAlignment::Left,
                    alpha: self.content_alpha,
                });
            }
        }
    }
}
