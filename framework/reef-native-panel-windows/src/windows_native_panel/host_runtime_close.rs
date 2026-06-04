use super::host_runtime::WindowsNativePanelRuntime;
use crate::native_panel_renderer::facade::descriptor::NativePanelRuntimeInputDescriptor;

impl WindowsNativePanelRuntime {
    pub(super) fn sync_snapshot_bundle(
        &mut self,
        snapshot: &echoisland_runtime::RuntimeSnapshot,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<Option<crate::native_panel_core::PanelSnapshotSyncResult>, String> {
        self.sync_snapshot_bundle_impl(snapshot, input)
    }

    pub(super) fn refresh_status_queue_from_last_raw_snapshot_with_input(
        &mut self,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<bool, String> {
        self.refresh_status_queue_from_last_raw_snapshot_with_input_impl(input)
    }
}
