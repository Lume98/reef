use crate::core::{
    color::Color,
    geometry::{Rect, Size},
};
use crate::draw::primitive::DrawPrimitive;
use crate::layout::Constraints;
use crate::theme::shell as theme;
use crate::view::widget_host::{PaintContext, Widget};

/// Expanded panel shell: rounded rect border + separator line.
#[derive(Clone)]
pub struct ExpandedShell {
    pub fill_color: Color,
    pub border_color: Color,
    pub separator_color: Color,
    pub radius: f64,
    pub separator_y: Option<f64>,
    pub alpha: f64,
}

impl ExpandedShell {
    pub fn new() -> Self {
        Self {
            fill_color: Color::from(theme::FILL),
            border_color: Color::from(theme::BORDER),
            separator_color: Color::from(theme::SEPARATOR),
            radius: theme::RADIUS,
            separator_y: None,
            alpha: 1.0,
        }
    }

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn separator_y(mut self, y: f64) -> Self {
        self.separator_y = Some(y);
        self
    }
}

impl Widget for ExpandedShell {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: constraints.max_height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        // Fill
        ctx.primitives.push(DrawPrimitive::RoundRect {
            frame: rect,
            radius: self.radius,
            color: self.fill_color,
            alpha: self.alpha,
        });
        // Border
        ctx.primitives.push(DrawPrimitive::RoundRect {
            frame: rect,
            radius: self.radius,
            color: self.border_color,
            alpha: 0.5 * self.alpha,
        });
        // Separator line
        if let Some(y) = self.separator_y {
            ctx.primitives.push(DrawPrimitive::Rect {
                frame: Rect {
                    x: rect.x + 12.0,
                    y: rect.y + y,
                    width: rect.width - 24.0,
                    height: 1.0,
                },
                color: self.separator_color,
                alpha: 0.5 * self.alpha,
            });
        }
    }
}
