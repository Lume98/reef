use echoisland_runtime::RuntimeSnapshot;
use reef_ui::native_panel_ui::{
    descriptor::NativePanelPlatformEvent, render::NativePanelTransitionRequest,
};
use reef_widgets::island_widget::{
    build_cards_from_input, build_compact_bar_from_input, build_mascot_from_input,
    IslandWidgetContentInput,
};
use reef_widgets::{DynamicIsland, DynamicIslandGesture, DynamicIslandTarget, IslandWidget};

use crate::island_widget_bridge::build_island_widget_input;
use crate::native_panel_core::PanelHitTarget;

#[derive(Clone, Debug)]
pub struct DynamicIslandPageState {
    content: IslandWidgetContentInput,
}

impl DynamicIslandPageState {
    pub fn new(content: IslandWidgetContentInput) -> Self {
        Self { content }
    }

    pub fn content(&self) -> &IslandWidgetContentInput {
        &self.content
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicIslandRuntimeAction {
    Dismiss,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicIslandRuntimeEffect {
    PlatformEvent(NativePanelPlatformEvent),
    Transition(NativePanelTransitionRequest),
}

pub fn build_dynamic_island_page_state(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> DynamicIslandPageState {
    DynamicIslandPageState::new(build_island_widget_input(
        snapshot,
        panel_expanded,
        settings_active,
    ))
}

pub fn render_dynamic_island_page(state: &DynamicIslandPageState) -> IslandWidget {
    let content = state.content();
    let mut widget = IslandWidget::new()
        .layout(content.layout)
        .mode(content.mode)
        .compact_bar(build_compact_bar_from_input(content));

    for card in build_cards_from_input(content) {
        widget = widget.card(card);
    }

    if let Some(mascot) = build_mascot_from_input(content) {
        widget = widget.mascot(mascot);
    }

    widget
}

pub fn build_dynamic_island(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> DynamicIsland<DynamicIslandRuntimeAction> {
    let state = build_dynamic_island_page_state(snapshot, panel_expanded, settings_active);
    let content = state.content();
    let mut island = DynamicIsland::new()
        .child(build_compact_bar_from_input(content))
        .on_swipe(DynamicIslandRuntimeAction::Dismiss);

    for card in build_cards_from_input(content) {
        island = island.child(card);
    }

    if let Some(mascot) = build_mascot_from_input(content) {
        island = island.child(mascot);
    }

    island
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

#[cfg(test)]
mod tests {
    use super::*;
    use reef_widgets::island_widget::DisplayMode;

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
    fn page_state_maps_runtime_snapshot_to_content() {
        let state = build_dynamic_island_page_state(&empty_snapshot(), true, true);

        assert_eq!(state.content().mode, DisplayMode::Expanded);
        assert!(state.content().settings_active);
        assert_eq!(state.content().active_session_count, 0);
        assert_eq!(state.content().total_session_count, 0);
    }

    #[test]
    fn page_render_matches_compat_widget_builder() {
        let snapshot = empty_snapshot();
        let state = build_dynamic_island_page_state(&snapshot, false, false);
        let widget = render_dynamic_island_page(&state);
        let compat = crate::island_widget_bridge::build_island_widget(&snapshot, false, false);

        assert_eq!(widget.mode, compat.mode);
        assert_eq!(widget.compact_bar.headline, compat.compact_bar.headline);
        assert_eq!(
            widget.compact_bar.active_count,
            compat.compact_bar.active_count
        );
        assert_eq!(
            widget.compact_bar.total_count,
            compat.compact_bar.total_count
        );
        assert_eq!(
            widget.compact_bar.show_actions,
            compat.compact_bar.show_actions
        );
        assert_eq!(widget.cards.len(), compat.cards.len());
        assert_eq!(widget.cards[0].title, compat.cards[0].title);
        assert_eq!(widget.cards[0].style, compat.cards[0].style);
        assert_eq!(widget.mascot.is_some(), compat.mascot.is_some());
    }

    #[test]
    fn page_builds_declarative_dynamic_island() {
        let island = build_dynamic_island(&empty_snapshot(), true, false);

        assert_eq!(island.bindings().len(), 1);
        assert_eq!(island.action_for_gesture(DynamicIslandGesture::Click), None);
    }

    #[test]
    fn page_resolves_runtime_action_from_swipe() {
        let action = resolve_dynamic_island_action(
            &empty_snapshot(),
            true,
            false,
            DynamicIslandGesture::Swipe,
        );

        assert_eq!(action, Some(DynamicIslandRuntimeAction::Dismiss));
    }

    #[test]
    fn page_resolves_runtime_effect_for_dismiss() {
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
    fn page_does_not_resolve_platform_event_from_click_gesture() {
        let event = resolve_dynamic_island_platform_event(
            &empty_snapshot(),
            false,
            false,
            DynamicIslandGesture::Click,
        );

        assert_eq!(event, None);
    }

    #[test]
    fn page_resolves_transition_request_from_swipe_gesture() {
        let request = resolve_dynamic_island_transition_request(
            &empty_snapshot(),
            true,
            false,
            DynamicIslandGesture::Swipe,
        );

        assert_eq!(request, Some(NativePanelTransitionRequest::Close));
    }

    #[test]
    fn page_keeps_hit_target_mapping_empty() {
        let key = dynamic_island_target_for_hit_target(&PanelHitTarget::focus_session("session-1"));

        assert_eq!(key, None);
    }

    #[test]
    fn page_keeps_target_effect_empty() {
        let effect = resolve_dynamic_island_target_effect(
            &empty_snapshot(),
            true,
            false,
            &DynamicIslandTarget::Session("session-1".to_string()),
            DynamicIslandGesture::Click,
        );

        assert_eq!(effect, None);
    }
}
