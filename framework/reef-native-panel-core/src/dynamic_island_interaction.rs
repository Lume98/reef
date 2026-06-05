use crate::panel::core::PanelPoint;
use crate::panel::ui::descriptor::{native_panel_pointer_state_at_point, NativePanelPointerRegion};

use crate::dynamic_island_page::DynamicIslandGesture;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dynamic_island_page::{DynamicIsland, DynamicIslandSource, DynamicIslandViewState};
    use crate::panel::core::{PanelHitTarget, PanelRect};
    use crate::panel::ui::descriptor::NativePanelPointerRegionKind;

    #[derive(Clone, Copy)]
    struct TestSource;

    impl DynamicIslandSource for TestSource {
        type Action = &'static str;
        type Effect = &'static str;

        fn build(&self, _state: DynamicIslandViewState) -> DynamicIsland<Self::Action> {
            DynamicIsland::new().on_click("focus")
        }

        fn resolve_effect(
            &self,
            action: Self::Action,
            _state: DynamicIslandViewState,
        ) -> Option<Self::Effect> {
            Some(action)
        }
    }

    #[test]
    fn root_gesture_resolves_inside_non_target_region() {
        let regions = vec![NativePanelPointerRegion {
            frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 120.0,
                height: 60.0,
            },
            kind: NativePanelPointerRegionKind::CompactBar,
        }];
        let snapshot = "snapshot";

        let effect = resolve_dynamic_island_root_gesture_at_point(
            &regions,
            PanelPoint { x: 10.0, y: 10.0 },
            DynamicIslandInteractionContext {
                snapshot: &snapshot,
                panel_expanded: false,
                settings_active: false,
            },
            DynamicIslandGesture::Click,
            |_, _| {
                Some(DynamicIslandInteractionEffect::<&str, ()>::PlatformEvent(
                    "focus",
                ))
            },
        );

        assert_eq!(
            effect,
            Some(DynamicIslandInteractionEffect::PlatformEvent("focus"))
        );
    }

    #[test]
    fn root_gesture_skips_hit_target_region() {
        let regions = vec![NativePanelPointerRegion {
            frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 120.0,
                height: 60.0,
            },
            kind: NativePanelPointerRegionKind::HitTarget(PanelHitTarget::focus_session(
                "session-1",
            )),
        }];
        let snapshot = "snapshot";

        let effect = resolve_dynamic_island_root_gesture_at_point(
            &regions,
            PanelPoint { x: 10.0, y: 10.0 },
            DynamicIslandInteractionContext {
                snapshot: &snapshot,
                panel_expanded: true,
                settings_active: false,
            },
            DynamicIslandGesture::Click,
            |_, _| {
                Some(DynamicIslandInteractionEffect::<&str, ()>::PlatformEvent(
                    "focus",
                ))
            },
        );

        assert_eq!(effect, None);
    }

    #[test]
    fn horizontal_swipe_requires_distance_and_directionality() {
        assert!(is_dynamic_island_horizontal_swipe(
            PanelPoint { x: 20.0, y: 20.0 },
            PanelPoint { x: 90.0, y: 24.0 },
            DynamicIslandSwipeSpec::default(),
        ));
        assert!(!is_dynamic_island_horizontal_swipe(
            PanelPoint { x: 20.0, y: 20.0 },
            PanelPoint { x: 50.0, y: 24.0 },
            DynamicIslandSwipeSpec::default(),
        ));
        assert!(!is_dynamic_island_horizontal_swipe(
            PanelPoint { x: 20.0, y: 20.0 },
            PanelPoint { x: 90.0, y: 90.0 },
            DynamicIslandSwipeSpec::default(),
        ));
    }
}
