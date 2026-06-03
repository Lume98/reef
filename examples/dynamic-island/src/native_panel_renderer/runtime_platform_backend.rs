use echoisland_runtime::RuntimeSnapshot;
use std::marker::PhantomData;

#[cfg(feature = "tauri-host")]
pub(crate) fn reposition_native_panel_to_selected_display_then_refresh<H>(
    host: &H,
    reposition: impl FnOnce(&H) -> Result<(), String>,
    refresh: impl FnOnce(&H) -> Result<(), String>,
) -> Result<(), String>
where
    H: crate::host_platform::NativePanelHostPlatform,
{
    reposition(host)?;
    refresh(host)
}

#[cfg(feature = "tauri-host")]
pub(crate) trait NativePanelPlatformRuntimeBackend {
    fn native_ui_enabled(&self) -> bool;

    fn create_panel(&self) -> Result<(), String>;

    fn hide_legacy_app_window<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String>;

    fn spawn_platform_loops<H: crate::host_platform::NativePanelHostPlatform + Clone + 'static>(
        &self,
        host: H,
    );

    fn update_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
        snapshot: &RuntimeSnapshot,
    ) -> Result<(), String>;

    fn hide_panel<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String>;

    fn refresh_from_last_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String>;

    fn reposition_to_selected_display<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String>;

    fn set_shared_expanded_body_height<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
        body_height: f64,
    ) -> Result<(), String>;
}

#[cfg(feature = "tauri-host")]
pub(crate) trait NativePanelPlatformRuntimeFacadeApi {
    fn native_ui_enabled() -> bool;

    fn create_panel() -> Result<(), String>;

    fn hide_legacy_app_window<H: crate::host_platform::NativePanelHostPlatform>(
        _: &H,
    ) -> Result<(), String> {
        Ok(())
    }

    fn spawn_platform_loops<H: crate::host_platform::NativePanelHostPlatform + Clone + 'static>(
        host: H,
    );

    fn update_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        host: &H,
        snapshot: &RuntimeSnapshot,
    ) -> Result<(), String>;

    fn hide_panel<H: crate::host_platform::NativePanelHostPlatform>(host: &H)
        -> Result<(), String>;

    fn refresh_from_last_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        host: &H,
    ) -> Result<(), String>;

    fn reposition_to_selected_display<H: crate::host_platform::NativePanelHostPlatform>(
        host: &H,
    ) -> Result<(), String>;

    fn set_shared_expanded_body_height<H: crate::host_platform::NativePanelHostPlatform>(
        host: &H,
        body_height: f64,
    ) -> Result<(), String>;
}

#[cfg(feature = "tauri-host")]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct NativePanelPlatformRuntimeBackendFacade<Api> {
    marker: PhantomData<fn() -> Api>,
}

#[cfg(feature = "tauri-host")]
impl<Api> NativePanelPlatformRuntimeBackendFacade<Api> {
    pub(crate) fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "tauri-host")]
impl<Api> NativePanelPlatformRuntimeBackend for NativePanelPlatformRuntimeBackendFacade<Api>
where
    Api: NativePanelPlatformRuntimeFacadeApi,
{
    fn native_ui_enabled(&self) -> bool {
        Api::native_ui_enabled()
    }

    fn create_panel(&self) -> Result<(), String> {
        Api::create_panel()
    }

    fn hide_legacy_app_window<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String> {
        Api::hide_legacy_app_window(host)
    }

    fn spawn_platform_loops<H: crate::host_platform::NativePanelHostPlatform + Clone + 'static>(
        &self,
        host: H,
    ) {
        Api::spawn_platform_loops(host);
    }

    fn update_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
        snapshot: &RuntimeSnapshot,
    ) -> Result<(), String> {
        Api::update_snapshot(host, snapshot)
    }

    fn hide_panel<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String> {
        Api::hide_panel(host)
    }

    fn refresh_from_last_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String> {
        Api::refresh_from_last_snapshot(host)
    }

    fn reposition_to_selected_display<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String> {
        Api::reposition_to_selected_display(host)
    }

    fn set_shared_expanded_body_height<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
        body_height: f64,
    ) -> Result<(), String> {
        Api::set_shared_expanded_body_height(host, body_height)
    }
}

#[cfg(feature = "tauri-host")]
pub(crate) trait NativePanelRuntimeBackend: NativePanelPlatformRuntimeBackend {
    fn native_ui_enabled(&self) -> bool {
        NativePanelPlatformRuntimeBackend::native_ui_enabled(self)
    }

