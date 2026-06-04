// Phase 3: 快照测试示例
//
// 演示如何为关键渲染场景创建快照测试

#[cfg(test)]
mod snapshot_tests {
    use crate::native_panel_core::{PanelPoint, PanelRect};
    use crate::native_panel_renderer::snapshot_testing::{RenderSnapshot, SnapshotMetadata};
    use crate::native_panel_renderer::visual_primitives::{
        NativePanelVisualColor, NativePanelVisualPlan, NativePanelVisualPrimitive,
        NativePanelVisualTextAlignment, NativePanelVisualTextRole, NativePanelVisualTextWeight,
    };
    use insta::assert_yaml_snapshot;

    // ============ 基础场景测试 ============

    /// 测试场景 1: Compact 模式初始状态
    #[test]
    fn test_compact_mode_initial() {
        let plan = create_compact_plan();

        let snapshot = RenderSnapshot::from_visual_plan(
            "compact_initial",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Compact".to_string(),
                animation_progress: 0.0,
            },
        );

        assert_yaml_snapshot!("compact_initial", snapshot);
    }

    /// 测试场景 2: Expanded 模式完整状态
    #[test]
    fn test_expanded_mode_full() {
        let plan = create_expanded_plan();

        let snapshot = RenderSnapshot::from_visual_plan(
            "expanded_full",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Expanded".to_string(),
                animation_progress: 1.0,
            },
        );

        assert_yaml_snapshot!("expanded_full", snapshot);
    }

    // ============ 动画关键帧测试 ============

    #[test]
    fn test_expanding_10() {
        let plan = create_expanding_plan(0.1);
        let snapshot = RenderSnapshot::from_visual_plan(
            "expanding_10",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Expanding".to_string(),
                animation_progress: 0.1,
            },
        );
        assert_yaml_snapshot!("expanding_10", snapshot);
    }

    #[test]
    fn test_expanding_25() {
        let plan = create_expanding_plan(0.25);
        let snapshot = RenderSnapshot::from_visual_plan(
            "expanding_25",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Expanding".to_string(),
                animation_progress: 0.25,
            },
        );
        assert_yaml_snapshot!("expanding_25", snapshot);
    }

    #[test]
    fn test_expanding_50() {
        let plan = create_expanding_plan(0.5);
        let snapshot = RenderSnapshot::from_visual_plan(
            "expanding_50",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Expanding".to_string(),
                animation_progress: 0.5,
            },
        );
        assert_yaml_snapshot!("expanding_50", snapshot);
    }

    #[test]
    fn test_expanding_75() {
        let plan = create_expanding_plan(0.75);
        let snapshot = RenderSnapshot::from_visual_plan(
            "expanding_75",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Expanding".to_string(),
                animation_progress: 0.75,
            },
        );
        assert_yaml_snapshot!("expanding_75", snapshot);
    }

    #[test]
    fn test_expanding_90() {
        let plan = create_expanding_plan(0.9);
        let snapshot = RenderSnapshot::from_visual_plan(
            "expanding_90",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Expanding".to_string(),
                animation_progress: 0.9,
            },
        );
        assert_yaml_snapshot!("expanding_90", snapshot);
    }

    // ============ 卡片状态测试 ============

    #[test]
    fn test_card_single() {
        let plan = create_card_plan(1);
        let snapshot = RenderSnapshot::from_visual_plan(
            "card_single",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Expanded".to_string(),
                animation_progress: 1.0,
            },
        );
        assert_yaml_snapshot!("card_single", snapshot);
    }

    #[test]
    fn test_card_stacked_2() {
        let plan = create_card_plan(2);
        let snapshot = RenderSnapshot::from_visual_plan(
            "card_stacked_2",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Expanded".to_string(),
                animation_progress: 1.0,
            },
        );
        assert_yaml_snapshot!("card_stacked_2", snapshot);
    }

    #[test]
    fn test_card_stacked_3() {
        let plan = create_card_plan(3);
        let snapshot = RenderSnapshot::from_visual_plan(
            "card_stacked_3",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Expanded".to_string(),
                animation_progress: 1.0,
            },
        );
        assert_yaml_snapshot!("card_stacked_3", snapshot);
    }

    // ============ 特殊状态测试 ============

    #[test]
    fn test_empty_state() {
        let plan = NativePanelVisualPlan {
            hidden: false,
            primitives: vec![
                NativePanelVisualPrimitive::RoundRect {
                    frame: PanelRect {
                        x: 0.0,
                        y: 0.0,
                        width: 400.0,
                        height: 200.0,
                    },
                    radius: 16.0,
                    color: NativePanelVisualColor::rgb(12, 12, 15),
                },
                NativePanelVisualPrimitive::Text {
                    role: NativePanelVisualTextRole::CardTitle,
                    origin: PanelPoint { x: 200.0, y: 90.0 },
                    max_width: 300.0,
                    text: "No active sessions".to_string(),
                    color: NativePanelVisualColor::rgb(156, 166, 184),
                    size: 14,
                    weight: NativePanelVisualTextWeight::Normal,
                    alignment: NativePanelVisualTextAlignment::Center,
                    alpha: 0.7,
                },
            ],
        };

        let snapshot = RenderSnapshot::from_visual_plan(
            "empty_state",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Expanded".to_string(),
                animation_progress: 1.0,
            },
        );
        assert_yaml_snapshot!("empty_state", snapshot);
    }

    // ============ 测试数据生成器 ============

    fn create_compact_plan() -> NativePanelVisualPlan {
        NativePanelVisualPlan {
            hidden: false,
            primitives: vec![
                // 背景胶囊
                NativePanelVisualPrimitive::RoundRect {
                    frame: PanelRect {
                        x: 0.0,
                        y: 0.0,
                        width: 208.0,
                        height: 44.0,
                    },
                    radius: 22.0,
                    color: NativePanelVisualColor::rgb(18, 18, 22),
                },
                // 标题文本
                NativePanelVisualPrimitive::Text {
                    role: NativePanelVisualTextRole::CompactHeadline,
                    origin: PanelPoint { x: 52.0, y: 15.0 },
                    max_width: 156.0,
                    text: "AI Gateway".to_string(),
                    color: NativePanelVisualColor::rgb(255, 255, 255),
                    size: 13,
                    weight: NativePanelVisualTextWeight::Semibold,
                    alignment: NativePanelVisualTextAlignment::Center,
                    alpha: 1.0,
                },
                // 活跃计数
                NativePanelVisualPrimitive::Text {
                    role: NativePanelVisualTextRole::CompactActiveCount,
                    origin: PanelPoint { x: 168.0, y: 14.0 },
                    max_width: 24.0,
                    text: "3".to_string(),
                    color: NativePanelVisualColor::rgb(102, 222, 145),
                    size: 15,
                    weight: NativePanelVisualTextWeight::Semibold,
                    alignment: NativePanelVisualTextAlignment::Right,
                    alpha: 1.0,
                },
            ],
        }
    }

    fn create_expanded_plan() -> NativePanelVisualPlan {
        NativePanelVisualPlan {
            hidden: false,
            primitives: vec![
                // Expanded 背景
                NativePanelVisualPrimitive::RoundRect {
                    frame: PanelRect {
                        x: 0.0,
                        y: 0.0,
                        width: 400.0,
                        height: 300.0,
                    },
                    radius: 16.0,
                    color: NativePanelVisualColor::rgb(12, 12, 15),
                },
                // 分隔线
                NativePanelVisualPrimitive::Rect {
                    frame: PanelRect {
                        x: 20.0,
                        y: 52.0,
                        width: 360.0,
                        height: 1.0,
                    },
                    color: NativePanelVisualColor::rgb(62, 62, 70),
                },
                // 卡片
                NativePanelVisualPrimitive::RoundRect {
                    frame: PanelRect {
                        x: 20.0,
                        y: 70.0,
                        width: 360.0,
                        height: 100.0,
                    },
                    radius: 8.0,
                    color: NativePanelVisualColor::rgb(24, 24, 27),
                },
            ],
        }
    }

    fn create_expanding_plan(progress: f64) -> NativePanelVisualPlan {
        let width = 208.0 + (400.0 - 208.0) * progress;
        let height = 44.0 + (300.0 - 44.0) * progress;

        NativePanelVisualPlan {
            hidden: false,
            primitives: vec![NativePanelVisualPrimitive::RoundRect {
                frame: PanelRect {
                    x: 0.0,
                    y: 0.0,
                    width,
                    height,
                },
                radius: 22.0 - (22.0 - 16.0) * progress,
                color: NativePanelVisualColor::rgb(18, 18, 22),
            }],
        }
    }

    fn create_card_plan(card_count: usize) -> NativePanelVisualPlan {
        let mut primitives = vec![
            // Expanded 背景
            NativePanelVisualPrimitive::RoundRect {
                frame: PanelRect {
                    x: 0.0,
                    y: 0.0,
                    width: 400.0,
                    height: 300.0,
                },
                radius: 16.0,
                color: NativePanelVisualColor::rgb(12, 12, 15),
            },
        ];

        // 添加多个卡片
        for i in 0..card_count {
            let y_offset = 70.0 + (i as f64 * 80.0);
            primitives.push(NativePanelVisualPrimitive::RoundRect {
                frame: PanelRect {
                    x: 20.0,
                    y: y_offset,
                    width: 360.0,
                    height: 70.0,
                },
                radius: 8.0,
                color: NativePanelVisualColor::rgb(24, 24, 27),
            });

            primitives.push(NativePanelVisualPrimitive::Text {
                role: NativePanelVisualTextRole::CardTitle,
                origin: PanelPoint {
                    x: 30.0,
                    y: y_offset + 15.0,
                },
                max_width: 340.0,
                text: format!("Session {}", i + 1),
                color: NativePanelVisualColor::rgb(255, 255, 255),
                size: 13,
                weight: NativePanelVisualTextWeight::Semibold,
                alignment: NativePanelVisualTextAlignment::Left,
                alpha: 1.0,
            });
        }

        NativePanelVisualPlan {
            hidden: false,
            primitives,
        }
    }
}
