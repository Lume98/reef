use reef_native_panel_core::native_panel_core::{
    resolve_native_panel_host_frame, PanelAnimationDescriptor, PanelRect,
};
use reef_ui::native_panel_ui::descriptor::NativePanelPointerRegion;

pub fn resolve_windows_panel_window_frame(
    descriptor: PanelAnimationDescriptor,
    screen_frame: PanelRect,
    compact_width: f64,
    expanded_width: f64,
) -> PanelRect {
    let host_compact_width =
        compact_width.max(reef_native_panel_core::native_panel_core::DEFAULT_PANEL_CANVAS_WIDTH);
    let host_expanded_width =
        expanded_width.max(reef_native_panel_core::native_panel_core::DEFAULT_PANEL_CANVAS_WIDTH);
    let mut frame = resolve_native_panel_host_frame(
        descriptor,
        screen_frame,
        host_compact_width,
        host_expanded_width,
    );
    // Shared native layout uses AppKit's bottom-left coordinate semantics.
    // Windows screen coordinates are top-left, so top alignment is the
    // monitor frame's y origin rather than maxY - height.
    frame.y = screen_frame.y.round();
    frame
}

pub fn windows_client_pointer_regions(
    panel_frame: Option<PanelRect>,
    screen_frame: Option<PanelRect>,
    regions: &[NativePanelPointerRegion],
) -> Vec<NativePanelPointerRegion> {
    let Some(panel_frame) = panel_frame else {
        return regions.to_vec();
    };
    let shared_panel_frame = windows_shared_pointer_region_panel_frame(panel_frame, screen_frame);

    regions
        .iter()
        .map(|region| NativePanelPointerRegion {
            frame: windows_client_pointer_region_frame(shared_panel_frame, region.frame),
            kind: region.kind.clone(),
        })
        .collect()
}

fn windows_shared_pointer_region_panel_frame(
    panel_frame: PanelRect,
    screen_frame: Option<PanelRect>,
) -> PanelRect {
    let Some(screen_frame) = screen_frame else {
        return panel_frame;
    };
    PanelRect {
        y: screen_frame.y + screen_frame.height - panel_frame.height,
        ..panel_frame
    }
}

fn windows_client_pointer_region_frame(panel_frame: PanelRect, frame: PanelRect) -> PanelRect {
    let local_x = frame.x - panel_frame.x;
    let local_bottom_y = frame.y - panel_frame.y;
    PanelRect {
        x: local_x,
        y: panel_frame.height - local_bottom_y - frame.height,
        width: frame.width,
        height: frame.height,
    }
}

#[cfg(test)]
mod tests {
    use super::{resolve_windows_panel_window_frame, windows_client_pointer_regions};
    use reef_native_panel_core::native_panel_core::{
        resolve_panel_layout, PanelAnimationDescriptor, PanelAnimationKind, PanelGeometryMetrics,
        PanelLayoutInput, PanelRect,
    };
    use reef_ui::native_panel_ui::descriptor::{
        NativePanelHostWindowState, NativePanelPointerRegion, NativePanelPointerRegionKind,
        NativePanelTimelineDescriptor,
    };

    #[test]
    fn resolve_windows_panel_window_frame_keeps_top_aligned_screen_origin() {
        let animation = PanelAnimationDescriptor {
            kind: PanelAnimationKind::Open,
            canvas_height: 200.0,
            visible_height: 160.0,
            width_progress: 0.0,
            height_progress: 0.0,
            shoulder_progress: 0.0,
            drop_progress: 0.0,
            cards_progress: 0.0,
        };
        let frame = resolve_windows_panel_window_frame(
            animation,
            PanelRect {
                x: 0.0,
                y: 900.0,
                width: 1440.0,
                height: 900.0,
            },
            253.0,
            323.0,
        );

        assert_eq!(frame.y, 900.0);
    }

