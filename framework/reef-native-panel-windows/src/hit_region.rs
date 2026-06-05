use reef_ui::panel::core::PanelPoint;
use reef_ui::panel::ui::descriptor::{NativePanelInteractionPlan, NativePanelPointerRegion};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowsNativePanelHitTest {
    Client,
    Transparent,
}

pub fn resolve_windows_native_panel_hit_test(
    regions: &[NativePanelPointerRegion],
    point: PanelPoint,
) -> WindowsNativePanelHitTest {
    if NativePanelInteractionPlan::from_pointer_regions(regions).inside_regions(point) {
        WindowsNativePanelHitTest::Client
    } else {
        WindowsNativePanelHitTest::Transparent
    }
}

#[cfg(test)]
mod tests {
    use reef_ui::panel::core::PanelRect;
    use reef_ui::panel::ui::descriptor::{NativePanelPointerRegion, NativePanelPointerRegionKind};

    use super::{resolve_windows_native_panel_hit_test, WindowsNativePanelHitTest};

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
            resolve_windows_native_panel_hit_test(
                &regions,
                reef_ui::panel::core::PanelPoint { x: 40.0, y: 20.0 }
            ),
            WindowsNativePanelHitTest::Client
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
            resolve_windows_native_panel_hit_test(
                &regions,
                reef_ui::panel::core::PanelPoint { x: 4.0, y: 4.0 }
            ),
            WindowsNativePanelHitTest::Transparent
        );
    }
}
