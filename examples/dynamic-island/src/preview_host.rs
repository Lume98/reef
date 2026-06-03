use echoisland_runtime::RuntimeSnapshot;

pub trait DynamicIslandUiPreviewHost {
    fn show(&self, snapshot: &RuntimeSnapshot) -> Result<(), String>;
    fn run(&self) -> Result<(), String>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StandaloneDynamicIslandUiPreviewHost;

impl DynamicIslandUiPreviewHost for StandaloneDynamicIslandUiPreviewHost {
    fn show(&self, snapshot: &RuntimeSnapshot) -> Result<(), String> {
        crate::native_window::show_without_app(snapshot)
    }

    fn run(&self) -> Result<(), String> {
        loop {
            std::thread::park();
        }
    }
}

#[cfg(feature = "tauri-host")]
pub struct TauriDynamicIslandUiPreviewHost<'a, R: tauri::Runtime> {
    app: &'a tauri::AppHandle<R>,
}

#[cfg(feature = "tauri-host")]
impl<'a, R: tauri::Runtime> TauriDynamicIslandUiPreviewHost<'a, R> {
    pub fn new(app: &'a tauri::AppHandle<R>) -> Self {
        Self { app }
    }
}

#[cfg(feature = "tauri-host")]
impl<R: tauri::Runtime + 'static> DynamicIslandUiPreviewHost
    for TauriDynamicIslandUiPreviewHost<'_, R>
{
    fn show(&self, snapshot: &RuntimeSnapshot) -> Result<(), String> {
        crate::native_window::show(self.app, 0, 0, 0, 0)?;

        #[cfg(target_os = "windows")]
        crate::windows_native_panel::update_native_panel_snapshot(self.app, snapshot)?;

        Ok(())
    }

    fn run(&self) -> Result<(), String> {
        Ok(())
    }
}

pub fn run_dynamic_island_ui_preview_standalone() -> Result<(), String> {
    let host = StandaloneDynamicIslandUiPreviewHost;
    let snapshot = dynamic_island_ui_preview_snapshot();
    host.show(&snapshot)?;
    host.run()
}

#[cfg(feature = "tauri-host")]
pub fn show_dynamic_island_ui_preview(
    app: &tauri::AppHandle,
    snapshot: &RuntimeSnapshot,
) -> Result<(), String> {
    TauriDynamicIslandUiPreviewHost::new(app).show(snapshot)
}

/// 构造一份稳定的灵动岛 UI 预览数据。
pub fn dynamic_island_ui_preview_snapshot() -> RuntimeSnapshot {
    reef_native_panel_core::preview_host::dynamic_island_ui_preview_snapshot()
}
