use reef_app::widget_host::{dispatch_to_child, EventContext, PaintContext, Widget};
use reef_core::{
    event::Event,
    geometry::{point_in_rect, Point, Rect, Size},
};
use reef_layout::{column, Constraints};
use reef_render::primitive::VisualPrimitive;

/// Vertical layout container. React-style: holds children as struct field.
///
/// ```ignore
/// Column {
///     children: vec![label_a, label_b],
///     gap: 8.0,
/// }
/// ```
pub struct Column {
    pub children: Vec<Box<dyn Widget>>,
    pub gap: f64,
}

impl Column {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self { children, gap: 0.0 }
    }

    pub fn gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }
}

impl Widget for Column {
    fn measure(&self, constraints: Constraints) -> Size {
        let sizes: Vec<Size> = self.children.iter().map(|c| {
            let child_max_h = constraints.max_height;
            c.measure(Constraints {
                min_width: constraints.min_width,
                max_width: constraints.max_width,
                min_height: 0.0,
                max_height: child_max_h,
            })
        }).collect();
        let total = column::column_total_size(&sizes, self.gap);
        constraints.constrain(total)
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        let sizes: Vec<Size> = self.children.iter().map(|c| {
            c.measure(Constraints {
                min_width: 0.0,
                max_width: rect.width,
                min_height: 0.0,
                max_height: rect.height,
            })
        }).collect();
        let child_rects = column::arrange_column(rect, &sizes, self.gap);
        for (child, child_rect) in self.children.iter().zip(child_rects.iter()) {
            ctx.primitives.push(VisualPrimitive::ClipStart { frame: *child_rect });
            child.paint(*child_rect, ctx);
            ctx.primitives.push(VisualPrimitive::ClipEnd);
        }
    }

    fn handle_event(&mut self, event: &Event, rect: Rect, ctx: &mut EventContext) -> bool {
        if let Event::Pointer(pe) = event {
            // Compute child rects for hit testing
            let sizes: Vec<Size> = self.children.iter().map(|c| {
                c.measure(Constraints {
                    min_width: 0.0,
                    max_width: rect.width,
                    min_height: 0.0,
                    max_height: rect.height,
                })
            }).collect();
            let child_rects = column::arrange_column(rect, &sizes, self.gap);
            // Dispatch to children in reverse (top-most first)
            for i in (0..self.children.len()).rev() {
                let child_rect = child_rects[i];
                if point_in_rect(pe.position, child_rect) {
                    if dispatch_to_child(
                        &mut *self.children[i],
                        event,
                        pe.position,
                        child_rect,
                        ctx,
                    ) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::label::Label;

    #[test]
    fn column_measures_stacked_children() {
        let col = Column::new(vec![
            Box::new(Label::new("Line 1")),
            Box::new(Label::new("Line 2")),
        ]).gap(8.0);
        let size = col.measure(Constraints::loose(Size { width: 800.0, height: 600.0 }));
        assert!(size.width > 0.0);
        assert!(size.height > 48.0); // Two lines + gap
    }

    #[test]
    fn column_paints_all_children() {
        let col = Column::new(vec![
            Box::new(Label::new("A")),
            Box::new(Label::new("B")),
        ]);
        let rect = Rect { x: 0.0, y: 0.0, width: 200.0, height: 100.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        col.paint(rect, &mut ctx);
        // Each child gets ClipStart + Text + ClipEnd = 6 primitives
        assert_eq!(primitives.len(), 6);
    }
}
