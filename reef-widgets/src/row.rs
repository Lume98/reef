use reef_app::widget_host::{dispatch_to_child, EventContext, PaintContext, Widget};
use reef_core::{
    event::Event,
    geometry::{point_in_rect, Point, Rect, Size},
};
use reef_layout::{row, Constraints};
use reef_render::primitive::VisualPrimitive;

/// Horizontal layout container.
pub struct Row {
    pub children: Vec<Box<dyn Widget>>,
    pub gap: f64,
}

impl Row {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self { children, gap: 0.0 }
    }

    pub fn gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }
}

impl Widget for Row {
    fn measure(&self, constraints: Constraints) -> Size {
        let sizes: Vec<Size> = self.children.iter().map(|c| {
            c.measure(Constraints {
                min_width: 0.0,
                max_width: constraints.max_width,
                min_height: constraints.min_height,
                max_height: constraints.max_height,
            })
        }).collect();
        let total = row::row_total_size(&sizes, self.gap);
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
        let child_rects = row::arrange_row(rect, &sizes, self.gap);
        for (child, child_rect) in self.children.iter().zip(child_rects.iter()) {
            ctx.primitives.push(VisualPrimitive::ClipStart { frame: *child_rect });
            child.paint(*child_rect, ctx);
            ctx.primitives.push(VisualPrimitive::ClipEnd);
        }
    }

    fn handle_event(&mut self, event: &Event, rect: Rect, ctx: &mut EventContext) -> bool {
        if let Event::Pointer(pe) = event {
            let sizes: Vec<Size> = self.children.iter().map(|c| {
                c.measure(Constraints {
                    min_width: 0.0,
                    max_width: rect.width,
                    min_height: 0.0,
                    max_height: rect.height,
                })
            }).collect();
            let child_rects = row::arrange_row(rect, &sizes, self.gap);
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
    fn row_measures_side_by_side_children() {
        let r = Row::new(vec![
            Box::new(Label::new("A")),
            Box::new(Label::new("BB")),
        ]).gap(10.0);
        let size = r.measure(Constraints::loose(Size { width: 800.0, height: 600.0 }));
        assert!(size.width > 0.0);
        assert!(size.height > 0.0);
    }
}
