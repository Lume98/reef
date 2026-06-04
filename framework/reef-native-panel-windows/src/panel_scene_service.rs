//! 面板场景服务。
//!
//! 该服务持有跨快照的 `PanelState`，每次收到新的运行时快照时先同步核心状态，
//! 再构建 Surface/Status/Session/Settings 场景。这样动画队列、提醒状态和展开状态不会
//! 因单次渲染调用丢失。

use std::sync::Mutex;

use chrono::Utc;
use echoisland_runtime::RuntimeSnapshot;

use crate::{
    native_panel_core::PanelState,
    native_panel_scene::{
        build_panel_scene, PanelSceneBuildInput, SessionSurfaceScene, SettingsSurfaceScene,
        StatusSurfaceScene, SurfaceScene,
    },
};

#[derive(Default)]
pub(crate) struct PanelSceneState {
    panel_state: Mutex<PanelState>,
}

impl PanelSceneState {
    pub(crate) fn build_surface_scenes(
        &self,
        raw_snapshot: &RuntimeSnapshot,
        input: &PanelSceneBuildInput,
    ) -> Result<
        (
            SurfaceScene,
            StatusSurfaceScene,
            SessionSurfaceScene,
            SettingsSurfaceScene,
        ),
        String,
    > {
        let mut panel_state = self
            .panel_state
            .lock()
            .map_err(|_| "panel scene state poisoned".to_string())?;
        let sync_result = crate::native_panel_core::sync_panel_snapshot_state(
            &mut panel_state,
            raw_snapshot,
            Utc::now(),
        );
        let scene = build_panel_scene(&panel_state, &sync_result.displayed_snapshot, input);
        Ok((
            scene.surface_scene,
            scene.status_surface,
            scene.session_surface,
            scene.settings_surface,
        ))
    }

    pub(crate) fn build_status_surface_scene(
        &self,
        raw_snapshot: &RuntimeSnapshot,
    ) -> Result<StatusSurfaceScene, String> {
        self.build_surface_scenes(raw_snapshot, &PanelSceneBuildInput::default())
            .map(|(_, status_surface, _, _)| status_surface)
    }
}
