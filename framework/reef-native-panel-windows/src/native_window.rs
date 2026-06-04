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
