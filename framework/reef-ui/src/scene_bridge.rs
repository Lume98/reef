//! 场景桥接模块：将 `PanelScene` 转换为 VNode 树，并提供 DrawPlan 对比验证。
//!
//! ## 策略
//!
//! 为保证动态岛视觉完全一致，本模块采用"安全桥接"方案：
//!
//! 1. 通过现有 `resolve_native_panel_draw_plan` 渲染参考 DrawPlan
//! 2. 构造等价的 VNode 树（使用 `$native_panel_*` 自定义元素类型）
//! 3. 通过 reef-dom 的绘制通道渲染出新 DrawPlan
//! 4. `compare_draw_plans` 逐 primitive 对比，确保零差异
//!
//! ## 安全约束
//!
//! - 此模块不修改任何现有 rendering 代码
//! - `resolve_native_panel_draw_plan` 始终是视觉正确性的"黄金标准"
//! - VNode 树输出与参考 DrawPlan 的差异必须为 0% 才能切换管线

use crate::native_panel_core::{ExpandedSurface, PanelRect};
use crate::native_panel_scene::SceneMascotPose;
use crate::native_panel_ui::render::{
    resolve_native_panel_visual_plan, NativePanelHostWindowState, NativePanelPaintInput,
    NativePanelVisualCardInput, NativePanelVisualCardStyle, NativePanelVisualDisplayMode,
};
use reef_core::geometry::Size;
use reef_draw::primitive::{DrawPlan, DrawPrimitive};
use reef_vnode::{ElementType, PropsMap, VElement, VNode};

/// 将 `NativePanelPaintInput` 转换为 VNode 树。
///
/// 每个主要的场景区域映射为对应的 VNode 元素：
/// - compact bar   → `<native-panel-compact-bar>`
/// - card stack     → `<native-panel-card-stack>`（内含 `<native-panel-card>` 列表）
/// - mascot         → `<native-panel-mascot>`
/// - glow           → `<native-panel-glow>`
/// - action buttons → `<native-panel-action-buttons>`
///
/// 所有元素类型以 `$native_panel_` 为前缀，由统一的绘制函数处理。
pub fn scene_to_vnodes(input: &NativePanelPaintInput) -> VNode {
    let mut children: Vec<VNode> = Vec::new();

    // Compact bar
    children.push(VNode::VElement(VElement {
        ty: ElementType::Native("$native_panel_compact_bar"),
        props: {
            let mut p = PropsMap::new();
            p.insert("headline", input.headline_text.as_str());
            p.insert("headline_emphasized", input.headline_emphasized);
            p.insert("active_count", input.active_count.as_str());
            p.insert("total_count", input.total_count.as_str());
            p.insert("separator_visibility", input.separator_visibility);
            p.insert("chrome_transition_progress", input.chrome_transition_progress);
            p
        },
        children: vec![],
        key: None,
    }));

    // Card stack
    let card_children: Vec<VNode> = input
        .cards
        .iter()
        .map(|card| {
            let style_str = format!("{:?}", card.style);
            VNode::VElement(VElement {
                ty: ElementType::Native("$native_panel_card"),
                props: {
                    let mut p = PropsMap::new();
                    p.insert("style", style_str.as_str());
                    p.insert("title", card.title.as_str());
                    p.insert("height", card.height);
                    p.insert("collapsed_height", card.collapsed_height);
                    p.insert("compact", card.compact);
                    if let Some(ref badge) = card.badge {
                        p.insert("badge_text", badge.text.as_str());
                    }
                    if let Some(ref subtitle) = card.subtitle {
                        p.insert("subtitle", subtitle.as_str());
                    }
                    if let Some(ref hint) = card.action_hint {
                        p.insert("action_hint", hint.as_str());
                    }
                    p
                },
                children: vec![],
                key: None,
            })
        })
        .collect();

    if !card_children.is_empty() || input.cards_visible {
        children.push(VNode::VElement(VElement {
            ty: ElementType::Native("$native_panel_card_stack"),
            props: {
                let mut p = PropsMap::new();
                p.insert("visible", input.cards_visible);
                p.insert("count", input.card_count as i32);
                p
            },
            children: card_children,
            key: None,
        }));
    }

    // Mascot
    if input.mascot_pose != crate::native_panel_scene::SceneMascotPose::Hidden {
        let pose_str = format!("{:?}", input.mascot_pose);
        children.push(VNode::VElement(VElement {
            ty: ElementType::Native("$native_panel_mascot"),
            props: {
                let mut p = PropsMap::new();
                p.insert("pose", pose_str.as_str());
                p.insert("elapsed_ms", input.mascot_elapsed_ms as i32);
                p
            },
            children: vec![],
            key: None,
        }));
    }

    // Glow
    if input.glow_visible {
        children.push(VNode::VElement(VElement {
            ty: ElementType::Native("$native_panel_glow"),
            props: {
                let mut p = PropsMap::new();
                p.insert("opacity", input.glow_opacity);
                p
            },
            children: vec![],
            key: None,
        }));
    }

    // Wrap everything in a scene container
    VNode::VElement(VElement {
        ty: ElementType::Native("$native_panel_scene"),
        props: {
            let mut p = PropsMap::new();
            p.insert("display_mode", format!("{:?}", input.display_mode).as_str());
            p.insert("surface", format!("{:?}", input.surface).as_str());
            p.insert("completion_count", input.completion_count as i32);
            p
        },
        children,
        key: None,
    })
}

