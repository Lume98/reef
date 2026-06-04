pub use reef_native_panel_core::dynamic_island_page::{
    DynamicIslandPageModel, DynamicIslandRuntimeAction, DynamicIslandRuntimeEffect,
    DynamicIslandViewState, RuntimeSnapshotDynamicIslandSource,
};

pub fn build_dynamic_island_page_model(
    snapshot: &echoisland_runtime::RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> DynamicIslandPageModel {
    reef_native_panel_core::dynamic_island_page::build_dynamic_island_page_model(
        snapshot,
        DynamicIslandViewState {
            panel_expanded,
            settings_active,
        },
    )
}

pub fn dynamic_island_page(
    model: &DynamicIslandPageModel,
) -> reef_widgets::prelude::DynamicIsland<DynamicIslandRuntimeAction> {
    reef_native_panel_core::dynamic_island_page::dynamic_island_page(model)
}

pub fn resolve_dynamic_island_gesture_effect(
    snapshot: &echoisland_runtime::RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
    gesture: reef_widgets::prelude::DynamicIslandGesture,
) -> Option<DynamicIslandRuntimeEffect> {
    let model = build_dynamic_island_page_model(snapshot, panel_expanded, settings_active);
    let action = dynamic_island_page(&model)
        .action_for_gesture(gesture)
        .cloned()?;
    resolve_dynamic_island_effect(snapshot, action)
}

pub fn resolve_dynamic_island_target_effect(
    snapshot: &echoisland_runtime::RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
    target: &reef_widgets::prelude::DynamicIslandTarget,
    gesture: reef_widgets::prelude::DynamicIslandGesture,
) -> Option<DynamicIslandRuntimeEffect> {
    let _ = (snapshot, panel_expanded, settings_active, target, gesture);
    None
}

pub fn dynamic_island_target_for_hit_target(
    target: &reef_native_panel_core::native_panel_core::PanelHitTarget,
) -> Option<reef_widgets::prelude::DynamicIslandTarget> {
    reef_native_panel_core::dynamic_island_page::dynamic_island_target_for_hit_target(target)
}

pub fn resolve_dynamic_island_effect(
    snapshot: &echoisland_runtime::RuntimeSnapshot,
    action: DynamicIslandRuntimeAction,
) -> Option<DynamicIslandRuntimeEffect> {
    reef_native_panel_core::dynamic_island_page::resolve_dynamic_island_effect(snapshot, action)
}

pub fn resolve_dynamic_island_source_gesture_effect<S>(
    source: &S,
    state: DynamicIslandViewState,
    gesture: reef_widgets::prelude::DynamicIslandGesture,
) -> Option<S::Effect>
where
    S: reef_native_panel_core::DynamicIslandSource,
{
    reef_native_panel_core::dynamic_island_page::resolve_dynamic_island_source_gesture_effect(
        source, state, gesture,
    )
}

pub fn resolve_dynamic_island_source_target_effect<S>(
    source: &S,
    state: DynamicIslandViewState,
    target: &reef_widgets::prelude::DynamicIslandTarget,
    gesture: reef_widgets::prelude::DynamicIslandGesture,
) -> Option<S::Effect>
where
    S: reef_native_panel_core::DynamicIslandSource,
{
    reef_native_panel_core::dynamic_island_page::resolve_dynamic_island_source_target_effect(
        source, state, target, gesture,
    )
}
