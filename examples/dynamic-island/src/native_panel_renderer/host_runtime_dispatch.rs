#[cfg(feature = "tauri-host")]
use tauri::AppHandle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NativePanelRuntimeDispatchMode {
    Scheduled,
    Immediate,
}

#[cfg(feature = "tauri-host")]
pub(crate) fn dispatch_native_panel_runtime_with_handles<R, H, P, W>(
    app: &AppHandle<R>,
    handles: Option<H>,
    mode: NativePanelRuntimeDispatchMode,
    payload: P,
    work: W,
    dispatch_scheduled: impl FnOnce(&AppHandle<R>, H, P, W) -> Result<(), String>,
    dispatch_immediate: impl FnOnce(AppHandle<R>, H, P, W),
) -> Result<(), String>
where
    R: tauri::Runtime,
    H: Copy,
{
    let Some(handles) = handles else {
        return Ok(());
    };

    match mode {
        NativePanelRuntimeDispatchMode::Scheduled => {
            dispatch_scheduled(app, handles, payload, work)
        }
        NativePanelRuntimeDispatchMode::Immediate => {
            dispatch_immediate(app.clone(), handles, payload, work);
            Ok(())
        }
    }
}

#[cfg(feature = "tauri-host")]
pub(crate) fn dispatch_native_panel_runtime_payload_with_handles<R, H, P>(
    app: &AppHandle<R>,
    handles: Option<H>,
    mode: NativePanelRuntimeDispatchMode,
    payload: P,
    dispatch_scheduled: impl FnOnce(&AppHandle<R>, H, P) -> Result<(), String>,
    dispatch_immediate: impl FnOnce(AppHandle<R>, H, P),
) -> Result<(), String>
where
    R: tauri::Runtime,
    H: Copy,
{
    let Some(handles) = handles else {
        return Ok(());
    };

    match mode {
        NativePanelRuntimeDispatchMode::Scheduled => dispatch_scheduled(app, handles, payload),
        NativePanelRuntimeDispatchMode::Immediate => {
            dispatch_immediate(app.clone(), handles, payload);
            Ok(())
        }
    }
}
