use serde::Serialize;

use crate::state::ExpandedSurface;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceScene {
    pub mode: SurfaceSceneMode,
    pub headline_text: String,
    pub headline_emphasized: bool,
    pub edge_actions_visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceSceneMode {
    Default,
    Status,
    Settings,
}

pub fn surface_scene_mode(surface: ExpandedSurface) -> SurfaceSceneMode {
    match surface {
        ExpandedSurface::Default => SurfaceSceneMode::Default,
        ExpandedSurface::Status => SurfaceSceneMode::Status,
        ExpandedSurface::Settings => SurfaceSceneMode::Settings,
    }
}
