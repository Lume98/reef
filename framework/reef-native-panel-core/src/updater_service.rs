pub use reef_ui::updater_service::{AppUpdatePhase, AppUpdateStatus};

pub trait NativePanelReleasePageHost {
    fn open_release_page(&self) -> Result<(), String>;
}

pub fn current_update_status() -> AppUpdateStatus {
    reef_ui::updater_service::current_update_status()
}

#[cfg(feature = "tauri-host")]
pub fn spawn_native_update_flow<H>(host: H)
where
    H: NativePanelReleasePageHost,
{
    if let Err(error) = host.open_release_page() {
        log::warn!("打开 AI Gateway 发布页失败: {error}");
    }
}
