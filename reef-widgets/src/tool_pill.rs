use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, VisualPrimitive};

/// Bordered pill showing tool name + optional description.
pub struct ToolPill {
    pub tool_name: String,
    pub description: Option<String>,
    pub border_color: Color,
    pub fill_color: Color,
    pub name_color: Color,
    pub desc_color: Color,
    pub font_size: i32,
    pub radius: f64,
    pub padding_h: f64,
    pub alpha: f64,
}

impl ToolPill {
    pub fn new(tool_name: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            description: None,
            border_color: Color::rgb(60, 60, 75),
            fill_color: Color::rgb(30, 30, 40),
            name_color: Color::rgb(180, 190, 210),
            desc_color: Color::rgb(120, 130, 150),
            font_size: 12,
            radius: 6.0,
            padding_h: 8.0,
            alpha: 1.0,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn colors(mut self, border: Color, fill: Color, name: Color, desc: Color) -> Self {
        self.border_color = border;
        self.fill_color = fill;
        self.name_color = name;
        self.desc_color = desc;
        self
    }

    fn measure_text_width(text: &str, font_size: i32) -> f64 {
        text.chars().count() as f64 * font_size as f64 * 0.6
    }
}

impl Widget for ToolPill {
    fn measure(&self, constraints: Constraints) -> Size {
        let name_w = Self::measure_text_width(&self.tool_name, self.font_size);
        let desc_w = self.description.as_ref().map_or(0.0, |d| {
            8.0 + Self::measure_text_width(d, self.font_size)
        });
        let width = self.padding_h * 2.0 + name_w + desc_w;
        let height = self.font_size as f64 + 8.0;
        constraints.constrain(Size { width, height })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        // Border
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: rect,
            radius: self.radius,
            color: self.border_color,
            alpha: self.alpha,
        });
        // Fill (inset by 1px)
        let fill_rect = Rect {
            x: rect.x + 1.0,
            y: rect.y + 1.0,
            width: (rect.width - 2.0).max(0.0),
            height: (rect.height - 2.0).max(0.0),
        };
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: fill_rect,
            radius: (self.radius - 1.0).max(0.0),
            color: self.fill_color,
            alpha: self.alpha,
        });
        // Tool name text
        let text_y = rect.y + 4.0;
        let name_w = Self::measure_text_width(&self.tool_name, self.font_size);
        ctx.primitives.push(VisualPrimitive::Text {
            origin: reef_core::geometry::Point { x: rect.x + self.padding_h, y: text_y },
            max_width: name_w + 10.0,
            text: self.tool_name.clone(),
            color: self.name_color,
            size: self.font_size,
            weight: FontWeight::Semibold,
            alignment: reef_render::primitive::TextAlignment::Left,
            alpha: self.alpha,
        });
        // Description text
        if let Some(desc) = &self.description {
            ctx.primitives.push(VisualPrimitive::Text {
                origin: reef_core::geometry::Point {
                    x: rect.x + self.padding_h + name_w + 8.0,
                    y: text_y,
                },
                max_width: rect.width,
                text: desc.clone(),
                color: self.desc_color,
                size: self.font_size,
                weight: FontWeight::Normal,
                alignment: reef_render::primitive::TextAlignment::Left,
                alpha: self.alpha,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_pill_paints_border_fill_and_name() {
        let pill = ToolPill::new("Bash");
        let rect = Rect { x: 0.0, y: 0.0, width: 80.0, height: 24.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        pill.paint(rect, &mut ctx);
        assert!(primitives.len() >= 3); // border + fill + name text
    }

    #[test]
    fn tool_pill_with_description_paints_extra_text() {
        let pill = ToolPill::new("Bash").description("running tests");
        let rect = Rect { x: 0.0, y: 0.0, width: 200.0, height: 24.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        pill.paint(rect, &mut ctx);
        assert!(primitives.len() >= 4); // border + fill + name + desc
    }
}
