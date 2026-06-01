use echoisland_runtime::RuntimeSnapshot;

use super::{
    resolve_mascot_base_state, ExpandedSurface, PanelMascotBaseState, PanelState,
    StatusQueuePayload,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PanelReminderState {
    pub completion_badge_count: usize,
    pub has_status_completion: bool,
    pub show_status_card: bool,
    pub show_completion_glow: bool,
    pub play_sound: bool,
    pub mascot_base_state: PanelMascotBaseState,
}

pub fn resolve_panel_reminder_state(
    state: &PanelState,
    snapshot: Option<&RuntimeSnapshot>,
) -> PanelReminderState {
    let completion_badge_count = state.completion_badge_items.len();
    let has_status_completion = state.expanded
        && state.surface_mode == ExpandedSurface::Status
        && state
            .status_queue
            .iter()
            .any(|item| matches!(item.payload, StatusQueuePayload::Completion(_)));

    PanelReminderState {
        completion_badge_count,
        has_status_completion,
        show_status_card: !state.status_queue.is_empty(),
        show_completion_glow: completion_badge_count > 0 && !state.expanded,
        play_sound: false,
        mascot_base_state: resolve_mascot_base_state(
            snapshot,
            has_status_completion,
            completion_badge_count > 0,
        ),
    }
}

pub fn resolve_panel_sync_reminder_state(
    state: &PanelState,
    snapshot: Option<&RuntimeSnapshot>,
    play_sound: bool,
) -> PanelReminderState {
    PanelReminderState {
        play_sound,
        ..resolve_panel_reminder_state(state, snapshot)
    }
}
