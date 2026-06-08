#[allow(unused_imports)]
pub use crate::updater_service::{AppUpdatePhase, AppUpdateStatus};

pub fn current_update_status() -> AppUpdateStatus {
    crate::updater_service::current_update_status()
}
