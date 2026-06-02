use reef_core::geometry::Size;
use reef_render::render_backend::RenderBackend;

use crate::widget_host::WidgetHost;

pub struct App<B: RenderBackend> {
    backend: B,
    host: WidgetHost,
}

impl<B: RenderBackend> App<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            host: WidgetHost::new(),
        }
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.host.set_size(Size { width, height });
        self
    }

    pub fn render(&mut self) -> Result<(), B::Error> {
        let plan = self.host.render();
        let submission = reef_render::render_backend::FrameSubmission {
            hidden: plan.hidden,
            commands: vec![plan],
        };
        self.backend.submit_frame(&submission)
    }

    pub fn host(&self) -> &WidgetHost {
        &self.host
    }

    pub fn host_mut(&mut self) -> &mut WidgetHost {
        &mut self.host
    }
}
