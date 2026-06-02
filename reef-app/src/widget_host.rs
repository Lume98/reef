use reef_core::{
    event::Event,
    geometry::{point_in_rect, Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{VisualPrimitive, VisualPlan};

/// A visual component in the reef UI tree.
///
/// React-style design: each widget is a struct with props as fields.
/// Container widgets hold child widgets as struct fields and recurse
/// in their measure/paint/handle_event implementations.
pub trait Widget {
    /// Compute the desired size within the given constraints.
    fn measure(&self, constraints: Constraints) -> Size;

    /// Draw into the primitive list. Container widgets recurse into children,
    /// computing child rects based on measure results and their layout strategy.
    fn paint(&self, rect: Rect, ctx: &mut PaintContext);

    /// Handle an event. Return true if consumed (stops bubbling).
    /// Container widgets should forward events to hit children first,
    /// then handle locally if not consumed.
    fn handle_event(
        &mut self,
        _event: &Event,
        _rect: Rect,
        _ctx: &mut EventContext,
    ) -> bool {
        false
    }
}

pub struct PaintContext<'a> {
    pub primitives: &'a mut Vec<VisualPrimitive>,
}

pub struct EventContext<'a> {
    pub dirty: &'a mut bool,
}

/// Manages the root widget and coordinates measure/paint cycles.
pub struct WidgetHost {
    root: Option<Box<dyn Widget>>,
    dirty: bool,
    size: Size,
}

impl WidgetHost {
    pub fn new() -> Self {
        Self {
            root: None,
            dirty: true,
            size: Size {
                width: 800.0,
                height: 600.0,
            },
        }
    }

    pub fn set_root(&mut self, widget: Box<dyn Widget>) {
        self.root = Some(widget);
        self.dirty = true;
    }

    pub fn set_size(&mut self, size: Size) {
        if self.size != size {
            self.size = size;
            self.dirty = true;
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn root(&self) -> Option<&dyn Widget> {
        self.root.as_deref()
    }

    pub fn root_mut(&mut self) -> Option<&mut Box<dyn Widget>> {
        self.root.as_mut()
    }

    /// Measure then paint the widget tree into a VisualPlan.
    pub fn render(&mut self) -> VisualPlan {
        let mut plan = VisualPlan::new();
        if let Some(root) = &self.root {
            let root_rect = Rect {
                x: 0.0,
                y: 0.0,
                width: self.size.width,
                height: self.size.height,
            };
            let constraints = Constraints::loose(self.size);

            let _measured = root.measure(constraints);

            let mut primitives = Vec::new();
            let mut ctx = PaintContext {
                primitives: &mut primitives,
            };
            root.paint(root_rect, &mut ctx);
            plan.primitives = primitives;
        }
        self.dirty = false;
        plan
    }

    /// Dispatch an event into the widget tree.
    /// The root widget's handle_event is called, which should recurse
    /// into children for hit testing and bubbling.
    pub fn dispatch_event(&mut self, event: &Event, position: Point) -> bool {
        let root = match &mut self.root {
            Some(r) => r,
            None => return false,
        };
        let root_rect = Rect {
            x: 0.0,
            y: 0.0,
            width: self.size.width,
            height: self.size.height,
        };
        if !point_in_rect(position, root_rect) {
            return false;
        }
        let mut dirty = false;
        let mut ctx = EventContext { dirty: &mut dirty };
        let consumed = root.handle_event(event, root_rect, &mut ctx);
        if dirty {
            self.dirty = true;
        }
        consumed
    }
}

impl Default for WidgetHost {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for containers: dispatch an event to a child widget.
/// Returns true if the child consumed the event.
pub fn dispatch_to_child(
    child: &mut dyn Widget,
    event: &Event,
    position: Point,
    child_rect: Rect,
    ctx: &mut EventContext,
) -> bool {
    if !point_in_rect(position, child_rect) {
        return false;
    }
    child.handle_event(event, child_rect, ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::color::Color;

    struct TestWidget {
        color: Color,
        size: Size,
    }

    impl Widget for TestWidget {
        fn measure(&self, _constraints: Constraints) -> Size {
            self.size
        }
        fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: rect,
                radius: 12.0,
                color: self.color,
                alpha: 1.0,
            });
        }
    }

    #[test]
    fn widget_host_renders_root_widget() {
        let mut host = WidgetHost::new();
        host.set_size(Size {
            width: 200.0,
            height: 100.0,
        });
        host.set_root(Box::new(TestWidget {
            color: Color::rgb(18, 18, 22),
            size: Size {
                width: 200.0,
                height: 100.0,
            },
        }));

        let plan = host.render();
        assert!(!plan.hidden);
        assert_eq!(plan.primitives.len(), 1);
    }

    #[test]
    fn widget_host_empty_render() {
        let mut host = WidgetHost::new();
        let plan = host.render();
        assert!(!plan.hidden);
        assert!(plan.primitives.is_empty());
    }

    #[test]
    fn dispatch_event_outside_root_returns_false() {
        use reef_core::event::{PointerButton, PointerEvent, PointerEventKind};

        let mut host = WidgetHost::new();
        host.set_size(Size {
            width: 200.0,
            height: 100.0,
        });
        host.set_root(Box::new(TestWidget {
            color: Color::rgb(18, 18, 22),
            size: Size {
                width: 200.0,
                height: 100.0,
            },
        }));

        let event = Event::Pointer(PointerEvent {
            kind: PointerEventKind::Move,
            position: Point { x: 500.0, y: 500.0 },
            button: None,
        });
        assert!(!host.dispatch_event(&event, Point { x: 500.0, y: 500.0 }));
    }
}
