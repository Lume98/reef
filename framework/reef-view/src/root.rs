use reef_core::geometry::{Point, Size};
use reef_draw::primitive::DrawPlan;

use crate::widget_host::{Widget, WidgetHost};

/// Declarative root facade over `WidgetHost`, aligned with `createRoot(...).set_root(...).render_current()`.
pub struct WidgetRoot {
    host: WidgetHost,
}

impl WidgetRoot {
    pub fn new(size: Size) -> Self {
        let mut host = WidgetHost::new();
        host.set_size(size);
        Self { host }
    }

    pub fn set_size(&mut self, size: Size) {
        self.host.set_size(size);
    }

    pub fn set_root<W: Widget + 'static>(&mut self, widget: W) {
        self.host.set_root_widget(widget);
    }

    pub fn host(&self) -> &WidgetHost {
        &self.host
    }

    pub fn host_mut(&mut self) -> &mut WidgetHost {
        &mut self.host
    }

    pub fn render_current(&mut self) -> DrawPlan {
        self.host.render()
    }

    pub fn render<W: Widget + 'static>(&mut self, widget: W) -> DrawPlan {
        self.set_root(widget);
        self.render_current()
    }

    pub fn dispatch_event(&mut self, event: &reef_core::event::Event, position: Point) -> bool {
        self.host.dispatch_event(event, position)
    }
}

pub fn create_root(size: Size) -> WidgetRoot {
    WidgetRoot::new(size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget_host::{PaintContext, Widget};
    use reef_core::{
        color::Color,
        geometry::{Rect, Size},
    };
    use reef_draw::primitive::DrawPrimitive;
    use reef_layout::Constraints;

    struct TestWidget;

    impl Widget for TestWidget {
        fn measure(&self, constraints: Constraints) -> Size {
            constraints.constrain(Size {
                width: 120.0,
                height: 48.0,
            })
        }

        fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
            ctx.primitives.push(DrawPrimitive::RoundRect {
                frame: rect,
                radius: 24.0,
                color: Color::BLACK,
                alpha: 1.0,
            });
        }
    }

    #[test]
    fn widget_root_renders_declared_widget() {
        let mut root = create_root(Size {
            width: 320.0,
            height: 48.0,
        });
        root.set_root(TestWidget);
        let plan = root.render_current();

        assert!(!plan.hidden);
        assert_eq!(plan.primitives.len(), 1);
    }
}
