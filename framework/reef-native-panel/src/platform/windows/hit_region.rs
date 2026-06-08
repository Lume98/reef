use crate::presentation::descriptor::{NativePanelInteractionPlan, NativePanelPointerRegion};
use crate::state::PanelPoint;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowsPanelHitTest {
    Client,
    Transparent,
}

pub fn resolve_windows_panel_hit_test(
    regions: &[NativePanelPointerRegion],
    point: PanelPoint,
) -> WindowsPanelHitTest {
    if NativePanelInteractionPlan::from_pointer_regions(regions).inside_regions(point) {
        WindowsPanelHitTest::Client
    } else {
        WindowsPanelHitTest::Transparent
    }
}

#[cfg(test)]
mod tests {
    use crate::presentation::descriptor::{NativePanelPointerRegion, NativePanelPointerRegionKind};
    use crate::state::PanelRect;

    use super::{resolve_windows_panel_hit_test, WindowsPanelHitTest};

    #[test]
    fn hit_test_accepts_points_inside_shared_pointer_regions() {
        let regions = vec![NativePanelPointerRegion {
            frame: PanelRect {
                x: 20.0,
                y: 10.0,
                width: 120.0,
                height: 36.0,
            },
            kind: NativePanelPointerRegionKind::CompactBar,
        }];

        assert_eq!(
            resolve_windows_panel_hit_test(&regions, crate::state::PanelPoint { x: 40.0, y: 20.0 }),
            WindowsPanelHitTest::Client
        );
    }

    #[test]
    fn hit_test_passes_through_transparent_margins() {
        let regions = vec![NativePanelPointerRegion {
            frame: PanelRect {
                x: 20.0,
                y: 10.0,
                width: 120.0,
                height: 36.0,
            },
            kind: NativePanelPointerRegionKind::CompactBar,
        }];

        assert_eq!(
            resolve_windows_panel_hit_test(&regions, crate::state::PanelPoint { x: 4.0, y: 4.0 }),
            WindowsPanelHitTest::Transparent
        );
    }
}