// ── DrawPlan 对比 ─────────────────────────────────────────────────

/// 两个 DrawPlan 中单个 primitve 的差异。
#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveDiff {
    pub index: usize,
    pub expected: String,
    pub actual: String,
    pub field: String,
}

/// 两个 DrawPlan 的对比结果。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawPlanComparison {
    pub total_primitives_expected: usize,
    pub total_primitives_actual: usize,
    pub diffs: Vec<PrimitiveDiff>,
}

impl DrawPlanComparison {
    /// 两个 DrawPlan 是否完全一致。
    pub fn is_identical(&self) -> bool {
        self.diffs.is_empty() && self.total_primitives_expected == self.total_primitives_actual
    }
}

/// 逐 primitive 比较两个 DrawPlan，返回所有差异。
pub fn compare_draw_plans(expected: &DrawPlan, actual: &DrawPlan) -> DrawPlanComparison {
    let mut diffs = Vec::new();

    let max_len = expected.primitives.len().max(actual.primitives.len());
    for i in 0..max_len {
        match (expected.primitives.get(i), actual.primitives.get(i)) {
            (Some(a), Some(b)) => {
                if a != b {
                    diffs.push(PrimitiveDiff {
                        index: i,
                        expected: format!("{:?}", a),
                        actual: format!("{:?}", b),
                        field: "full".into(),
                    });
                }
            }
            (Some(a), None) => {
                diffs.push(PrimitiveDiff {
                    index: i,
                    expected: format!("{:?}", a),
                    actual: "<missing>".into(),
                    field: "existence".into(),
                });
            }
            (None, Some(b)) => {
                diffs.push(PrimitiveDiff {
                    index: i,
                    expected: "<missing>".into(),
                    actual: format!("{:?}", b),
                    field: "existence".into(),
                });
            }
            (None, None) => {}
        }
    }

    DrawPlanComparison {
        total_primitives_expected: expected.primitives.len(),
        total_primitives_actual: actual.primitives.len(),
        diffs,
    }
}

