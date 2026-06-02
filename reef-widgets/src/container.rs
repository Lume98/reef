use std::any::Any;

use reef_app::widget_host::{MeasureContext, PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

pub struct Container {
    pub color: Color,
    pub radius: f64,
    pub border_color: Option<Color>,
    pub border_width: f64,
    pub padding: f64,
    pub min_size: Size,
}

impl Container {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            radius: 0.0,
            border_color: None,
            border_width: 0.0,
            padding: 0.0,
            min_size: Size {
                width: 0.0,
                height: 0.0,
            },
        }
    }

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn border(mut self, color: Color, width: f64) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self
    }

    pub fn padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }

    pub fn min_size(mut self, size: Size) -> Self {
        self.min_size = size;
        self
    }
}

impl Widget for Container {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn measure(&self, constraints: Constraints, _ctx: &mut MeasureContext) -> Size {
        let doubled_padding = self.padding * 2.0;
        let inner_constraints = Constraints {
            min_width: (constraints.min_width - doubled_padding).max(0.0),
            max_width: (constraints.max_width - doubled_padding).max(0.0),
            min_height: (constraints.min_height - doubled_padding).max(0.0),
            max_height: (constraints.max_height - doubled_padding).max(0.0),
        };
        let _ = inner_constraints;
        constraints.constrain(Size {
            width: self.min_size.width.max(constraints.min_width),
            height: self.min_size.height.max(constraints.min_height),
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: rect,
            radius: self.radius,
            color: self.color,
            alpha: 1.0,
        });
        if let Some(border_color) = self.border_color {
            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: rect,
                radius: self.radius,
                color: border_color,
                alpha: 1.0,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_app::widget_host::{PaintContext, Widget};
    use reef_core::geometry::Size;
    use reef_layout::Constraints;

    #[test]
    fn container_meets_minimum_size() {
        let container = Container::new(Color::rgb(18, 18, 22)).min_size(Size {
            width: 200.0,
            height: 100.0,
        });
        let constraints = Constraints::loose(Size {
            width: 800.0,
            height: 600.0,
        });
        let mut ctx = MeasureContext { children: &[] };
        let size = container.measure(constraints, &mut ctx);
        assert_eq!(size.width, 200.0);
        assert_eq!(size.height, 100.0);
    }

    #[test]
    fn container_paints_round_rect() {
        let container = Container::new(Color::rgb(18, 18, 22)).radius(12.0);
        let rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 100.0,
        };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext {
            primitives: &mut primitives,
        };
        container.paint(rect, &mut ctx);
        assert_eq!(primitives.len(), 1);
    }
}
