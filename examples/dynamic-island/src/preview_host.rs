use echoisland_runtime::RuntimeSnapshot;
pub use reef_native_panel_core::DynamicIslandUiPreviewHost;

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

pub fn run_dynamic_island_ui_preview_standalone() -> Result<(), String> {
    let host = StandaloneDynamicIslandUiPreviewHost;
    let snapshot = dynamic_island_ui_preview_snapshot();
    host.show(&snapshot)?;
    host.run()
}

/// 构造一份稳定的灵动岛 UI 预览数据。
pub fn dynamic_island_ui_preview_snapshot() -> RuntimeSnapshot {
    reef_native_panel_core::preview_host::dynamic_island_ui_preview_snapshot()
}
