use echoisland_runtime::RuntimeSnapshot;
use reef_widgets::island_widget::{
    build_cards_from_input, build_compact_bar_from_input, build_mascot_from_input,
    IslandPendingApprovalInput, IslandPendingQuestionInput, IslandSessionInput,
    IslandWidgetContentInput,
};
use reef_widgets::prelude::DisplayMode;
pub use reef_widgets::prelude::{DynamicIsland, DynamicIslandGesture, DynamicIslandTarget};

use crate::native_panel_core::PanelHitTarget;
use crate::panel::ui::{
    descriptor::NativePanelPlatformEvent, render::NativePanelTransitionRequest,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DynamicIslandViewState {
    pub panel_expanded: bool,
    pub settings_active: bool,
}

pub trait DynamicIslandSource {
    type Action: Clone;
    type Effect;

    fn build(&self, state: DynamicIslandViewState) -> DynamicIsland<Self::Action>;

    fn resolve_effect(
        &self,
        action: Self::Action,
        state: DynamicIslandViewState,
    ) -> Option<Self::Effect>;
}

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

#[derive(Clone, Copy, Debug)]
pub struct RuntimeSnapshotDynamicIslandSource<'a> {
    snapshot: &'a RuntimeSnapshot,
}

impl<'a> RuntimeSnapshotDynamicIslandSource<'a> {
    pub fn new(snapshot: &'a RuntimeSnapshot) -> Self {
        Self { snapshot }
    }

    pub fn snapshot(&self) -> &'a RuntimeSnapshot {
        self.snapshot
    }
}

impl DynamicIslandSource for RuntimeSnapshotDynamicIslandSource<'_> {
    type Action = DynamicIslandRuntimeAction;
    type Effect = DynamicIslandRuntimeEffect;

    fn build(&self, state: DynamicIslandViewState) -> DynamicIsland<Self::Action> {
        dynamic_island_page(&build_dynamic_island_page_model(self.snapshot, state))
    }

    fn resolve_effect(
        &self,
        action: Self::Action,
        _state: DynamicIslandViewState,
    ) -> Option<Self::Effect> {
        resolve_dynamic_island_effect(self.snapshot, action)
    }
}

pub fn build_dynamic_island_page_model(
    snapshot: &RuntimeSnapshot,
    state: DynamicIslandViewState,
) -> DynamicIslandPageModel {
    DynamicIslandPageModel {
        content: IslandWidgetContentInput {
            mode: if state.panel_expanded {
                DisplayMode::Expanded
            } else {
                DisplayMode::Compact
            },
            layout: Default::default(),
            settings_active: state.settings_active,
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

pub fn resolve_dynamic_island_source_gesture_effect<S>(
    source: &S,
    state: DynamicIslandViewState,
    gesture: DynamicIslandGesture,
) -> Option<S::Effect>
where
    S: DynamicIslandSource,
{
    let action = source.build(state).action_for_gesture(gesture).cloned()?;
    source.resolve_effect(action, state)
}

pub fn resolve_dynamic_island_source_target_effect<S>(
    source: &S,
    state: DynamicIslandViewState,
    target: &DynamicIslandTarget,
    gesture: DynamicIslandGesture,
) -> Option<S::Effect>
where
    S: DynamicIslandSource,
{
    let action = source
        .build(state)
        .action_for_target_gesture(target, gesture)
        .cloned()?;
    source.resolve_effect(action, state)
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

pub fn dynamic_island_target_for_hit_target(
    _target: &PanelHitTarget,
) -> Option<DynamicIslandTarget> {
    None
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
        let model = build_dynamic_island_page_model(
            &empty_snapshot(),
            DynamicIslandViewState {
                panel_expanded: true,
                settings_active: true,
            },
        );

        assert_eq!(model.content().mode, DisplayMode::Expanded);
        assert!(model.content().settings_active);
        assert_eq!(model.content().active_session_count, 0);
        assert_eq!(model.content().total_session_count, 0);
    }

    #[test]
    fn page_render_matches_current_visual_behavior() {
        let snapshot = empty_snapshot();
        let widget = dynamic_island_page(&build_dynamic_island_page_model(
            &snapshot,
            DynamicIslandViewState::default(),
        ))
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
            DynamicIslandViewState {
                panel_expanded: true,
                settings_active: false,
            },
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

    #[test]
    fn snapshot_source_contract_stays_stable() {
        let snapshot = empty_snapshot();
        let source = RuntimeSnapshotDynamicIslandSource::new(&snapshot);
        let state = DynamicIslandViewState {
            panel_expanded: false,
            settings_active: false,
        };

        let island = source.build(state);
        let effect = resolve_dynamic_island_source_gesture_effect(
            &source,
            state,
            DynamicIslandGesture::Swipe,
        );

        assert_eq!(
            island.action_for_gesture(DynamicIslandGesture::Swipe),
            Some(&DynamicIslandRuntimeAction::Dismiss)
        );
        assert_eq!(
            effect,
            Some(DynamicIslandRuntimeEffect::Transition(
                NativePanelTransitionRequest::Close
            ))
        );
    }
}
