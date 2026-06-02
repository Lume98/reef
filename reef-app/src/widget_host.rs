use reef_core::{
    event::Event,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{VisualPrimitive, VisualPlan};

use std::any::Any;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

pub trait Widget {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn measure(
        &self,
        constraints: Constraints,
        ctx: &mut MeasureContext,
    ) -> Size;

    fn paint(&self, rect: Rect, ctx: &mut PaintContext);

    fn handle_event(
        &mut self,
        _event: &Event,
        _rect: Rect,
        _ctx: &mut EventContext,
    ) -> bool {
        false
    }
}

pub struct MeasureContext<'a> {
    pub children: &'a [(WidgetId, Box<dyn Widget>)],
}

pub struct PaintContext<'a> {
    pub primitives: &'a mut Vec<VisualPrimitive>,
}

pub struct EventContext<'a> {
    pub dirty: &'a mut bool,
}

struct WidgetEntry {
    widget: Box<dyn Widget>,
    rect: Rect,
}

pub struct WidgetHost {
    widgets: HashMap<WidgetId, WidgetEntry>,
    root: Option<WidgetId>,
    next_id: u64,
    size: Size,
    dirty: bool,
}

impl WidgetHost {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            root: None,
            next_id: 1,
            dirty: true,
            size: Size {
                width: 800.0,
                height: 600.0,
            },
        }
    }

    pub fn set_root(&mut self, widget: Box<dyn Widget>) -> WidgetId {
        let id = self.allocate_id();
        self.root = Some(id);
        self.widgets.insert(
            id,
            WidgetEntry {
                widget,
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: self.size.width,
                    height: self.size.height,
                },
            },
        );
        self.dirty = true;
        id
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

    pub fn render(&mut self) -> VisualPlan {
        let mut plan = VisualPlan::new();
        if let Some(root_id) = self.root {
            if let Some(entry) = self.widgets.get(&root_id) {
                let root_rect = Rect {
                    x: 0.0,
                    y: 0.0,
                    width: self.size.width,
                    height: self.size.height,
                };
                let constraints = Constraints::loose(self.size);
                let _ = constraints;

                let mut primitives = Vec::new();
                let mut ctx = PaintContext {
                    primitives: &mut primitives,
                };
                entry.widget.paint(root_rect, &mut ctx);
                plan.primitives = primitives;
            }
        }
        self.dirty = false;
        plan
    }

    pub fn dispatch_event(&mut self, event: &Event, position: Point) -> bool {
        let root_id = match self.root {
            Some(id) => id,
            None => return false,
        };
        self.dispatch_event_to(event, position, root_id)
    }

    fn dispatch_event_to(
        &mut self,
        event: &Event,
        position: Point,
        target: WidgetId,
    ) -> bool {
        let rect = match self.widgets.get(&target) {
            Some(entry) => entry.rect,
            None => return false,
        };
        if !reef_core::geometry::point_in_rect(position, rect) {
            return false;
        }
        let mut dirty = false;
        {
            let entry = self.widgets.get_mut(&target).unwrap();
            let mut ctx = EventContext { dirty: &mut dirty };
            entry.widget.handle_event(event, rect, &mut ctx);
        }
        if dirty {
            self.dirty = true;
        }
        true
    }

    fn allocate_id(&mut self) -> WidgetId {
        let id = WidgetId(self.next_id);
        self.next_id += 1;
        id
    }
}

impl Default for WidgetHost {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::color::Color;
    use reef_render::primitive::VisualPrimitive;

    struct TestWidget {
        color: Color,
        size: Size,
    }

    impl Widget for TestWidget {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn measure(&self, _constraints: Constraints, _ctx: &mut MeasureContext) -> Size {
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
}