    fn create_panel(&self) -> Result<(), String> {
        NativePanelPlatformRuntimeBackend::create_panel(self)
    }

    fn hide_legacy_app_window<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String> {
        host.hide_native_panel()
    }

    fn spawn_platform_loops<H: crate::host_platform::NativePanelHostPlatform + Clone + 'static>(
        &self,
        host: H,
    ) {
        host.spawn_platform_loops();
    }

    fn update_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
        snapshot: &RuntimeSnapshot,
    ) -> Result<(), String> {
        let _ = snapshot;
        host.refresh_native_panel_from_last_snapshot()
    }

    fn hide_panel<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String> {
        host.hide_native_panel()
    }

    fn refresh_from_last_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String> {
        host.refresh_native_panel_from_last_snapshot()
    }

    fn reposition_to_selected_display<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
    ) -> Result<(), String> {
        host.reposition_native_panel_to_selected_display()
    }

    fn set_shared_expanded_body_height<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        host: &H,
        body_height: f64,
    ) -> Result<(), String> {
        host.set_shared_expanded_body_height(body_height)
    }
}

#[cfg(feature = "tauri-host")]
impl<T> NativePanelRuntimeBackend for T where T: NativePanelPlatformRuntimeBackend {}

#[cfg(all(feature = "tauri-host", target_os = "macos"))]
#[cfg(target_os = "macos")]
pub(crate) type CurrentNativePanelRuntimeBackend =
    crate::macos_native_panel::MacosNativePanelRuntimeBackendFacade;

#[cfg(all(feature = "tauri-host", target_os = "windows"))]
#[cfg(target_os = "windows")]
pub(crate) type CurrentNativePanelRuntimeBackend =
    crate::windows_native_panel::WindowsNativePanelRuntimeBackendFacade;

#[cfg(all(
    feature = "tauri-host",
    not(target_os = "macos"),
    not(target_os = "windows")
))]
#[cfg(not(target_os = "macos"))]
#[cfg(not(target_os = "windows"))]
pub(crate) struct CurrentNativePanelRuntimeBackend;

#[cfg(all(feature = "tauri-host", target_os = "macos"))]
#[cfg(target_os = "macos")]
pub(crate) fn current_native_panel_runtime_backend() -> CurrentNativePanelRuntimeBackend {
    crate::macos_native_panel::current_macos_native_panel_runtime_backend()
}

#[cfg(all(feature = "tauri-host", target_os = "windows"))]
#[cfg(target_os = "windows")]
pub(crate) fn current_native_panel_runtime_backend() -> CurrentNativePanelRuntimeBackend {
    crate::windows_native_panel::current_windows_native_panel_runtime_backend()
}

#[cfg(all(
    feature = "tauri-host",
    not(target_os = "macos"),
    not(target_os = "windows")
))]
#[cfg(not(target_os = "macos"))]
#[cfg(not(target_os = "windows"))]
impl NativePanelPlatformRuntimeBackend for CurrentNativePanelRuntimeBackend {
    fn native_ui_enabled(&self) -> bool {
        false
    }

    fn create_panel(&self) -> Result<(), String> {
        Ok(())
    }

    fn hide_legacy_app_window<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        _: &H,
    ) -> Result<(), String> {
        Ok(())
    }

    fn spawn_platform_loops<H: crate::host_platform::NativePanelHostPlatform + Clone + 'static>(
        &self,
        _: H,
    ) {
    }

    fn update_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        _: &H,
        _: &RuntimeSnapshot,
    ) -> Result<(), String> {
        Ok(())
    }

    fn hide_panel<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        _: &H,
    ) -> Result<(), String> {
        Ok(())
    }

    fn refresh_from_last_snapshot<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        _: &H,
    ) -> Result<(), String> {
        Ok(())
    }

    fn reposition_to_selected_display<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        _: &H,
    ) -> Result<(), String> {
        Ok(())
    }

    fn set_shared_expanded_body_height<H: crate::host_platform::NativePanelHostPlatform>(
        &self,
        _: &H,
        _: f64,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(all(
    feature = "tauri-host",
    not(target_os = "macos"),
    not(target_os = "windows")
))]
#[cfg(not(target_os = "macos"))]
#[cfg(not(target_os = "windows"))]
pub(crate) fn current_native_panel_runtime_backend() -> CurrentNativePanelRuntimeBackend {
    CurrentNativePanelRuntimeBackend
}
