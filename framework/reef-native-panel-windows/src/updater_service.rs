#[allow(unused_imports)]
pub use reef_ui::panel::{AppUpdatePhase, AppUpdateStatus};

pub fn current_update_status() -> AppUpdateStatus {
    reef_ui::updater_service::current_update_status()
}
