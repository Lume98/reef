//! 桥接模块：将 `RuntimeSnapshot` 转换为声明式 `DynamicIsland`，再下沉为 `IslandWidget`。
//!
//! 这里不再构造专用的 `IslandWidgetContentInput`，而是直接组合已有组件，并绑定业务动作。

use crate::native_panel_core::PanelHitTarget;
use crate::native_panel_renderer::facade::{
    command::NativePanelPlatformEvent, transition::NativePanelTransitionRequest,
};
use echoisland_runtime::RuntimeSnapshot;
use reef_widgets::{
    Badge, BodyLine, Card, CardStyle, ChromeVisibility, CompactBar, DynamicIsland,
    DynamicIslandGesture, DynamicIslandTarget, IslandWidget, MascotPose, MascotWidget, SettingsRow,
};

pub use reef_native_panel_core::island_render_overrides;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicIslandRuntimeAction {
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
        .on_swipe(DynamicIslandRuntimeAction::Dismiss);

    if settings_active {
        island = island.child(build_settings_card());
    } else {
        for card in build_runtime_cards(snapshot) {
            island = island.child(card);
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
    target: &DynamicIslandTarget,
    gesture: DynamicIslandGesture,
) -> Option<DynamicIslandRuntimeAction> {
    build_dynamic_island(snapshot, panel_expanded, settings_active)
        .action_for_target_gesture(target, gesture)
        .cloned()
}

pub fn resolve_dynamic_island_effect(
    _snapshot: &RuntimeSnapshot,
    action: DynamicIslandRuntimeAction,
) -> Option<DynamicIslandRuntimeEffect> {
    match action {
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
    target: &DynamicIslandTarget,
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

pub fn dynamic_island_target_for_hit_target(
    _target: &PanelHitTarget,
) -> Option<DynamicIslandTarget> {
    None
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

fn build_runtime_cards(snapshot: &RuntimeSnapshot) -> Vec<Card> {
    let mut cards = Vec::new();

    for pending in &snapshot.pending_permissions {
        let mut card = Card::new(CardStyle::PendingApproval)
            .title(format!("{} wants permission", pending.source))
            .badge(Badge::status("Approval", true))
            .badge(Badge::source(pending.source.clone()));
        if let Some(tool) = &pending.tool_description {
            card = card.body_line(BodyLine::plain(Some("$"), tool.clone()));
        }
        cards.push(card.height(104.0));
    }

    for pending in &snapshot.pending_questions {
        let title = pending
            .header
            .clone()
            .unwrap_or_else(|| format!("{} asks a question", pending.source));
        cards.push(
            Card::new(CardStyle::PendingQuestion)
                .title(title)
                .badge(Badge::status("Question", true))
                .badge(Badge::source(pending.source.clone()))
                .body_line(BodyLine::plain(None, pending.text.clone()))
                .height(112.0),
        );
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
        cards.push(card.height(96.0));
    }

    if cards.is_empty() {
        cards.push(
            Card::new(CardStyle::Empty)
                .title("No active sessions")
                .body_line(BodyLine::plain(None, "Reef is waiting for the next event."))
                .height(88.0),
        );
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

        assert_eq!(island.bindings().len(), 1);
        assert_eq!(island.action_for_gesture(DynamicIslandGesture::Click), None);
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
    fn bridge_maps_hit_target_to_dynamic_island_target() {
        let key = dynamic_island_target_for_hit_target(&PanelHitTarget::focus_session("session-1"));

        assert_eq!(key, None);
    }

    #[test]
    fn bridge_builds_widget_from_dynamic_island() {
        let widget = build_island_widget(&empty_snapshot(), false, false);

        assert_eq!(widget.compact_bar.headline, "Reef");
        assert_eq!(widget.cards.len(), 1);
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
    fn bridge_does_not_resolve_platform_event_from_click_gesture() {
        let event = resolve_dynamic_island_platform_event(
            &empty_snapshot(),
            false,
            false,
            DynamicIslandGesture::Click,
        );

        assert_eq!(event, None);
    }

    #[test]
    fn bridge_does_not_resolve_gesture_effect_from_click_gesture() {
        let effect = resolve_dynamic_island_gesture_effect(
            &empty_snapshot(),
            false,
            false,
            DynamicIslandGesture::Click,
        );

        assert_eq!(effect, None);
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
