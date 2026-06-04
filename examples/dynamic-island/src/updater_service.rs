#[allow(unused_imports)]
pub use reef_native_panel_core::updater_service::{AppUpdatePhase, AppUpdateStatus};

pub fn current_update_status() -> AppUpdateStatus {
    reef_native_panel_core::updater_service::current_update_status()
}