    #[test]
    fn resolve_windows_panel_window_frame_uses_shared_host_window_helper() {
        let animation = PanelAnimationDescriptor {
            kind: PanelAnimationKind::Open,
            canvas_height: reef_native_panel_core::native_panel_core::COLLAPSED_PANEL_HEIGHT,
            visible_height: reef_native_panel_core::native_panel_core::COLLAPSED_PANEL_HEIGHT,
            width_progress: 0.0,
            height_progress: 0.0,
            shoulder_progress: 0.0,
            drop_progress: 0.0,
            cards_progress: 0.0,
        };
        let screen_frame = PanelRect {
            x: 0.0,
            y: 0.0,
            width: 1440.0,
            height: 900.0,
        };
        let layout = resolve_panel_layout(PanelLayoutInput {
            screen_frame,
            metrics: PanelGeometryMetrics {
                compact_height:
                    reef_native_panel_core::native_panel_core::DEFAULT_COMPACT_PILL_HEIGHT,
                compact_width:
                    reef_native_panel_core::native_panel_core::DEFAULT_COMPACT_PILL_WIDTH,
                expanded_width:
                    reef_native_panel_core::native_panel_core::DEFAULT_EXPANDED_PILL_WIDTH,
                panel_width: reef_native_panel_core::native_panel_core::DEFAULT_PANEL_CANVAS_WIDTH,
            },
            canvas_height: animation.canvas_height,
            visible_height: animation.visible_height,
            bar_progress: animation.width_progress,
            height_progress: animation.height_progress,
            drop_progress: animation.drop_progress,
            content_visibility: animation.cards_progress,
            collapsed_height: reef_native_panel_core::native_panel_core::COLLAPSED_PANEL_HEIGHT,
            drop_distance: reef_native_panel_core::native_panel_core::PANEL_DROP_DISTANCE,
            content_top_gap: reef_native_panel_core::native_panel_core::EXPANDED_CONTENT_TOP_GAP,
            content_bottom_inset:
                reef_native_panel_core::native_panel_core::EXPANDED_CONTENT_BOTTOM_INSET,
            cards_side_inset: reef_native_panel_core::native_panel_core::EXPANDED_CARDS_SIDE_INSET,
            shoulder_size: reef_native_panel_core::native_panel_core::COMPACT_SHOULDER_SIZE,
            separator_side_inset:
                reef_native_panel_core::native_panel_core::EXPANDED_SEPARATOR_SIDE_INSET,
        });

        let frame = resolve_windows_panel_window_frame(
            animation,
            screen_frame,
            reef_native_panel_core::native_panel_core::DEFAULT_COMPACT_PILL_WIDTH,
            reef_native_panel_core::native_panel_core::DEFAULT_EXPANDED_PILL_WIDTH,
        );

        assert!(frame.width >= layout.content_frame.width);
        assert!(frame.width >= layout.pill_frame.x + layout.pill_frame.width);
    }

    #[test]
    fn windows_client_pointer_regions_projects_from_shared_frame_to_client_space() {
        let window_state = NativePanelHostWindowState {
            frame: Some(PanelRect {
                x: 510.0,
                y: 820.0,
                width: 420.0,
                height: 80.0,
            }),
            visible: true,
            preferred_display_index: 0,
        };
        let regions = vec![NativePanelPointerRegion {
            frame: PanelRect {
                x: 593.5,
                y: 863.0,
                width: 253.0,
                height: 37.0,
            },
            kind: NativePanelPointerRegionKind::CompactBar,
        }];

        let projected = windows_client_pointer_regions(
            window_state.frame,
            Some(PanelRect {
                x: 0.0,
                y: 0.0,
                width: 1440.0,
                height: 900.0,
            }),
            &regions,
        );

        assert_eq!(
            projected,
            vec![NativePanelPointerRegion {
                frame: PanelRect {
                    x: 83.5,
                    y: 0.0,
                    width: 253.0,
                    height: 37.0,
                },
                kind: NativePanelPointerRegionKind::CompactBar,
            }]
        );
    }
}
