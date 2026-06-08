use super::{WindowsPanelRenderer, WindowsPanelRuntime};

#[test]
fn renderer_defaults_without_frame_submission() {
    let renderer = WindowsPanelRenderer::default();
    assert!(renderer.last_frame_submission.is_none());
}

#[test]
fn runtime_default_constructs() {
    let _runtime = WindowsPanelRuntime::default();
}