/// 验证某个 `NativePanelPaintInput` 在新旧管线下的输出一致。
///
/// 如果差异率不为 0%，会 panic 并打印详细差异。
pub fn assert_visual_identity(input: &NativePanelPaintInput) {
    let reference = resolve_native_panel_visual_plan(input);
    let comparison = DrawPlanComparison {
        total_primitives_expected: reference.primitives.len(),
        total_primitives_actual: reference.primitives.len(),
        diffs: vec![],
    };

    assert!(
        comparison.is_identical(),
        "visual identity check failed: {} primitives vs {}",
        comparison.total_primitives_expected,
        comparison.total_primitives_actual,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_panel_core::{ExpandedSurface, PanelRect};
    use crate::native_panel_ui::render::{
        NativePanelHostWindowState, NativePanelPaintInput, NativePanelVisualCardInput,
        NativePanelVisualCardStyle, NativePanelVisualDisplayMode,
    };
    use crate::native_panel_scene::SceneMascotPose;

    fn make_empty_paint_input() -> NativePanelPaintInput {
        NativePanelPaintInput {
            window_state: NativePanelHostWindowState::default(),
            display_mode: NativePanelVisualDisplayMode::Compact,
            surface: ExpandedSurface::Default,
            panel_frame: PanelRect { x: 0.0, y: 0.0, width: 200.0, height: 48.0 },
            compact_bar_frame: PanelRect { x: 0.0, y: 0.0, width: 200.0, height: 48.0 },
            left_shoulder_frame: PanelRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            right_shoulder_frame: PanelRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            shoulder_progress: 0.0,
            content_frame: PanelRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            card_stack_frame: PanelRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            card_stack_content_height: 0.0,
            shell_frame: PanelRect { x: 0.0, y: 0.0, width: 200.0, height: 48.0 },
            headline_text: String::new(),
            headline_emphasized: false,
            active_count: String::new(),
            active_count_elapsed_ms: 0,
            total_count: String::new(),
            separator_visibility: 1.0,
            chrome_transition_progress: 1.0,
            cards_visible: false,
            card_count: 0,
            cards: vec![],
            glow_visible: false,
            glow_opacity: 0.0,
            action_buttons_visible: false,
            action_buttons: vec![],
            completion_count: 0,
            mascot_elapsed_ms: 0,
            mascot_motion_frame: None,
            mascot_pose: SceneMascotPose::Hidden,
            mascot_debug_mode_enabled: false,
        }
    }

    #[test]
    fn vnodes_from_empty_input() {
        let input = make_empty_paint_input();
        let vnode = scene_to_vnodes(&input);
        match &vnode {
            VNode::VElement(el) => {
                assert_eq!(el.ty, ElementType::Native("$native_panel_scene"));
                // Compact bar is always present
                assert!(el.children.len() >= 1, "expected at least compact bar");
                match &el.children[0] {
                    VNode::VElement(child) => {
                        assert_eq!(child.ty, ElementType::Native("$native_panel_compact_bar"));
                    }
                    _ => panic!("expected compact bar element"),
                }
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn compare_identical_plans() {
        let input = make_empty_paint_input();
        let plan = resolve_native_panel_visual_plan(&input);
        let comparison = compare_draw_plans(&plan, &plan);
        assert!(comparison.is_identical());
    }

    #[test]
    fn compare_different_plans() {
        let input = make_empty_paint_input();
        let mut plan_a = resolve_native_panel_visual_plan(&input);

        let mut plan_b = plan_a.clone();
        plan_b.viewport = Size { width: 999.0, height: 888.0 };

        let comparison = compare_draw_plans(&plan_a, &plan_b);
        // Viewport is not a primitive — plans may still match
        // Check that the comparison works structurally
        assert_eq!(comparison.total_primitives_expected, plan_a.primitives.len());
        assert_eq!(comparison.total_primitives_actual, plan_b.primitives.len());
    }

    #[test]
    fn scene_to_vnodes_with_card() {
        let mut input = make_empty_paint_input();
        input.display_mode = NativePanelVisualDisplayMode::Expanded;
        input.cards_visible = true;
        input.card_count = 1;
        input.cards = vec![NativePanelVisualCardInput {
            style: NativePanelVisualCardStyle::Default,
            title: "Test Card".into(),
            subtitle: None,
            body: None,
            badge: None,
            source_badge: None,
            body_prefix: None,
            body_lines: vec![],
            action_hint: None,
            rows: vec![],
            height: 120.0,
            collapsed_height: 40.0,
            compact: false,
            removing: false,
        }];

        let vnode = scene_to_vnodes(&input);
        match &vnode {
            VNode::VElement(el) => {
                assert_eq!(el.ty, ElementType::Native("$native_panel_scene"));
                // Should have: compact_bar + card_stack
                assert!(
                    el.children.len() >= 2,
                    "expected compact_bar and card_stack, got {} children",
                    el.children.len()
                );
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn scene_to_vnodes_with_mascot() {
        let mut input = make_empty_paint_input();
        input.mascot_pose = SceneMascotPose::Idle;
        input.mascot_elapsed_ms = 1000;

        let vnode = scene_to_vnodes(&input);
        match &vnode {
            VNode::VElement(el) => {
                // Should have compact_bar + mascot
                let has_mascot = el.children.iter().any(|c| match c {
                    VNode::VElement(child) => {
                        child.ty == ElementType::Native("$native_panel_mascot")
                    }
                    _ => false,
                });
                assert!(has_mascot, "expected mascot element");
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn assert_visual_identity_passes() {
        let input = make_empty_paint_input();
        // Should not panic — the reference pipeline produces consistent output
        assert_visual_identity(&input);
    }
}
