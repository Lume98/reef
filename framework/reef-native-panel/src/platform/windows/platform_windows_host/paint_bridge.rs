use crate::runtime::facade::{
    descriptor::{NativePanelHostWindowState, NativePanelPointerRegion},
    presentation::NativePanelPresentationModel,
};

use super::{
    draw_presenter::WindowsPanelDrawPresenter,
    host_window::{WindowsPanelDrawFrame, WindowsPanelHostWindow},
    window_shell::{WindowsPanelShellPresentResult, WindowsPanelWindowShell},
};

pub(super) fn present_window_into_presenter(
    window: &mut WindowsPanelHostWindow,
    presenter: &mut WindowsPanelDrawPresenter,
    window_state: NativePanelHostWindowState,
    pointer_regions: &[NativePanelPointerRegion],
    presentation_model: Option<NativePanelPresentationModel>,
) {
    window.present(window_state, pointer_regions, presentation_model);
    sync_draw_presenter(window, presenter);
}

pub(super) fn sync_draw_presenter(
    window: &mut WindowsPanelHostWindow,
    presenter: &mut WindowsPanelDrawPresenter,
) {
    if let Some(frame) = window.take_pending_draw_frame() {
        presenter.present(frame);
    }
}

pub(super) fn take_pending_draw_frame(
    presenter: &mut WindowsPanelDrawPresenter,
) -> Option<WindowsPanelDrawFrame> {
    presenter.take_redraw_frame()
}

pub(super) fn consume_presenter_into_shell(
    presenter: &mut WindowsPanelDrawPresenter,
    shell: &mut WindowsPanelWindowShell,
) -> bool {
    consume_presenter_into_shell_result(presenter, shell).redraw_requested
}

pub(super) fn consume_presenter_into_shell_result(
    presenter: &mut WindowsPanelDrawPresenter,
    shell: &mut WindowsPanelWindowShell,
) -> WindowsPanelShellPresentResult {
    shell.consume_presenter(presenter)
}
