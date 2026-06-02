use std::time::Instant;

use echoisland_runtime::RuntimeSnapshot;

use crate::native_panel_core::{
    compact_active_session_count, display_snippet, displayed_default_pending_permissions,
    displayed_default_pending_questions, displayed_prompt_assist_sessions, displayed_sessions,
    resolve_panel_reminder_state, ExpandedSurface, PanelMascotBaseState, PanelState,
    StatusQueuePayload,
};

use super::{
    build_pending_permission_status_card_scene, build_pending_question_status_card_scene,
    build_prompt_assist_status_card_scene, build_session_card_scene,
    build_status_queue_status_card_scene, settings_surface_row_action, CompactBarScene, SceneBadge,
    SceneCard, SceneGlow, SceneGlowStyle, SceneMascotPose, SceneText, SessionSurfaceScene,
    SettingsRowScene, SettingsSurfaceScene, StatusSurfaceDefaultState, StatusSurfaceDisplayMode,
    StatusSurfaceQueueState, StatusSurfaceScene, SurfaceScene,
};

pub(crate) fn build_surface_scene(
    state: &PanelState,
    compact_bar: &CompactBarScene,
) -> SurfaceScene {
    SurfaceScene {
        mode: super::surface_scene_mode(state.surface_mode),
        headline_text: compact_bar.headline.text.clone(),
        headline_emphasized: compact_bar.headline.emphasized,
        edge_actions_visible: compact_bar.actions_visible,
    }
}

pub(crate) fn build_session_surface_scene(
    state: &PanelState,
    snapshot: &RuntimeSnapshot,
) -> SessionSurfaceScene {
    let prompt_assist_sessions = displayed_prompt_assist_sessions(snapshot);
    let completion_session_ids = state
        .status_queue
        .iter()
        .filter_map(|item| match &item.payload {
            StatusQueuePayload::Completion(_) => Some(item.session_id.as_str()),
            _ => None,
        })
        .collect::<std::collections::HashSet<_>>();

    SessionSurfaceScene {
        cards: displayed_sessions(snapshot, &prompt_assist_sessions)
            .into_iter()
            .map(|session| {
                let completion = completion_session_ids.contains(session.session_id.as_str());
                build_session_card_scene(&session, completion)
            })
            .collect(),
    }
}

pub(crate) fn build_status_surface_scene(
    state: &PanelState,
    snapshot: &RuntimeSnapshot,
) -> StatusSurfaceScene {
    let reminder = resolve_panel_reminder_state(state, Some(snapshot));
    if reminder.show_status_card {
        let now = Instant::now();
        return StatusSurfaceScene {
            cards: state
                .status_queue
                .iter()
                .map(build_status_queue_status_card_scene)
                .collect(),
            display_mode: StatusSurfaceDisplayMode::Queue,
            default_state: StatusSurfaceDefaultState::default(),
            queue_state: StatusSurfaceQueueState {
                total_count: state.status_queue.len(),
                live_count: state
                    .status_queue
                    .iter()
                    .filter(|item| item.is_live)
                    .count(),
                removing_count: state
                    .status_queue
                    .iter()
                    .filter(|item| item.is_removing)
                    .count(),
                next_transition_in_ms: state
                    .status_queue
                    .iter()
                    .filter_map(|item| {
                        if item.is_removing {
                            item.remove_after
                        } else {
                            Some(item.expires_at)
                        }
                    })
                    .filter(|transition_at| *transition_at > now)
                    .map(|transition_at| {
                        transition_at
                            .saturating_duration_since(now)
                            .as_millis()
                            .min(u64::MAX as u128) as u64
                    })
                    .min(),
            },
            completion_badge_count: reminder.completion_badge_count,
            show_completion_glow: reminder.show_completion_glow,
        };
    }

    let mut cards = Vec::new();
    cards.extend(
        displayed_default_pending_permissions(snapshot)
            .into_iter()
            .take(1)
            .map(|pending| build_pending_permission_status_card_scene(&pending)),
    );
    cards.extend(
        displayed_default_pending_questions(snapshot)
            .into_iter()
            .take(1)
            .map(|pending| build_pending_question_status_card_scene(&pending)),
    );
    cards.extend(
        displayed_prompt_assist_sessions(snapshot)
            .into_iter()
            .map(|session| build_prompt_assist_status_card_scene(&session)),
    );

    StatusSurfaceScene {
        display_mode: if cards.is_empty() {
            StatusSurfaceDisplayMode::Hidden
        } else {
            StatusSurfaceDisplayMode::DefaultStack
        },
        cards,
        default_state: StatusSurfaceDefaultState {
            approval_count: snapshot.pending_permission_count,
            question_count: snapshot.pending_question_count,
            prompt_assist_count: displayed_prompt_assist_sessions(snapshot).len(),
        },
        queue_state: StatusSurfaceQueueState::default(),
        completion_badge_count: reminder.completion_badge_count,
        show_completion_glow: reminder.show_completion_glow,
    }
}

