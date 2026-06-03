use reef_view::widget_host::{PaintContext, Widget};
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
    pub child: Option<Box<dyn Widget>>,
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
            child: None,
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

    pub fn child(mut self, widget: Box<dyn Widget>) -> Self {
        self.child = Some(widget);
        self
    }
}

impl Widget for Container {
    fn measure(&self, constraints: Constraints) -> Size {
        let doubled = self.padding * 2.0;
        let inner = Constraints {
            min_width: (constraints.min_width - doubled).max(0.0),
            max_width: (constraints.max_width - doubled).max(0.0),
            min_height: (constraints.min_height - doubled).max(0.0),
            max_height: (constraints.max_height - doubled).max(0.0),
        };

        let child_size = match &self.child {
            Some(child) => child.measure(inner),
            None => Size {
                width: 0.0,
                height: 0.0,
            },
        };

        let width = child_size.width + doubled;
        let height = child_size.height + doubled;
        let width = self.min_size.width.max(width);
        let height = self.min_size.height.max(height);
        constraints.constrain(Size { width, height })
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
                alpha: 0.6,
            });
        }

        if let Some(child) = &self.child {
            let child_rect = Rect {
                x: rect.x + self.padding,
                y: rect.y + self.padding,
                width: (rect.width - self.padding * 2.0).max(0.0),
                height: (rect.height - self.padding * 2.0).max(0.0),
            };
            ctx.primitives
                .push(VisualPrimitive::ClipStart { frame: child_rect });
            child.paint(child_rect, ctx);
            ctx.primitives.push(VisualPrimitive::ClipEnd);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let size = container.measure(constraints);
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

    #[test]
    fn container_with_child_measures_including_padding() {
        let container = Container::new(Color::BLACK)
            .padding(10.0)
            .child(Box::new(crate::base::Label::new("Hi")));
        let constraints = Constraints::loose(Size {
            width: 800.0,
            height: 600.0,
        });
        let size = container.measure(constraints);
        assert!(size.width > 20.0);
        assert!(size.height > 20.0);
    }

    #[test]
    fn container_with_child_paints_clip_and_child() {
        let container = Container::new(Color::BLACK)
            .padding(8.0)
            .child(Box::new(crate::base::Label::new("Hi")));
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
        // ClipStart + RoundRect + Child Text + ClipEnd
        assert!(primitives.len() >= 4);
    }
}
