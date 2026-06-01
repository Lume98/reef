#[allow(unused_imports)]
pub use echoisland_ui::{AppUpdatePhase, AppUpdateStatus};

pub fn current_update_status() -> AppUpdateStatus {
    AppUpdateStatus::idle()
}

#[cfg(feature = "tauri-host")]
pub fn spawn_native_update_flow<H>(host: H)
where
    H: crate::host_platform::NativePanelHostPlatform,
{
    if let Err(error) = host.open_release_page() {
        log::warn!("打开 AI Gateway 发布页失败: {error}");
    }
}
