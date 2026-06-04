use serde::Serialize;

use echoisland_runtime::{PendingPermissionView, PendingQuestionView, SessionSnapshotView};

use crate::native_panel_core::{
    ExpandedSurface, PanelHitAction, PanelHitTarget, PanelMascotBaseState, PanelSemanticTarget,
    StatusQueueItem,
};
use crate::native_panel_scene::{
    SessionSurfaceScene, SettingsSurfaceScene, StatusCardScene, SurfaceScene,
};

#[derive(Clone, Debug)]
pub struct PanelScene {
    pub surface: ExpandedSurface,
    pub compact_bar: CompactBarScene,
    pub surface_scene: SurfaceScene,
    pub status_surface: StatusSurfaceScene,
    pub session_surface: SessionSurfaceScene,
    pub settings_surface: SettingsSurfaceScene,
    pub cards: Vec<SceneCard>,
    pub glow: Option<SceneGlow>,
    pub mascot_pose: SceneMascotPose,
    pub debug_mode_enabled: bool,
    pub hit_targets: Vec<SceneHitTarget>,
    pub nodes: Vec<SceneNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompactBarScene {
    pub headline: SceneText,
    pub active_count: String,
    pub total_count: String,
    pub completion_count: usize,
    pub actions_visible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusSurfaceScene {
    pub cards: Vec<StatusCardScene>,
    pub display_mode: StatusSurfaceDisplayMode,
    pub default_state: StatusSurfaceDefaultState,
    pub queue_state: StatusSurfaceQueueState,
    pub completion_badge_count: usize,
    pub show_completion_glow: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusSurfaceDisplayMode {
    Hidden,
    DefaultStack,
    Queue,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusSurfaceDefaultState {
    pub approval_count: usize,
    pub question_count: usize,
    pub prompt_assist_count: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusSurfaceQueueState {
    pub total_count: usize,
    pub live_count: usize,
    pub removing_count: usize,
    pub next_transition_in_ms: Option<u64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PanelShellSceneState {
    pub surface: ExpandedSurface,
    pub headline_emphasized: bool,
    pub edge_actions_visible: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PanelRuntimeRenderState {
    pub transitioning: bool,
    pub shell_scene: PanelShellSceneState,
}

#[derive(Clone, Debug)]
pub enum SceneNode {
    Text(SceneText),
    Badge(SceneBadge),
    Card(SceneCard),
    Glow(SceneGlow),
    Mascot(SceneMascotPose),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SceneText {
    pub text: String,
    pub emphasized: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SceneBadge {
    pub text: String,
    pub emphasized: bool,
}

#[derive(Clone, Debug)]
pub enum SceneCard {
    Settings {
        title: String,
        version: SceneBadge,
        rows: Vec<SettingsRowScene>,
    },
    PendingPermission {
        pending: PendingPermissionView,
        count: usize,
    },
    PendingQuestion {
        pending: PendingQuestionView,
        count: usize,
    },
    PromptAssist {
        session: SessionSnapshotView,
    },
    Session {
        session: SessionSnapshotView,
        title: String,
        status: SceneBadge,
        snippet: Option<String>,
    },
    StatusApproval {
        item: StatusQueueItem,
    },
    StatusQuestion {
        item: StatusQueueItem,
    },
    StatusCompletion {
        item: StatusQueueItem,
    },
    Empty,
}

#[derive(Clone, Copy, Debug)]
pub enum SceneCardHeightInput<'a> {
    Settings { row_count: usize },
    PendingPermission(&'a PendingPermissionView),
    PendingQuestion(&'a PendingQuestionView),
    PromptAssist(&'a SessionSnapshotView),
    Session(&'a SessionSnapshotView),
    StatusItem(&'a StatusQueueItem),
    Empty,
}

pub fn resolve_scene_card_height_input(card: &SceneCard) -> SceneCardHeightInput<'_> {
    match card {
        SceneCard::Settings { rows, .. } => SceneCardHeightInput::Settings {
            row_count: rows.len(),
        },
        SceneCard::PendingPermission { pending, .. } => {
            SceneCardHeightInput::PendingPermission(pending)
        }
        SceneCard::PendingQuestion { pending, .. } => {
            SceneCardHeightInput::PendingQuestion(pending)
        }
        SceneCard::PromptAssist { session } => SceneCardHeightInput::PromptAssist(session),
        SceneCard::Session { session, .. } => SceneCardHeightInput::Session(session),
        SceneCard::StatusApproval { item }
        | SceneCard::StatusQuestion { item }
        | SceneCard::StatusCompletion { item } => SceneCardHeightInput::StatusItem(item),
        SceneCard::Empty => SceneCardHeightInput::Empty,
    }
}

pub fn resolve_scene_cards_total_height(
    scene: &PanelScene,
    resolve_card_height: impl FnMut(&SceneCard) -> f64,
    card_gap: f64,
    empty_height: f64,
) -> f64 {
    let card_heights = scene
        .cards
        .iter()
        .map(resolve_card_height)
        .collect::<Vec<_>>();
    crate::native_panel_core::resolve_stacked_cards_total_height(
        &card_heights,
        card_gap,
        empty_height,
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct SettingsRowScene {
    pub title: String,
    pub value: SceneBadge,
    pub action: PanelHitAction,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SceneGlow {
    pub style: SceneGlowStyle,
    pub opacity: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneGlowStyle {
    Completion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneMascotPose {
    Hidden,
    Idle,
    Running,
    Approval,
    Question,
    MessageBubble,
    Complete,
    Sleepy,
    WakeAngry,
}

pub fn panel_mascot_state_from_scene_pose(pose: SceneMascotPose) -> PanelMascotBaseState {
    match pose {
        SceneMascotPose::Running => PanelMascotBaseState::Running,
        SceneMascotPose::Approval => PanelMascotBaseState::Approval,
        SceneMascotPose::Question => PanelMascotBaseState::Question,
        SceneMascotPose::MessageBubble => PanelMascotBaseState::MessageBubble,
        SceneMascotPose::Complete => PanelMascotBaseState::Complete,
        SceneMascotPose::Sleepy => PanelMascotBaseState::Sleepy,
        SceneMascotPose::WakeAngry => PanelMascotBaseState::WakeAngry,
        SceneMascotPose::Idle | SceneMascotPose::Hidden => PanelMascotBaseState::Idle,
    }
}

pub fn visible_panel_mascot_state_from_scene_pose(
    pose: SceneMascotPose,
) -> Option<PanelMascotBaseState> {
    if pose == SceneMascotPose::Hidden {
        return None;
    }
    Some(panel_mascot_state_from_scene_pose(pose))
}

pub fn scene_mascot_pose_from_panel_state(state: PanelMascotBaseState) -> SceneMascotPose {
    match state {
        PanelMascotBaseState::Idle => SceneMascotPose::Idle,
        PanelMascotBaseState::Running => SceneMascotPose::Running,
        PanelMascotBaseState::Approval => SceneMascotPose::Approval,
        PanelMascotBaseState::Question => SceneMascotPose::Question,
        PanelMascotBaseState::MessageBubble => SceneMascotPose::MessageBubble,
        PanelMascotBaseState::Complete => SceneMascotPose::Complete,
        PanelMascotBaseState::Sleepy => SceneMascotPose::Sleepy,
        PanelMascotBaseState::WakeAngry => SceneMascotPose::WakeAngry,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SceneHitTarget {
    pub action: PanelHitAction,
    pub value: String,
    pub semantic_target: Option<PanelSemanticTarget>,
}

impl From<SceneHitTarget> for PanelHitTarget {
    fn from(value: SceneHitTarget) -> Self {
        Self {
            action: value.action,
            value: value.value,
            semantic_target: value.semantic_target,
        }
    }
}
