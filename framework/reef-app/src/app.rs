use reef_core::geometry::Size;
use reef_render::render_backend::RenderBackend;

use crate::widget_host::WidgetHost;

/// 应用程序主结构体，负责管理渲染后端和控件树。
///
/// `App` 是 Reef 框架的顶层入口，持有渲染后端 `B` 和控件宿主 `WidgetHost`，
/// 通过 `render` 方法驱动整个 UI 的布局与绘制。
pub struct App<B: RenderBackend> {
    /// 渲染后端，负责将渲染指令提交到目标平台。
    backend: B,
    /// 控件宿主，管理控件树及其布局。
    host: WidgetHost,
}

impl<B: RenderBackend> App<B> {
    /// 使用给定的渲染后端创建新的 `App` 实例。
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            host: WidgetHost::new(),
        }
    }

    /// 设置应用程序窗口的逻辑尺寸。
    ///
    /// 采用建造者模式，便于链式调用。
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.host.set_size(Size { width, height });
        self
    }

    /// 执行一帧的布局计算和渲染提交。
    ///
    /// 先通过 `WidgetHost` 生成渲染计划，再将其封装为 `FrameSubmission`
    /// 提交给渲染后端进行实际绘制。
    pub fn render(&mut self) -> Result<(), B::Error> {
        let plan = self.host.render();
        let submission = reef_render::render_backend::FrameSubmission {
            hidden: plan.hidden,
            commands: vec![plan],
        };
        self.backend.submit_frame(&submission)
    }

    /// 获取 `WidgetHost` 的不可变引用。
    pub fn host(&self) -> &WidgetHost {
        &self.host
    }

    /// 获取 `WidgetHost` 的可变引用。
    pub fn host_mut(&mut self) -> &mut WidgetHost {
        &mut self.host
    }
}
