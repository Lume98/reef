use crate::state::PanelRect;
#[cfg(feature = "tauri-host")]
use tauri::AppHandle;
#[cfg(feature = "tauri-host")]
use tauri::Manager;

use crate::scene::PanelDisplayOptionState;

pub trait NativePanelHostPlatform: Clone + Send + Sync + 'static {
    fn focus_main_window(&self) -> Result<(), String>;
    fn quit_application(&self);
    fn available_displays(&self) -> Result<Vec<PanelDisplayOptionState>, String>;
    fn open_settings_location(&self) -> Result<(), String>;
    fn open_release_page(&self) -> Result<(), String>;
    fn hide_native_panel(&self) -> Result<(), String> {
        Ok(())
    }
    fn refresh_native_panel_from_last_snapshot(&self) -> Result<(), String> {
        Ok(())
    }
    fn reposition_native_panel_to_selected_display(&self) -> Result<(), String> {
        Ok(())
    }
    fn set_shared_expanded_body_height(&self, _body_height: f64) -> Result<(), String> {
        Ok(())
    }
    fn spawn_platform_loops(&self) {}
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

    fn available_displays(&self) -> Result<Vec<PanelDisplayOptionState>, String> {
        self.available_monitors()
            .map(|_| Vec::new())
            .map_err(|error| error.to_string())
    }

    fn open_settings_location(&self) -> Result<(), String> {
        Err("settings location is not wired in the shared core".to_string())
    }

    fn open_release_page(&self) -> Result<(), String> {
        Err("release page is not wired in the shared core".to_string())
    }

    fn run_on_platform_thread(&self, work: impl FnOnce() + Send + 'static) -> Result<(), String> {
        self.run_on_main_thread(work)
            .map_err(|error| error.to_string())
    }
}

#[cfg(feature = "tauri-host")]
impl<R: tauri::Runtime + 'static> crate::updater_service::NativePanelReleasePageHost
    for AppHandle<R>
{
    fn open_release_page(&self) -> Result<(), String> {
        tauri_plugin_opener::OpenerExt::opener(self)
            .open_url(
                "https://github.com/Lume98/ai-gateway/releases",
                None::<&str>,
            )
            .map_err(|error| error.to_string())
    }
}

pub fn fallback_panel_frame() -> PanelRect {
    PanelRect {
        x: 0.0,
        y: 0.0,
        width: 1440.0,
        height: 900.0,
    }
}
