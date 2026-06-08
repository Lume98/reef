use super::host_runtime::WindowsPanelRuntime;
use crate::runtime::facade::descriptor::NativePanelRuntimeInputDescriptor;

impl WindowsPanelRuntime {
    pub(super) fn sync_snapshot_bundle(
        &mut self,
        snapshot: &echoisland_runtime::RuntimeSnapshot,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<Option<crate::state::PanelSnapshotSyncResult>, String> {
        self.sync_snapshot_bundle_impl(snapshot, input)
    }

    pub(super) fn refresh_status_queue_from_last_raw_snapshot_with_input(
        &mut self,
        input: &NativePanelRuntimeInputDescriptor,
    ) -> Result<bool, String> {
        self.refresh_status_queue_from_last_raw_snapshot_with_input_impl(input)
    }
}
