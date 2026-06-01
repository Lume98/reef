#[cfg(all(target_os = "windows", feature = "tauri-host"))]
pub fn show<R: tauri::Runtime + 'static>(
    app: &tauri::AppHandle<R>,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
) -> Result<(), String> {
    crate::windows_native_panel::spawn_platform_loops(app.clone());
    crate::windows_native_panel::create_native_panel()?;
    crate::windows_native_panel::update_native_panel_snapshot(
        app,
        &echoisland_runtime::RuntimeSnapshot::idle(),
    )
}

#[cfg(target_os = "windows")]
pub fn show_without_app(snapshot: &echoisland_runtime::RuntimeSnapshot) -> Result<(), String> {
    crate::windows_native_panel::spawn_platform_loops_without_app();
    crate::windows_native_panel::create_native_panel()?;
    crate::windows_native_panel::update_native_panel_snapshot_without_app(snapshot)
}

#[cfg(target_os = "windows")]
pub fn hide() -> Result<(), String> {
    crate::windows_native_panel::hide_native_panel_without_app()
}

#[cfg(target_os = "windows")]
pub fn snap() -> Result<(), String> {
    Ok(())
}

#[cfg(all(not(target_os = "windows"), feature = "tauri-host"))]
pub fn show<R: tauri::Runtime>(
    _app: &tauri::AppHandle<R>,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn show_without_app(_snapshot: &echoisland_runtime::RuntimeSnapshot) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn hide() -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn snap() -> Result<(), String> {
    Ok(())
}
