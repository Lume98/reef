#[cfg(test)]
mod snapshot_tests {
    use crate::runtime::snapshot_testing::{RenderSnapshot, SnapshotMetadata};
    use reef::core::{
        color::Color,
        geometry::{Rect, Size},
    };
    use reef::draw::primitive::{DrawPlan, DrawPrimitive, TextAlignment, TextWeight};

    #[test]
    fn snapshot_serializes_draw_plan_text_frame() {
        let plan = DrawPlan {
            hidden: false,
            viewport: Size {
                width: 120.0,
                height: 44.0,
            },
            primitives: vec![DrawPrimitive::Text {
                frame: Rect {
                    x: 10.0,
                    y: 12.0,
                    width: 80.0,
                    height: 24.0,
                },
                text: "Reef UI".to_string(),
                color: Color::WHITE,
                size: 13,
                weight: TextWeight::Semibold,
                alignment: TextAlignment::Center,
                alpha: 1.0,
            }],
        };

        let snapshot = RenderSnapshot::from_visual_plan(
            "compact",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-06-05".to_string(),
                display_mode: "Compact".to_string(),
                animation_progress: 0.0,
            },
        );

        assert_eq!(snapshot.primitives.len(), 1);
        assert_eq!(snapshot.primitives[0].primitive_type, "Text");
        assert!(snapshot.primitives[0].data.get("role").is_none());
    }
}
