use echoisland_runtime::RuntimeSnapshot;

use crate::native_panel_renderer::facade::runtime::{
    NativePanelPlatformRuntimeBackendFacade, NativePanelPlatformRuntimeFacadeApi,
};

use super::{create_native_panel, native_ui_enabled};

pub(crate) struct WindowsNativePanelRuntimeBackendApi;

impl NativePanelPlatformRuntimeFacadeApi for WindowsNativePanelRuntimeBackendApi {
    fn native_ui_enabled() -> bool {
        native_ui_enabled()
    }

    fn create_panel() -> Result<(), String> {
        create_native_panel()
    }

    fn spawn_platform_loops<H: crate::host_platform::NativePanelHostPlatform + Clone + 'static>(
        app: H,
    ) {
        app.spawn_platform_loops();
    }

    fn update_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        app: &H,
        snapshot: &RuntimeSnapshot,
    ) -> Result<(), String> {
        let _ = snapshot;
        app.refresh_native_panel_from_last_snapshot()
    }

    fn hide_panel<H: crate::host_platform::NativePanelHostPlatform>(app: &H) -> Result<(), String> {
        app.hide_native_panel()
    }

    fn refresh_from_last_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        app: &H,
    ) -> Result<(), String> {
        app.refresh_native_panel_from_last_snapshot()
    }

    fn reposition_to_selected_display<H: crate::host_platform::NativePanelHostPlatform>(
        app: &H,
    ) -> Result<(), String> {
        app.reposition_native_panel_to_selected_display()
    }

    fn set_shared_expanded_body_height<H: crate::host_platform::NativePanelHostPlatform>(
        app: &H,
        body_height: f64,
    ) -> Result<(), String> {
        app.set_shared_expanded_body_height(body_height)
    }
}

pub(crate) type WindowsNativePanelRuntimeBackendFacade =
    NativePanelPlatformRuntimeBackendFacade<WindowsNativePanelRuntimeBackendApi>;

pub(crate) fn current_windows_native_panel_runtime_backend(
) -> WindowsNativePanelRuntimeBackendFacade {
    NativePanelPlatformRuntimeBackendFacade::new()
}
