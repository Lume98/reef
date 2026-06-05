use echoisland_runtime::RuntimeSnapshot;
use reef_ui::panel::{
    core::{PanelHitTarget, PanelPoint},
    ui::{
        descriptor::{
            native_panel_pointer_state_at_point, NativePanelPlatformEvent, NativePanelPointerRegion,
        },
        render::NativePanelTransitionRequest,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicIslandGesture {
    Click,
    Swipe,
    Hover,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DynamicIslandTarget {
    Session(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicIslandRuntimeEffect {
    PlatformEvent(NativePanelPlatformEvent),
    Transition(NativePanelTransitionRequest),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DynamicIslandInteractionContext<'a, Snapshot> {
    pub snapshot: &'a Snapshot,
    pub panel_expanded: bool,
    pub settings_active: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicIslandInteractionEffect<Event, Transition> {
    PlatformEvent(Event),
    Transition(Transition),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DynamicIslandSwipeSpec {
    pub min_horizontal_distance: f64,
    pub axis_bias: f64,
}

impl Default for DynamicIslandSwipeSpec {
    fn default() -> Self {
        Self {
            min_horizontal_distance: 48.0,
            axis_bias: 1.5,
        }
    }
}

pub fn resolve_dynamic_island_gesture_effect(
    _snapshot: &RuntimeSnapshot,
    _panel_expanded: bool,
    _settings_active: bool,
    gesture: DynamicIslandGesture,
) -> Option<DynamicIslandRuntimeEffect> {
    match gesture {
        DynamicIslandGesture::Swipe => Some(DynamicIslandRuntimeEffect::Transition(
            NativePanelTransitionRequest::Close,
        )),
        DynamicIslandGesture::Click | DynamicIslandGesture::Hover => None,
    }
}

pub fn resolve_dynamic_island_target_effect(
    _snapshot: &RuntimeSnapshot,
    _panel_expanded: bool,
    _settings_active: bool,
    _target: &DynamicIslandTarget,
    _gesture: DynamicIslandGesture,
) -> Option<DynamicIslandRuntimeEffect> {
    None
}

pub fn dynamic_island_target_for_hit_target(
    _target: &PanelHitTarget,
) -> Option<DynamicIslandTarget> {
    None
}

pub fn resolve_dynamic_island_root_gesture_at_point<Snapshot, Event, Transition>(
    pointer_regions: &[NativePanelPointerRegion],
    point: PanelPoint,
    context: DynamicIslandInteractionContext<'_, Snapshot>,
    gesture: DynamicIslandGesture,
    resolve: impl FnOnce(
        DynamicIslandInteractionContext<'_, Snapshot>,
        DynamicIslandGesture,
    ) -> Option<DynamicIslandInteractionEffect<Event, Transition>>,
) -> Option<DynamicIslandInteractionEffect<Event, Transition>> {
    let pointer_state = native_panel_pointer_state_at_point(pointer_regions, point);
    if !pointer_state.inside
        || pointer_state.platform_event.is_some()
        || pointer_state.hit_target.is_some()
    {
        return None;
    }

    resolve(context, gesture)
}

pub fn resolve_dynamic_island_gesture<Snapshot, Event, Transition>(
    context: DynamicIslandInteractionContext<'_, Snapshot>,
    gesture: DynamicIslandGesture,
    resolve: impl FnOnce(
        DynamicIslandInteractionContext<'_, Snapshot>,
        DynamicIslandGesture,
    ) -> Option<DynamicIslandInteractionEffect<Event, Transition>>,
) -> Option<DynamicIslandInteractionEffect<Event, Transition>> {
    resolve(context, gesture)
}

pub fn is_dynamic_island_horizontal_swipe(
    start: PanelPoint,
    end: PanelPoint,
    spec: DynamicIslandSwipeSpec,
) -> bool {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    dx.abs() >= spec.min_horizontal_distance && dx.abs() >= dy.abs() * spec.axis_bias
}