pub(crate) fn build_compact_bar_scene(
    state: &PanelState,
    snapshot: &RuntimeSnapshot,
) -> CompactBarScene {
    let active_count = compact_active_session_count(snapshot);
    let reminder = resolve_panel_reminder_state(state, Some(snapshot));
    let actions_visible =
        (state.expanded || state.transitioning) && state.surface_mode != ExpandedSurface::Status;
    CompactBarScene {
        headline: SceneText {
            text: compact_headline(state, snapshot),
            emphasized: !state.status_queue.is_empty(),
        },
        active_count: if active_count == 0 {
            "0".to_string()
        } else {
            active_count.to_string()
        },
        total_count: snapshot.total_session_count.to_string(),
        completion_count: reminder.completion_badge_count,
        actions_visible,
    }
}

pub(crate) fn build_completion_glow(state: &PanelState) -> Option<SceneGlow> {
    if !resolve_panel_reminder_state(state, None).show_completion_glow {
        return None;
    }
    Some(SceneGlow {
        style: SceneGlowStyle::Completion,
        opacity: 0.78,
    })
}

pub(crate) fn build_mascot_pose(
    state: &PanelState,
    snapshot: &RuntimeSnapshot,
    mascot_enabled: bool,
) -> SceneMascotPose {
    if !mascot_enabled {
        return SceneMascotPose::Hidden;
    }
    let mascot_base_state = resolve_panel_reminder_state(state, Some(snapshot)).mascot_base_state;
    let mascot_base_state = if state.surface_mode == ExpandedSurface::Status
        && mascot_base_state == PanelMascotBaseState::MessageBubble
    {
        PanelMascotBaseState::Complete
    } else {
        mascot_base_state
    };
    match mascot_base_state {
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

pub(crate) fn build_settings_card(settings_surface: &SettingsSurfaceScene) -> SceneCard {
    SceneCard::Settings {
        title: settings_surface.title.clone(),
        version: SceneBadge {
            text: settings_surface
                .version_text
                .strip_prefix("Reef UI ")
                .unwrap_or(&settings_surface.version_text)
                .to_string(),
            emphasized: false,
        },
        rows: settings_surface
            .rows
            .iter()
            .enumerate()
            .filter_map(|(index, row)| {
                Some(SettingsRowScene {
                    title: row.label.clone(),
                    value: SceneBadge {
                        text: row.value_text.clone(),
                        emphasized: row.checked.unwrap_or(false),
                    },
                    action: settings_surface_row_action(index)?,
                })
            })
            .collect(),
    }
}

pub(crate) fn settings_rows(card: &SceneCard) -> &[SettingsRowScene] {
    match card {
        SceneCard::Settings { rows, .. } => rows.as_slice(),
        _ => &[],
    }
}

pub(crate) fn compact_headline(state: &PanelState, snapshot: &RuntimeSnapshot) -> String {
    let approval_count = state
        .status_queue
        .iter()
        .filter(|item| {
            matches!(
                item.payload,
                StatusQueuePayload::Approval(_) | StatusQueuePayload::Question(_)
            )
        })
        .count();
    if approval_count > 0 {
        return if approval_count > 1 {
            "Requests waiting".to_string()
        } else {
            match state.status_queue.first().map(|item| &item.payload) {
                Some(StatusQueuePayload::Question(_)) => "Question waiting".to_string(),
                _ => "Approval waiting".to_string(),
            }
        };
    }

    let completion_count = state
        .status_queue
        .iter()
        .filter(|item| matches!(item.payload, StatusQueuePayload::Completion(_)))
        .count();
    if completion_count > 1 {
        return format!("{completion_count} tasks complete");
    }
    if completion_count == 1 {
        if let Some(StatusQueuePayload::Completion(session)) =
            state.status_queue.first().map(|item| &item.payload)
        {
            return display_snippet(session.last_assistant_message.as_deref(), 42)
                .unwrap_or_else(|| "Task complete".to_string());
        }
    }

    let active_count = compact_active_session_count(snapshot);
    if active_count > 0 {
        format!(
            "{} active task{}",
            active_count,
            if active_count > 1 { "s" } else { "" }
        )
    } else {
        "No active tasks".to_string()
    }
}
