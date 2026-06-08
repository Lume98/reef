#[cfg(target_os = "windows")]
pub fn show_without_app(snapshot: &echoisland_runtime::RuntimeSnapshot) -> Result<(), String> {
    crate::platform_windows_host::spawn_platform_loops_without_app();
    let initial_generation = crate::windows_native_platform_loop_generations();
    crate::platform_windows_host::create_native_panel()?;
    crate::platform_windows_host::update_native_panel_snapshot_without_app(snapshot)?;
    wait_for_windows_panel_first_frame(initial_generation);
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn show_without_app(_snapshot: &echoisland_runtime::RuntimeSnapshot) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn wait_for_windows_panel_first_frame(initial_generation: Option<(u64, u64)>) {
    let target_generation = initial_generation
        .map(|(wake, _)| wake.saturating_add(2))
        .or_else(|| {
            crate::windows_native_platform_loop_generations()
                .map(|(wake, _)| wake.saturating_add(1))
        });
    let Some(target_generation) = target_generation else {
        return;
    };
    let _ = crate::wait_windows_native_platform_loop_processed_at_least(target_generation, 1500);
}
