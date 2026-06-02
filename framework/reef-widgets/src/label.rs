use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

pub struct Label {
    pub text: String,
    pub color: Color,
    pub font_size: i32,
    pub weight: FontWeight,
    pub alignment: TextAlignment,
    pub max_width: Option<f64>,
    pub line_height: Option<f64>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: Color::WHITE,
            font_size: 14,
            weight: FontWeight::Normal,
            alignment: TextAlignment::Left,
            max_width: None,
            line_height: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn font_size(mut self, size: i32) -> Self {
        self.font_size = size;
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn max_width(mut self, width: f64) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn line_height(mut self, height: f64) -> Self {
        self.line_height = Some(height);
        self
    }
}

fn estimate_text_width(text: &str, font_size: i32) -> f64 {
    let char_width = font_size as f64 * 0.6;
    text.chars().count() as f64 * char_width
}

fn default_line_height(font_size: i32) -> f64 {
    if font_size >= 13 {
        24.0
    } else {
        font_size as f64 + 8.0
    }
}

impl Widget for Label {
    fn measure(&self, constraints: Constraints) -> Size {
        let line_count = self.text.lines().count().max(1);
        let text_width = estimate_text_width(&self.text, self.font_size);
        let max_w = self.max_width.unwrap_or(constraints.max_width);
        let width = text_width.min(max_w);
        let lh = self.line_height.unwrap_or_else(|| default_line_height(self.font_size));
        let height = lh * line_count as f64;
        constraints.constrain(Size { width, height })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        let origin = match self.alignment {
            TextAlignment::Center => Point {
                x: rect.x,
                y: rect.y,
            },
            TextAlignment::Right => Point {
                x: rect.x + rect.width
                    - estimate_text_width(&self.text, self.font_size).min(rect.width),
                y: rect.y,
            },
            TextAlignment::Left => Point {
                x: rect.x,
                y: rect.y,
            },
        };
        let max_width = self.max_width.unwrap_or(rect.width);
        ctx.primitives.push(VisualPrimitive::Text {
            origin,
            max_width,
            text: self.text.clone(),
            color: self.color,
            size: self.font_size,
            weight: self.weight,
            alignment: self.alignment,
            alpha: 1.0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_layout::Constraints;

    #[test]
    fn label_measures_text_size() {
        let label = Label::new("Hello");
        let constraints = Constraints::loose(Size {
            width: 800.0,
            height: 600.0,
        });
        let size = label.measure(constraints);
        assert!(size.width > 0.0);
        assert!(size.height > 0.0);
    }

    #[test]
    fn label_paints_text_primitive() {
        let label = Label::new("Hello").color(Color::WHITE);
        let rect = Rect {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 30.0,
        };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext {
            primitives: &mut primitives,
        };
        label.paint(rect, &mut ctx);
        assert_eq!(primitives.len(), 1);
        assert!(matches!(
            &primitives[0],
            VisualPrimitive::Text { text, .. } if text == "Hello"
        ));
    }

    #[test]
    fn label_right_alignment_shifts_origin() {
        let label = Label::new("Hi").alignment(TextAlignment::Right);
        let rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 30.0,
        };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext {
            primitives: &mut primitives,
        };
        label.paint(rect, &mut ctx);
        if let VisualPrimitive::Text { origin, .. } = &primitives[0] {
            assert!(origin.x > 0.0, "Right alignment should shift origin right");
        }
    }
}
