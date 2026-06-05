use super::{WindowsNativePanelRenderer, WindowsNativePanelRuntime};

#[test]
fn renderer_defaults_without_frame_submission() {
    let renderer = WindowsNativePanelRenderer::default();
    assert!(renderer.last_frame_submission.is_none());
}

#[test]
fn runtime_default_constructs() {
    let _runtime = WindowsNativePanelRuntime::default();
}
