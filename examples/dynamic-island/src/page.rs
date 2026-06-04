use echoisland_runtime::RuntimeSnapshot;
use reef_ui::native_panel_ui::{
    descriptor::NativePanelPlatformEvent, render::NativePanelTransitionRequest,
};
use reef_widgets::island_widget::{
    build_cards_from_input, build_compact_bar_from_input, build_mascot_from_input, DisplayMode,
    IslandPendingApprovalInput, IslandPendingQuestionInput, IslandSessionInput,
    IslandWidgetContentInput,
};
use reef_widgets::{DynamicIsland, DynamicIslandGesture, DynamicIslandTarget};

use crate::native_panel_core::PanelHitTarget;

#[derive(Clone, Debug)]
pub struct DynamicIslandPageModel {
    content: IslandWidgetContentInput,
}

impl DynamicIslandPageModel {
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

pub fn build_dynamic_island_page_model(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> DynamicIslandPageModel {
    DynamicIslandPageModel {
        content: IslandWidgetContentInput {
            mode: if panel_expanded {
                DisplayMode::Expanded
            } else {
                DisplayMode::Compact
            },
            layout: Default::default(),
            settings_active,
            active_session_count: snapshot.active_session_count,
            total_session_count: snapshot.total_session_count,
            pending_permissions: snapshot
                .pending_permissions
                .iter()
                .map(|pending| IslandPendingApprovalInput {
                    session_id: pending.session_id.clone(),
                    source: pending.source.clone(),
                    tool_description: pending.tool_description.clone(),
                })
                .collect(),
            pending_questions: snapshot
                .pending_questions
                .iter()
                .map(|pending| IslandPendingQuestionInput {
                    session_id: pending.session_id.clone(),
                    source: pending.source.clone(),
                    header: pending.header.clone(),
                    text: pending.text.clone(),
                })
                .collect(),
            sessions: snapshot
                .sessions
                .iter()
                .map(|session| IslandSessionInput {
                    status: session.status.clone(),
                    source: session.source.clone(),
                    model: session.model.clone(),
                    last_user_prompt: session.last_user_prompt.clone(),
                    last_assistant_message: session.last_assistant_message.clone(),
                    current_tool: session.current_tool.clone(),
                    tool_description: session.tool_description.clone(),
                })
                .collect(),
        },
    }
}

pub fn dynamic_island_page(
    model: &DynamicIslandPageModel,
) -> DynamicIsland<DynamicIslandRuntimeAction> {
    let content = model.content();
    let mut island = DynamicIsland::new()
        .mode(content.mode)
        .layout(content.layout)
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
    dynamic_island_page(&build_dynamic_island_page_model(
        snapshot,
        panel_expanded,
        settings_active,
    ))
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
    dynamic_island_page(&build_dynamic_island_page_model(
        snapshot,
        panel_expanded,
        settings_active,
    ))
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
    fn page_model_maps_runtime_snapshot_to_content() {
        let model = build_dynamic_island_page_model(&empty_snapshot(), true, true);

        assert_eq!(model.content().mode, DisplayMode::Expanded);
        assert!(model.content().settings_active);
        assert_eq!(model.content().active_session_count, 0);
        assert_eq!(model.content().total_session_count, 0);
    }

    #[test]
    fn page_render_matches_current_visual_behavior() {
        let snapshot = empty_snapshot();
        let widget = dynamic_island_page(&build_dynamic_island_page_model(&snapshot, false, false))
            .to_widget();

        assert_eq!(widget.mode, DisplayMode::Compact);
        assert_eq!(widget.compact_bar.headline, "Reef");
        assert_eq!(widget.compact_bar.active_count, "0");
        assert_eq!(widget.compact_bar.total_count, "0");
        assert!(!widget.compact_bar.show_actions);
        assert_eq!(widget.cards.len(), 1);
        assert_eq!(widget.cards[0].title, "No active sessions");
        assert!(widget.mascot.is_none());
    }

    #[test]
    fn page_interaction_is_driven_by_same_tree() {
        let island = dynamic_island_page(&build_dynamic_island_page_model(
            &empty_snapshot(),
            true,
            false,
        ));

        assert_eq!(island.bindings().len(), 1);
        assert_eq!(
            island.action_for_gesture(DynamicIslandGesture::Swipe),
            Some(&DynamicIslandRuntimeAction::Dismiss)
        );
    }

    #[test]
    fn dismiss_maps_to_close_transition() {
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
    fn target_mapping_remains_empty() {
        let key = dynamic_island_target_for_hit_target(&PanelHitTarget::focus_session("session-1"));

        assert_eq!(key, None);
    }
}
