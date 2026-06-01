use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppUpdatePhase {
    Idle,
    Checking,
    UpToDate,
    Available,
    Downloading,
    Installing,
    Installed,
    Failed,
    UnsupportedPortable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUpdateStatus {
    pub phase: AppUpdatePhase,
    pub label: String,
    pub value_text: String,
    pub version: Option<String>,
    pub error: Option<String>,
    pub can_install: bool,
    pub can_open_release_page: bool,
}

impl AppUpdateStatus {
    pub fn idle() -> Self {
        Self {
            phase: AppUpdatePhase::Idle,
            label: "AI Gateway".to_string(),
            value_text: "Release".to_string(),
            version: None,
            error: None,
            can_install: false,
            can_open_release_page: true,
        }
    }
}

pub fn current_update_status() -> AppUpdateStatus {
    AppUpdateStatus::idle()
}

#[cfg(feature = "tauri-host")]
pub fn spawn_native_update_flow<R: tauri::Runtime + 'static>(app: tauri::AppHandle<R>) {
    tauri::async_runtime::spawn(async move {
        if let Err(error) = tauri_plugin_opener::OpenerExt::opener(&app).open_url(
            "https://github.com/Lume98/ai-gateway/releases",
            None::<&str>,
        ) {
            log::warn!("打开 AI Gateway 发布页失败: {error}");
        }
    });
}
