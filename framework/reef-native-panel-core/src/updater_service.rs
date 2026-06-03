#[allow(unused_imports)]
pub use reef_ui::{AppUpdatePhase, AppUpdateStatus};

pub fn current_update_status() -> AppUpdateStatus {
    AppUpdateStatus::idle()
}

pub trait NativePanelReleasePageHost {
    fn open_release_page(&self) -> Result<(), String>;
}

#[cfg(feature = "tauri-host")]
pub fn spawn_native_update_flow<H>(host: H)
where
    H: NativePanelReleasePageHost,
{
    if let Err(error) = host.open_release_page() {
        log::warn!("打开发布页失败: {error}");
    }
}
