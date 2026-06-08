use crate::runtime::facade::command::NativePanelPlatformEvent;

use std::sync::{Arc, Mutex, OnceLock};

use tokio::sync::Notify;
use tracing::warn;

use super::{
    platform_loop::{
        ensure_windows_native_platform_loop_thread, platform_loop_thread_started,
        wake_windows_native_platform_loop,
    },
    WindowsPanelRuntime,
};

static WINDOWS_PANEL_RUNTIME: OnceLock<Mutex<WindowsPanelRuntime>> = OnceLock::new();
static WINDOWS_PANEL_EVENT_DISPATCH_NOTIFY: OnceLock<Arc<Notify>> = OnceLock::new();

pub(super) fn windows_panel_runtime() -> &'static Mutex<WindowsPanelRuntime> {
    WINDOWS_PANEL_RUNTIME.get_or_init(|| Mutex::new(WindowsPanelRuntime::default()))
}

pub(super) fn pump_windows_panel_runtime_once() -> Result<(), String> {
    let result = {
        let mut runtime = windows_panel_runtime()
            .lock()
            .map_err(|_| "windows native panel runtime poisoned".to_string())?;
        runtime.pump_platform_loop()
    };
    if let Err(error) = &result {
        warn!(error = %error, "windows native panel platform pump failed");
    }
    notify_windows_panel_event_dispatcher();
    result
}

pub(super) fn windows_panel_event_dispatch_notifier() -> Arc<Notify> {
    WINDOWS_PANEL_EVENT_DISPATCH_NOTIFY
        .get_or_init(|| Arc::new(Notify::new()))
        .clone()
}

pub(super) fn notify_windows_panel_event_dispatcher() {
    windows_panel_event_dispatch_notifier().notify_one();
}

pub(super) fn with_windows_panel_runtime<T>(
    f: impl FnOnce(&mut WindowsPanelRuntime) -> Result<T, String>,
) -> Result<T, String> {
    let value = {
        let mut runtime = windows_panel_runtime()
            .lock()
            .map_err(|_| "windows native panel runtime poisoned".to_string())?;
        f(&mut runtime)?
    };
    if platform_loop_thread_started() {
        wake_windows_native_platform_loop();
    } else {
        pump_windows_panel_runtime_once()?;
    }
    Ok(value)
}

pub(super) fn drain_windows_panel_platform_events() -> Result<Vec<NativePanelPlatformEvent>, String>
{
    let mut runtime = windows_panel_runtime()
        .lock()
        .map_err(|_| "windows native panel runtime poisoned".to_string())?;
    Ok(runtime.take_queued_platform_events())
}

pub(super) fn spawn_platform_loops_internal() {
    ensure_windows_native_platform_loop_thread(pump_windows_panel_runtime_once);
    let _ = with_windows_panel_runtime(|runtime| {
        runtime.host.record_platform_loop_spawn();
        Ok(())
    });
}
