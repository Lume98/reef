use echoisland_runtime::RuntimeSnapshot;

#[cfg(feature = "tauri-host")]
pub fn show<R: tauri::Runtime + 'static>(
    _app: &tauri::AppHandle<R>,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn show_without_app(_snapshot: &RuntimeSnapshot) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn show_without_app(_snapshot: &RuntimeSnapshot) -> Result<(), String> {
    Ok(())
}

pub fn hide() -> Result<(), String> {
    Ok(())
}

pub fn snap() -> Result<(), String> {
    Ok(())
}

