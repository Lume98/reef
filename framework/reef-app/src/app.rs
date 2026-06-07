use reef_core::geometry::Size;
use reef_dom::ReefRenderer;
use reef_draw::draw_backend::{DrawBackend, FrameSubmission};
use reef_draw::primitive::DrawPlan;
use reef_vnode::VNode;

/// Reef 的 React-like 顶层入口。
///
/// `App` 持有平台绘制后端和 VNode renderer，主路径为：
/// `rsx! -> VNode -> Reconciler -> HostConfig -> DrawPlan -> DrawBackend`。
pub struct App<B: DrawBackend> {
    backend: B,
    renderer: ReefRenderer,
}

impl<B: DrawBackend> App<B> {
    /// 使用给定后端和视口尺寸创建应用。
    pub fn new(backend: B, viewport: Size) -> Self {
        Self {
            backend,
            renderer: ReefRenderer::new(viewport),
        }
    }

    /// 渲染 VNode 并提交到绘制后端。
    pub fn render(&mut self, vnode: VNode) -> Result<(), B::Error> {
        let plan = self.render_plan(vnode);
        let submission = FrameSubmission {
            hidden: plan.hidden,
            plans: vec![plan],
        };
        self.backend.submit_frame(&submission)
    }

    /// 渲染 VNode 并返回 DrawPlan，不提交后端。
    pub fn render_plan(&mut self, vnode: VNode) -> DrawPlan {
        self.renderer.render(vnode)
    }

    /// 设置当前逻辑视口尺寸。
    pub fn set_viewport(&mut self, size: Size) {
        self.renderer.set_viewport(size);
    }

    pub fn viewport(&self) -> Size {
        self.renderer.viewport()
    }

    pub fn renderer(&self) -> &ReefRenderer {
        &self.renderer
    }

    pub fn renderer_mut(&mut self) -> &mut ReefRenderer {
        &mut self.renderer
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn into_backend(self) -> B {
        self.backend
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::color::Color;
    use reef_draw::draw_backend::FrameSubmission;
    use reef_vnode::{Container, Label};

    #[derive(Default)]
    struct RecordingBackend {
        submissions: Vec<FrameSubmission>,
    }

    impl DrawBackend for RecordingBackend {
        type Error = ();

        fn submit_frame(&mut self, submission: &FrameSubmission) -> Result<(), Self::Error> {
            self.submissions.push(submission.clone());
            Ok(())
        }
    }

    #[test]
    fn render_plan_uses_vnode_pipeline() {
        let mut app = App::new(
            RecordingBackend::default(),
            Size {
                width: 320.0,
                height: 180.0,
            },
        );

        let vnode = reef_view_macros::rsx! {
            <Container color={Color::rgb(18, 18, 22)} radius={16.0}>
                <Label text={"Hello"} color={Color::WHITE} />
            </Container>
        };

        let plan = app.render_plan(vnode);

        assert_eq!(plan.viewport.width, 320.0);
        assert!(!plan.primitives.is_empty());
    }

    #[test]
    fn render_submits_frame_to_backend() {
        let mut app = App::new(
            RecordingBackend::default(),
            Size {
                width: 320.0,
                height: 180.0,
            },
        );

        let vnode = reef_view_macros::rsx! {
            <container color={Color::rgb(18, 18, 22)} />
        };

        app.render(vnode).unwrap();

        assert_eq!(app.backend().submissions.len(), 1);
        assert_eq!(app.backend().submissions[0].plans.len(), 1);
    }
}
