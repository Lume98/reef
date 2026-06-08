use crate::{
    scene::SceneMascotPose,
    state::{ExpandedSurface, MascotVisualFrame, PanelRect},
};

use super::super::descriptors::{NativePanelEdgeAction, NativePanelHostWindowState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativePanelVisualDisplayMode {
    Hidden,
    Compact,
    Expanded,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativePanelPaintInput {
    pub window_state: NativePanelHostWindowState,
    pub display_mode: NativePanelVisualDisplayMode,
    pub surface: ExpandedSurface,
    pub panel_frame: PanelRect,
    pub compact_bar_frame: PanelRect,
    pub left_shoulder_frame: PanelRect,
    pub right_shoulder_frame: PanelRect,
    pub shoulder_progress: f64,
    pub content_frame: PanelRect,
    pub card_stack_frame: PanelRect,
    pub card_stack_content_height: f64,
    pub shell_frame: PanelRect,
    pub headline_text: String,
    pub headline_emphasized: bool,
    pub active_count: String,
    pub active_count_elapsed_ms: u128,
    pub total_count: String,
    pub separator_visibility: f64,
    pub chrome_transition_progress: f64,
    pub cards_visible: bool,
    pub card_count: usize,
    pub cards: Vec<NativePanelVisualCardInput>,
    pub glow_visible: bool,
    pub glow_opacity: f64,
    pub action_buttons_visible: bool,
    pub action_buttons: Vec<NativePanelVisualActionButtonInput>,
    pub completion_count: usize,
    pub mascot_elapsed_ms: u128,
    pub mascot_motion_frame: Option<MascotVisualFrame>,
    pub mascot_pose: SceneMascotPose,
    pub mascot_debug_mode_enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativePanelVisualCardInput {
    pub style: NativePanelVisualCardStyle,
    pub title: String,
    pub subtitle: Option<String>,
    pub body: Option<String>,
    pub badge: Option<NativePanelVisualCardBadgeInput>,
    pub source_badge: Option<NativePanelVisualCardBadgeInput>,
    pub body_prefix: Option<String>,
    pub body_lines: Vec<NativePanelVisualCardBodyLineInput>,
    pub action_hint: Option<String>,
    pub rows: Vec<NativePanelVisualCardRowInput>,
    pub height: f64,
    pub collapsed_height: f64,
    pub compact: bool,
    pub removing: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativePanelVisualCardStyle {
    Default,
    Pending,
    PendingApproval,
    PendingQuestion,
    PromptAssist,
    Completion,
    Settings,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativePanelVisualCardBodyRole {
    Assistant,
    User,
    Tool,
    Plain,
    ActionHint,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativePanelVisualCardBodyLineInput {
    pub role: NativePanelVisualCardBodyRole,
    pub prefix: Option<String>,
    pub text: String,
    pub max_lines: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativePanelVisualCardBadgeInput {
    pub text: String,
    pub emphasized: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativePanelVisualCardRowInput {
    pub title: String,
    pub value: String,
    pub active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelVisualActionButtonInput {
    pub action: NativePanelEdgeAction,
    pub frame: PanelRect,
    pub debug_mode_enabled: bool,
}
