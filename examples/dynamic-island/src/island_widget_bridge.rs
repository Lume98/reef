//! 桥接模块：将 `RuntimeSnapshot` 转换为声明式 `DynamicIsland`，再下沉为 `IslandWidget`。
//!
//! 这里不再构造专用的 `IslandWidgetContentInput`，而是直接组合已有组件，并绑定业务动作。

use crate::native_panel_core::{PanelHitAction, PanelHitTarget};
use crate::native_panel_renderer::facade::{
    command::NativePanelPlatformEvent, transition::NativePanelTransitionRequest,
};
use echoisland_runtime::RuntimeSnapshot;
use reef_widgets::{
    Badge, BodyLine, Card, CardStyle, ChromeVisibility, CompactBar, DynamicIsland,
    DynamicIslandGesture, IslandWidget, MascotPose, MascotWidget, SettingsRow,
};

pub use reef_native_panel_core::island_render_overrides;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicIslandRuntimeAction {
    OpenPrimarySession,
    OpenSession(String),
    ToggleSettings,
    Dismiss,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicIslandRuntimeEffect {
    PlatformEvent(NativePanelPlatformEvent),
    Transition(NativePanelTransitionRequest),
}

pub fn build_dynamic_island(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> DynamicIsland<DynamicIslandRuntimeAction> {
    let mut island = DynamicIsland::new()
        .child(build_compact_bar(snapshot, panel_expanded, settings_active))
        .on_click(DynamicIslandRuntimeAction::OpenPrimarySession)
        .on_swipe(DynamicIslandRuntimeAction::Dismiss);

    if settings_active {
        island = island.child(build_settings_card());
    } else {
        for (card, action_target) in build_runtime_cards(snapshot) {
            island = island.child(card);
            if let Some((target, action)) = action_target {
                island = island.on_target_click(target, action);
            }
        }
    }

    if let Some(mascot) = build_mascot(snapshot, panel_expanded, settings_active) {
        island = island.child(mascot);
    }

    island
}

pub fn build_island_widget(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> IslandWidget {
    build_dynamic_island(snapshot, panel_expanded, settings_active).into_widget()
}

pub fn resolve_dynamic_island_action(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
    gesture: DynamicIslandGesture,
) -> Option<DynamicIslandRuntimeAction> {
    build_dynamic_island(snapshot, panel_expanded, settings_active)
        .action_for_gesture(gesture)
        .cloned()
}

pub fn resolve_dynamic_island_target_action(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
    target: &str,
    gesture: DynamicIslandGesture,
) -> Option<DynamicIslandRuntimeAction> {
    build_dynamic_island(snapshot, panel_expanded, settings_active)
        .action_for_target_gesture(target, gesture)
        .cloned()
}

pub fn resolve_dynamic_island_effect(
    snapshot: &RuntimeSnapshot,
    action: DynamicIslandRuntimeAction,
) -> Option<DynamicIslandRuntimeEffect> {
    match action {
        DynamicIslandRuntimeAction::OpenPrimarySession => {
            resolve_primary_session_id(snapshot).map(|session_id| {
                DynamicIslandRuntimeEffect::PlatformEvent(NativePanelPlatformEvent::FocusSession(
                    session_id,
                ))
            })
        }
        DynamicIslandRuntimeAction::OpenSession(session_id) => {
            Some(DynamicIslandRuntimeEffect::PlatformEvent(
                NativePanelPlatformEvent::FocusSession(session_id),
            ))
        }
        DynamicIslandRuntimeAction::ToggleSettings => {
            Some(DynamicIslandRuntimeEffect::PlatformEvent(
                NativePanelPlatformEvent::ToggleSettingsSurface,
            ))
        }
        DynamicIslandRuntimeAction::Dismiss => Some(DynamicIslandRuntimeEffect::Transition(
            NativePanelTransitionRequest::Close,
        )),
    }
}

pub fn resolve_dynamic_island_gesture_effect(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
    gesture: DynamicIslandGesture,
) -> Option<DynamicIslandRuntimeEffect> {
    let action = resolve_dynamic_island_action(snapshot, panel_expanded, settings_active, gesture)?;
    resolve_dynamic_island_effect(snapshot, action)
}

pub fn resolve_dynamic_island_target_effect(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
    target: &str,
    gesture: DynamicIslandGesture,
) -> Option<DynamicIslandRuntimeEffect> {
    let action = resolve_dynamic_island_target_action(
        snapshot,
        panel_expanded,
        settings_active,
        target,
        gesture,
    )?;
    resolve_dynamic_island_effect(snapshot, action)
}

pub fn dynamic_island_target_key_for_hit_target(target: &PanelHitTarget) -> Option<String> {
    match target.action {
        PanelHitAction::FocusSession if !target.value.is_empty() => {
            Some(format!("session:{}", target.value))
        }
        _ => None,
    }
}

pub fn resolve_dynamic_island_platform_event(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
    gesture: DynamicIslandGesture,
) -> Option<NativePanelPlatformEvent> {
    match resolve_dynamic_island_gesture_effect(snapshot, panel_expanded, settings_active, gesture)?
    {
        DynamicIslandRuntimeEffect::PlatformEvent(event) => Some(event),
        DynamicIslandRuntimeEffect::Transition(_) => None,
    }
}

pub fn resolve_dynamic_island_transition_request(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
    gesture: DynamicIslandGesture,
) -> Option<NativePanelTransitionRequest> {
    match resolve_dynamic_island_gesture_effect(snapshot, panel_expanded, settings_active, gesture)?
    {
        DynamicIslandRuntimeEffect::PlatformEvent(_) => None,
        DynamicIslandRuntimeEffect::Transition(request) => Some(request),
    }
}

fn build_compact_bar(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> CompactBar {
    let chrome = if panel_expanded {
        ChromeVisibility::expanded()
    } else {
        ChromeVisibility::compact()
    };

    CompactBar::new()
        .headline("Reef")
        .headline_emphasized(panel_expanded)
        .counts(
            snapshot.active_session_count.to_string(),
            snapshot.total_session_count.to_string(),
        )
        .show_actions(panel_expanded || settings_active)
        .chrome(chrome)
}

fn build_settings_card() -> Card {
    Card::new(CardStyle::Settings)
        .title("Settings")
        .subtitle("Dynamic Island")
        .settings_rows(vec![
            SettingsRow {
                title: "Display".to_string(),
                value: "Cycle".to_string(),
                active: true,
            },
            SettingsRow {
                title: "Width".to_string(),
                value: "Adaptive".to_string(),
                active: false,
            },
            SettingsRow {
                title: "Language".to_string(),
                value: "EN/ZH/JA".to_string(),
                active: false,
            },
        ])
        .height(146.0)
}

fn resolve_primary_session_id(snapshot: &RuntimeSnapshot) -> Option<String> {
    snapshot
        .sessions
        .first()
        .map(|session| session.session_id.clone())
        .or_else(|| {
            snapshot
                .pending_permissions
                .first()
                .map(|pending| pending.session_id.clone())
        })
        .or_else(|| {
            snapshot
                .pending_questions
                .first()
                .map(|pending| pending.session_id.clone())
        })
}

fn build_runtime_cards(
    snapshot: &RuntimeSnapshot,
) -> Vec<(Card, Option<(String, DynamicIslandRuntimeAction)>)> {
    let mut cards = Vec::new();

    for pending in &snapshot.pending_permissions {
        let mut card = Card::new(CardStyle::PendingApproval)
            .title(format!("{} wants permission", pending.source))
            .badge(Badge::status("Approval", true))
            .badge(Badge::source(pending.source.clone()));
        if let Some(tool) = &pending.tool_description {
            card = card.body_line(BodyLine::plain(Some("$"), tool.clone()));
        }
        let session_id = pending.session_id.clone();
        cards.push((
            card.height(104.0),
            Some((
                format!("session:{session_id}"),
                DynamicIslandRuntimeAction::OpenSession(session_id),
            )),
        ));
    }

    for pending in &snapshot.pending_questions {
        let title = pending
            .header
            .clone()
            .unwrap_or_else(|| format!("{} asks a question", pending.source));
        let session_id = pending.session_id.clone();
        cards.push((
            Card::new(CardStyle::PendingQuestion)
                .title(title)
                .badge(Badge::status("Question", true))
                .badge(Badge::source(pending.source.clone()))
                .body_line(BodyLine::plain(None, pending.text.clone()))
                .height(112.0),
            Some((
                format!("session:{session_id}"),
                DynamicIslandRuntimeAction::OpenSession(session_id),
            )),
        ));
    }

    for session in &snapshot.sessions {
        let mut card = Card::new(CardStyle::Default)
            .title(session.source.clone())
            .badge(Badge::status(
                session.status.clone(),
                session.status.eq_ignore_ascii_case("running"),
            ))
            .badge(Badge::source(session.source.clone()));

        if let Some(message) = session
            .last_assistant_message
            .clone()
            .or(session.last_user_prompt.clone())
        {
            card = card.body_line(BodyLine::plain(None, message));
        }
        if let Some(tool) = &session.current_tool {
            card = card.tool(tool.clone(), session.tool_description.clone());
        }
        let session_id = session.session_id.clone();
        cards.push((
            card.height(96.0),
            Some((
                format!("session:{session_id}"),
                DynamicIslandRuntimeAction::OpenSession(session_id),
            )),
        ));
    }

    if cards.is_empty() {
        cards.push((
            Card::new(CardStyle::Empty)
                .title("No active sessions")
                .body_line(BodyLine::plain(None, "Reef is waiting for the next event."))
                .height(88.0),
            None,
        ));
    }

    cards
}

fn build_mascot(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> Option<MascotWidget> {
    if !panel_expanded && snapshot.active_session_count == 0 && !settings_active {
        return None;
    }

    let pose = if settings_active {
        MascotPose::Idle
    } else if !snapshot.pending_permissions.is_empty() {
        MascotPose::Approval
    } else if !snapshot.pending_questions.is_empty() {
        MascotPose::Question
    } else if snapshot.active_session_count > 0 {
        MascotPose::Running
    } else {
        MascotPose::Idle
    };

    Some(MascotWidget::new(200.0, 24.0, 14.0).pose(pose))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_snapshot() -> RuntimeSnapshot {
        RuntimeSnapshot {
            status: "idle".to_string(),
            primary_source: "reef".to_string(),
            active_session_count: 0,
            total_session_count: 0,
            pending_permission_count: 0,
            pending_question_count: 0,
            pending_permission: None,
            pending_question: None,
            pending_permissions: vec![],
            pending_questions: vec![],
            sessions: vec![],
        }
    }

    #[test]
    fn bridge_builds_declarative_dynamic_island() {
        let island = build_dynamic_island(&empty_snapshot(), true, false);

        assert_eq!(island.bindings().len(), 2);
        assert_eq!(
            island.action_for_gesture(DynamicIslandGesture::Click),
            Some(&DynamicIslandRuntimeAction::OpenPrimarySession)
        );
    }

    #[test]
    fn bridge_resolves_runtime_action_from_target_click() {
        let mut snapshot = empty_snapshot();
        snapshot
            .sessions
            .push(echoisland_runtime::SessionSnapshotView {
                session_id: "session-1".to_string(),
                source: "reef".to_string(),
                project_name: None,
                cwd: None,
                model: None,
                terminal_app: None,
                terminal_bundle: None,
                host_app: None,
                window_title: None,
                tty: None,
                terminal_pid: None,
                cli_pid: None,
                iterm_session_id: None,
                kitty_window_id: None,
                tmux_env: None,
                tmux_pane: None,
                tmux_client_tty: None,
                status: "running".to_string(),
                current_tool: None,
                tool_description: None,
                last_user_prompt: None,
                last_assistant_message: None,
                tool_history_count: 0,
                tool_history: vec![],
                last_activity: chrono::Utc::now(),
            });

        let action = resolve_dynamic_island_target_action(
            &snapshot,
            true,
            false,
            "session:session-1",
            DynamicIslandGesture::Click,
        );

        assert_eq!(
            action,
            Some(DynamicIslandRuntimeAction::OpenSession(
                "session-1".to_string()
            ))
        );
    }

    #[test]
    fn bridge_resolves_runtime_action_from_gesture() {
        let action = resolve_dynamic_island_action(
            &empty_snapshot(),
            true,
            false,
            DynamicIslandGesture::Swipe,
        );

        assert_eq!(action, Some(DynamicIslandRuntimeAction::Dismiss));
    }

    #[test]
    fn bridge_maps_hit_target_to_dynamic_island_target_key() {
        let key = dynamic_island_target_key_for_hit_target(&PanelHitTarget {
            action: PanelHitAction::FocusSession,
            value: "session-1".to_string(),
        });

        assert_eq!(key, Some("session:session-1".to_string()));
    }

    #[test]
    fn bridge_builds_widget_from_dynamic_island() {
        let widget = build_island_widget(&empty_snapshot(), false, false);

        assert_eq!(widget.compact_bar.headline, "Reef");
        assert_eq!(widget.cards.len(), 1);
    }

    #[test]
    fn bridge_resolves_runtime_effect_for_primary_session() {
        let mut snapshot = empty_snapshot();
        snapshot
            .sessions
            .push(echoisland_runtime::SessionSnapshotView {
                session_id: "session-1".to_string(),
                source: "reef".to_string(),
                project_name: None,
                cwd: None,
                model: None,
                terminal_app: None,
                terminal_bundle: None,
                host_app: None,
                window_title: None,
                tty: None,
                terminal_pid: None,
                cli_pid: None,
                iterm_session_id: None,
                kitty_window_id: None,
                tmux_env: None,
                tmux_pane: None,
                tmux_client_tty: None,
                status: "running".to_string(),
                current_tool: None,
                tool_description: None,
                last_user_prompt: None,
                last_assistant_message: None,
                tool_history_count: 0,
                tool_history: vec![],
                last_activity: chrono::Utc::now(),
            });

        let effect = resolve_dynamic_island_effect(
            &snapshot,
            DynamicIslandRuntimeAction::OpenPrimarySession,
        );

        assert_eq!(
            effect,
            Some(DynamicIslandRuntimeEffect::PlatformEvent(
                NativePanelPlatformEvent::FocusSession("session-1".to_string())
            ))
        );
    }

    #[test]
    fn bridge_resolves_runtime_effect_for_dismiss() {
        let effect =
            resolve_dynamic_island_effect(&empty_snapshot(), DynamicIslandRuntimeAction::Dismiss);

        assert_eq!(
            effect,
            Some(DynamicIslandRuntimeEffect::Transition(
                NativePanelTransitionRequest::Close
            ))
        );
    }

    #[test]
    fn bridge_resolves_platform_event_from_click_gesture() {
        let mut snapshot = empty_snapshot();
        snapshot
            .sessions
            .push(echoisland_runtime::SessionSnapshotView {
                session_id: "session-1".to_string(),
                source: "reef".to_string(),
                project_name: None,
                cwd: None,
                model: None,
                terminal_app: None,
                terminal_bundle: None,
                host_app: None,
                window_title: None,
                tty: None,
                terminal_pid: None,
                cli_pid: None,
                iterm_session_id: None,
                kitty_window_id: None,
                tmux_env: None,
                tmux_pane: None,
                tmux_client_tty: None,
                status: "running".to_string(),
                current_tool: None,
                tool_description: None,
                last_user_prompt: None,
                last_assistant_message: None,
                tool_history_count: 0,
                tool_history: vec![],
                last_activity: chrono::Utc::now(),
            });

        let event = resolve_dynamic_island_platform_event(
            &snapshot,
            false,
            false,
            DynamicIslandGesture::Click,
        );

        assert_eq!(
            event,
            Some(NativePanelPlatformEvent::FocusSession(
                "session-1".to_string()
            ))
        );
    }

    #[test]
    fn bridge_resolves_gesture_effect_from_click_gesture() {
        let mut snapshot = empty_snapshot();
        snapshot
            .sessions
            .push(echoisland_runtime::SessionSnapshotView {
                session_id: "session-1".to_string(),
                source: "reef".to_string(),
                project_name: None,
                cwd: None,
                model: None,
                terminal_app: None,
                terminal_bundle: None,
                host_app: None,
                window_title: None,
                tty: None,
                terminal_pid: None,
                cli_pid: None,
                iterm_session_id: None,
                kitty_window_id: None,
                tmux_env: None,
                tmux_pane: None,
                tmux_client_tty: None,
                status: "running".to_string(),
                current_tool: None,
                tool_description: None,
                last_user_prompt: None,
                last_assistant_message: None,
                tool_history_count: 0,
                tool_history: vec![],
                last_activity: chrono::Utc::now(),
            });

        let effect = resolve_dynamic_island_gesture_effect(
            &snapshot,
            false,
            false,
            DynamicIslandGesture::Click,
        );

        assert_eq!(
            effect,
            Some(DynamicIslandRuntimeEffect::PlatformEvent(
                NativePanelPlatformEvent::FocusSession("session-1".to_string())
            ))
        );
    }

    #[test]
    fn bridge_resolves_transition_request_from_swipe_gesture() {
        let request = resolve_dynamic_island_transition_request(
            &empty_snapshot(),
            true,
            false,
            DynamicIslandGesture::Swipe,
        );

        assert_eq!(request, Some(NativePanelTransitionRequest::Close));
    }
}
