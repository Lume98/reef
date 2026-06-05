use reef_core::{
    event::Event,
    geometry::{point_in_rect, Point, Rect, Size},
};
use reef_draw::primitive::{DrawPlan, DrawPrimitive};
use reef_layout::Constraints;

/// Reef UI 树中的可视化组件。
///
/// 采用类 React 设计：每个组件是一个结构体，属性作为字段。
/// 容器组件将子组件作为字段持有，并在 measure/paint/handle_event 中递归处理。
pub trait Widget {
    /// 在给定约束下计算期望尺寸。
    fn measure(&self, constraints: Constraints) -> Size;

    /// 将自身绘制到图元列表中。容器组件应递归绘制子组件，
    /// 根据测量结果和布局策略计算子组件区域。
    fn paint(&self, rect: Rect, ctx: &mut PaintContext);

    /// 处理事件。返回 true 表示事件已被消费（停止冒泡）。
    /// 容器组件应先尝试将事件转发给命中的子组件，
    /// 若未被消费，再自行处理。
    fn handle_event(&mut self, _event: &Event, _rect: Rect, _ctx: &mut EventContext) -> bool {
        false
    }
}

/// paint 阶段的上下文，用于收集绘制图元。
pub struct PaintContext<'a> {
    pub primitives: &'a mut Vec<DrawPrimitive>,
}

/// 事件处理阶段的上下文，用于标记组件树是否需要重绘。
pub struct EventContext<'a> {
    pub dirty: &'a mut bool,
}

/// 管理根组件并协调 measure/paint 周期。
pub struct WidgetHost {
    /// 根组件
    root: Option<Box<dyn Widget>>,
    /// 脏标记，为 true 时表示需要重新 measure/paint
    dirty: bool,
    /// 最近一次生成的视觉计划
    last_plan: Option<DrawPlan>,
    /// 窗口逻辑尺寸
    size: Size,
}

impl WidgetHost {
    pub fn new() -> Self {
        Self {
            root: None,
            dirty: true,
            last_plan: None,
            size: Size {
                width: 800.0,
                height: 600.0,
            },
        }
    }

    /// 设置根组件，并标记为脏。
    pub fn set_root(&mut self, widget: Box<dyn Widget>) {
        self.root = Some(widget);
        self.dirty = true;
    }

    /// 以泛型组件类型设置根组件，避免调用方手动装箱。
    pub fn set_root_widget<W: Widget + 'static>(&mut self, widget: W) {
        self.set_root(Box::new(widget));
    }

    /// 设置窗口尺寸，尺寸变化时自动标记为脏。
    pub fn set_size(&mut self, size: Size) {
        if self.size != size {
            self.size = size;
            self.dirty = true;
        }
    }

    /// 手动标记组件树需要重新渲染。
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// 返回当前宿主是否需要重新布局或绘制。
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// 获取根组件的不可变引用。
    pub fn root(&self) -> Option<&dyn Widget> {
        self.root.as_deref()
    }

    /// 获取根组件的可变引用。
    pub fn root_mut(&mut self) -> Option<&mut Box<dyn Widget>> {
        self.root.as_mut()
    }

    /// 移除并返回当前根组件。
    pub fn take_root(&mut self) -> Option<Box<dyn Widget>> {
        self.dirty = true;
        self.last_plan = None;
        self.root.take()
    }

    /// 先 measure 再 paint，产出完整的 DrawPlan 交给渲染层。
    pub fn render(&mut self) -> DrawPlan {
        if !self.dirty {
            if let Some(plan) = &self.last_plan {
                return plan.clone();
            }
        }

        let mut plan = DrawPlan::with_viewport(self.size);
        if let Some(root) = &self.root {
            let root_rect = Rect {
                x: 0.0,
                y: 0.0,
                width: self.size.width,
                height: self.size.height,
            };
            let constraints = Constraints::loose(self.size);

            // measure 阶段：自顶向下传递约束，自底向上返回尺寸
            let _measured = root.measure(constraints);

            // paint 阶段：根据测量结果递归绘制
            let mut primitives = Vec::new();
            let mut ctx = PaintContext {
                primitives: &mut primitives,
            };
            root.paint(root_rect, &mut ctx);
            plan.primitives = primitives;
        }
        self.last_plan = Some(plan.clone());
        self.dirty = false;
        plan
    }

    /// 向组件树分发事件。
    /// 调用根组件的 handle_event，由其负责递归命中测试和事件冒泡。
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
        // 超出根区域的事件直接丢弃
        if !point_in_rect(position, root_rect) {
            return false;
        }
        let mut dirty = false;
        let mut ctx = EventContext { dirty: &mut dirty };
        let consumed = root.handle_event(event, root_rect, &mut ctx);
        // 事件处理过程中若有状态变更，标记需要重绘
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

/// 容器组件的辅助函数：向子组件转发事件。
/// 自动进行命中测试，只有事件位置在子组件区域内时才转发。
/// 返回 true 表示子组件消费了该事件。
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
            ctx.primitives.push(DrawPrimitive::RoundRect {
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
        use reef_core::event::{PointerEvent, PointerEventKind};

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

    struct CountingWidget {
        size: Size,
        measure_count: std::rc::Rc<std::cell::Cell<usize>>,
        paint_count: std::rc::Rc<std::cell::Cell<usize>>,
    }

    impl Widget for CountingWidget {
        fn measure(&self, _constraints: Constraints) -> Size {
            self.measure_count
                .set(self.measure_count.get().saturating_add(1));
            self.size
        }

        fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
            self.paint_count
                .set(self.paint_count.get().saturating_add(1));
            ctx.primitives.push(DrawPrimitive::RoundRect {
                frame: rect,
                radius: 8.0,
                color: Color::BLACK,
                alpha: 1.0,
            });
        }
    }

    #[test]
    fn widget_host_reuses_clean_render_plan() {
        let measure_count = std::rc::Rc::new(std::cell::Cell::new(0));
        let paint_count = std::rc::Rc::new(std::cell::Cell::new(0));

        let mut host = WidgetHost::new();
        host.set_size(Size {
            width: 200.0,
            height: 100.0,
        });
        host.set_root_widget(CountingWidget {
            size: Size {
                width: 200.0,
                height: 100.0,
            },
            measure_count: measure_count.clone(),
            paint_count: paint_count.clone(),
        });

        let first = host.render();
        let second = host.render();

        assert_eq!(first, second);
        assert_eq!(measure_count.get(), 1);
        assert_eq!(paint_count.get(), 1);
        assert!(!host.is_dirty());
    }
}
