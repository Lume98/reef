use crate::display_settings::DisplayOption;

#[cfg(feature = "tauri-host")]
use tauri::AppHandle;

pub(crate) trait NativePanelHostPlatform: Clone + Send + Sync + 'static {
    fn focus_main_window(&self) -> Result<(), String>;

    fn quit_application(&self);

    fn available_displays(&self) -> Result<Vec<DisplayOption>, String>;

    fn open_settings_location(&self) -> Result<(), String>;

    fn open_release_page(&self) -> Result<(), String>;

    fn run_on_platform_thread(&self, work: impl FnOnce() + Send + 'static) -> Result<(), String>;
}

#[cfg(feature = "tauri-host")]
impl<R: tauri::Runtime + 'static> NativePanelHostPlatform for AppHandle<R> {
    fn focus_main_window(&self) -> Result<(), String> {
        if let Some(window) = self.get_webview_window("main") {
            let _ = window.show();
            let _ = window.unminimize();
            let _ = window.set_focus();
        }
        Ok(())
    }

    fn quit_application(&self) {
        self.exit(0);
    }

    fn available_displays(&self) -> Result<Vec<DisplayOption>, String> {
        self.available_monitors()
            .map(|monitors| crate::display_settings::display_options_from_monitors(&monitors))
            .map_err(|error| error.to_string())
    }

    fn open_settings_location(&self) -> Result<(), String> {
        tauri_plugin_opener::OpenerExt::opener(self)
            .open_path(
                crate::config::get_app_config_dir().to_string_lossy().to_string(),
                None::<&str>,
            )
            .map_err(|error| error.to_string())
    }

    fn open_release_page(&self) -> Result<(), String> {
        tauri_plugin_opener::OpenerExt::opener(self)
            .open_url("https://github.com/Lume98/ai-gateway/releases", None::<&str>)
            .map_err(|error| error.to_string())
    }

    fn run_on_platform_thread(&self, work: impl FnOnce() + Send + 'static) -> Result<(), String> {
        self.run_on_main_thread(work)
            .map_err(|error| error.to_string())
    }
}
