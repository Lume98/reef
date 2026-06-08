use super::host_window::WindowsPanelDrawFrame;

#[derive(Clone, Debug, Default)]
pub(super) struct WindowsPanelDrawPresenter {
    last_frame: Option<WindowsPanelDrawFrame>,
    redraw_requested: bool,
}

impl WindowsPanelDrawPresenter {
    pub(super) fn present(&mut self, frame: WindowsPanelDrawFrame) {
        self.last_frame = Some(frame);
        self.redraw_requested = true;
    }

    pub(super) fn redraw_requested(&self) -> bool {
        self.redraw_requested
    }

    pub(super) fn last_frame(&self) -> Option<&WindowsPanelDrawFrame> {
        self.last_frame.as_ref()
    }

    pub(super) fn take_redraw_frame(&mut self) -> Option<WindowsPanelDrawFrame> {
        if !self.redraw_requested {
            return None;
        }
        self.redraw_requested = false;
        self.last_frame.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::WindowsPanelDrawPresenter;
    use crate::presentation::descriptor::{NativePanelHostWindowState, NativePanelPointerRegion};
    use crate::{platform_windows_host::host_window::WindowsPanelDrawFrame, state::PanelRect};

    #[test]
    fn presenter_caches_frame_and_invalidates_once() {
        let mut presenter = WindowsPanelDrawPresenter::default();
        let frame = WindowsPanelDrawFrame {
            window_state: NativePanelHostWindowState {
                frame: Some(PanelRect {
                    x: 10.0,
                    y: 20.0,
                    width: 300.0,
                    height: 120.0,
                }),
                visible: true,
                preferred_display_index: 1,
            },
            pointer_regions: Vec::<NativePanelPointerRegion>::new(),
            presentation_model: None,
        };

        presenter.present(frame.clone());

        assert!(presenter.redraw_requested());
        assert_eq!(
            presenter
                .last_frame()
                .and_then(|frame| frame.window_state.frame),
            frame.window_state.frame
        );
        assert_eq!(
            presenter
                .take_redraw_frame()
                .and_then(|frame| frame.window_state.frame),
            frame.window_state.frame
        );
        assert!(!presenter.redraw_requested());
        assert!(presenter.take_redraw_frame().is_none());
    }
}
