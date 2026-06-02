use reef_app::widget_host::{dispatch_to_child, EventContext, PaintContext, Widget};
use reef_core::{
    event::Event,
    geometry::{point_in_rect, Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

/// Z-ordered stacking container. Children overlap with configurable overhang.
/// Used for the expanded card stack.
pub struct Stack {
    pub children: Vec<Box<dyn Widget>>,
    pub gap: f64,
    pub overhang: f64,
}

impl Stack {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            children,
            gap: 0.0,
            overhang: 0.0,
        }
    }

    pub fn gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    pub fn overhang(mut self, overhang: f64) -> Self {
        self.overhang = overhang;
        self
    }

    /// Arrange children bottom-up: last child at bottom, first at top.
    fn arrange(&self, rect: Rect, sizes: &[Size]) -> Vec<Rect> {
        let mut rects = vec![Rect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }; sizes.len()];
        let mut y = rect.y + rect.height;
        for i in (0..sizes.len()).rev() {
            let h = sizes[i].height;
            y -= h;
            rects[i] = Rect {
                x: rect.x,
                y,
                width: sizes[i].width.max(rect.width),
                height: h,
            };
            if i > 0 {
                y -= self.gap;
                // Overhang: next card peeks above by this amount
                y += self.overhang;
            }
        }
        rects
    }
}

impl Widget for Stack {
    fn measure(&self, constraints: Constraints) -> Size {
        let sizes: Vec<Size> = self.children.iter().map(|c| {
            c.measure(Constraints {
                min_width: constraints.min_width,
                max_width: constraints.max_width,
                min_height: 0.0,
                max_height: constraints.max_height,
            })
        }).collect();
        let total_height: f64 = sizes.iter().map(|s| s.height).sum::<f64>()
            + self.gap * sizes.len().saturating_sub(1) as f64;
        let max_width = sizes.iter().map(|s| s.width).fold(0.0f64, f64::max);
        constraints.constrain(Size { width: max_width, height: total_height })
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
        let child_rects = self.arrange(rect, &sizes);
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
            let child_rects = self.arrange(rect, &sizes);
            for i in (0..self.children.len()).rev() {
                let child_rect = child_rects[i];
                if point_in_rect(pe.position, child_rect) {
                    if dispatch_to_child(&mut *self.children[i], event, pe.position, child_rect, ctx) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
