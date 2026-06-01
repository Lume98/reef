use std::time::Instant;

use chrono::Utc;
use echoisland_runtime::{
    PendingPermissionView, PendingQuestionView, RuntimeSnapshot, SessionSnapshotView,
};

use super::PanelReminderState;

#[derive(Clone, Debug)]
pub enum StatusQueuePayload {
    Approval(PendingPermissionView),
    Question(PendingQuestionView),
    Completion(SessionSnapshotView),
}

#[derive(Clone, Debug)]
pub struct StatusQueueItem {
    pub key: String,
    pub session_id: String,
    pub sort_time: chrono::DateTime<Utc>,
    pub expires_at: Instant,
    pub is_live: bool,
    pub is_removing: bool,
    pub remove_after: Option<Instant>,
    pub payload: StatusQueuePayload,
}

#[derive(Clone)]
pub struct PendingPermissionCardState {
    pub request_id: String,
    pub payload: PendingPermissionView,
    pub started_at: Instant,
    pub last_seen_at: Instant,
    pub visible_until: Instant,
}

#[derive(Clone)]
pub struct PendingQuestionCardState {
    pub request_id: String,
    pub payload: PendingQuestionView,
    pub started_at: Instant,
    pub last_seen_at: Instant,
    pub visible_until: Instant,
}

#[derive(Clone)]
pub struct CompletionBadgeItem {
    pub session_id: String,
    pub completed_at: chrono::DateTime<Utc>,
    pub last_user_prompt: Option<String>,
    pub last_assistant_message: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompletionReminderEvent {
    Added,
    ViewedByManualExpansion,
    ViewedBySettings,
    ClearedByNewDialogue,
    StatusCardExpired,
}

#[derive(Clone, Copy, Default)]
pub struct StatusQueueSyncResult {
    pub added_approvals: usize,
    pub added_questions: usize,
    pub added_completions: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelHitAction {
    FocusSession,
    CycleDisplay,
    CycleIslandWidth,
    CycleLanguage,
    ToggleCompletionSound,
    ToggleMascot,
    OpenSettingsLocation,
    OpenReleasePage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ExpandedSurface {
    #[default]
    Default,
    Status,
    Settings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverTransition {
    Expand,
    Collapse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PanelMascotBaseState {
    #[default]
    Idle,
    Running,
    Approval,
    Question,
    MessageBubble,
    Complete,
    Sleepy,
    WakeAngry,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelTransitionFrame {
    pub canvas_height: f64,
    pub visible_height: f64,
    pub bar_progress: f64,
    pub height_progress: f64,
    pub shoulder_progress: f64,
    pub drop_progress: f64,
    pub cards_progress: f64,
}

impl PanelTransitionFrame {
    pub fn expanded(height: f64) -> Self {
        Self {
            canvas_height: height,
            visible_height: height,
            bar_progress: 1.0,
            height_progress: 1.0,
            shoulder_progress: 1.0,
            drop_progress: 1.0,
            cards_progress: 1.0,
        }
    }

    pub fn collapsed(height: f64) -> Self {
        Self {
            canvas_height: height,
            visible_height: height,
            bar_progress: 0.0,
            height_progress: 0.0,
            shoulder_progress: 0.0,
            drop_progress: 0.0,
            cards_progress: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusSurfaceTransition {
    pub panel_transition: Option<bool>,
    pub surface_transition: bool,
}

#[derive(Clone, Debug)]
pub struct PanelSnapshotSyncResult {
    pub displayed_snapshot: RuntimeSnapshot,
    pub reminder: PanelReminderState,
    pub panel_transition: Option<bool>,
    pub surface_transition: bool,
}

#[derive(Clone, Default)]
pub struct PanelState {
    pub expanded: bool,
    pub transitioning: bool,
    pub skip_next_close_card_exit: bool,
    pub last_raw_snapshot: Option<RuntimeSnapshot>,
    pub status_queue: Vec<StatusQueueItem>,
    pub completion_badge_items: Vec<CompletionBadgeItem>,
    pub pending_permission_card: Option<PendingPermissionCardState>,
    pub pending_question_card: Option<PendingQuestionCardState>,
    pub status_auto_expanded: bool,
    pub surface_mode: ExpandedSurface,
    pub pointer_inside_since: Option<Instant>,
    pub pointer_outside_since: Option<Instant>,
}
