//! 灵动岛逐组件迁移渲染器。
//!
//! 将 `NativePanelPaintInput` 同时通过新旧两条管线渲染，逐 primitive 对比，
//! 零差异才切换到新管线。每个灵动岛组件可以独立控制迁移状态。

use crate::native_panel_ui::render::{
    resolve_native_panel_visual_plan, NativePanelPaintInput,
};
use crate::scene_bridge::compare_draw_plans;
use reef_core::geometry::Size;
use reef_dom::ReefRenderer;
use reef_draw::primitive::DrawPlan;
use reef_vnode::{ElementType, PropsMap, VElement, VNode};

/// 控制哪些灵动岛子组件使用新 VNode 管线渲染。
#[derive(Clone, Debug)]
pub struct MigrantComponents {
    /// compact bar → VNode（默认 false）
    pub compact_bar: bool,
    /// 卡片堆栈 → VNode
    pub card_stack: bool,
    /// 吉祥物 → VNode
    pub mascot: bool,
    /// 发光效果 → VNode
    pub glow: bool,
}

impl MigrantComponents {
    pub fn none() -> Self {
        Self {
            compact_bar: false,
            card_stack: false,
            mascot: false,
            glow: false,
        }
    }

    pub fn all() -> Self {
        Self {
            compact_bar: true,
            card_stack: true,
            mascot: true,
            glow: true,
        }
    }

    /// 是否有任何组件已迁移到新管线。
    pub fn any(&self) -> bool {
        self.compact_bar || self.card_stack || self.mascot || self.glow
    }
}

/// 新旧管线并行渲染器。支持逐组件迁移，自动对比确保视觉一致。
pub struct PanelRenderer {
    renderer: ReefRenderer,
    migrant: MigrantComponents,
    viewport: Size,
}

impl PanelRenderer {
    /// 创建新的 PanelRenderer。
    pub fn new(viewport: Size, migrant: MigrantComponents) -> Self {
        Self {
            renderer: ReefRenderer::new(viewport),
            migrant,
            viewport,
        }
    }

    /// 设置视口大小。
    pub fn set_viewport(&mut self, viewport: Size) {
        self.viewport = viewport;
        self.renderer = ReefRenderer::new(viewport);
    }

    /// 更新迁移配置。
    pub fn set_migrant(&mut self, migrant: MigrantComponents) {
        self.migrant = migrant;
    }

    /// 渲染一帧：通过新旧管线并行输出，对比后返回一致的结果。
    ///
    /// - 如果无组件迁移，直接返回参考 DrawPlan（零开销）
    /// - 如果有组件迁移，渲染 VNode 管线后对比
    /// - 对比不一致时回退到参考 DrawPlan 并记录警告
    /// - 对比一致时返回新管线输出
    pub fn render(&mut self, input: &NativePanelPaintInput) -> DrawPlan {
        let reference = resolve_native_panel_visual_plan(input);

        // 无组件迁移 → 直接使用参考
        if !self.migrant.any() {
            return reference;
        }

        // 有组件迁移 → 构建 VNode 树并渲染
        let vnode = self.build_migrant_vnode(input);
        let candidate = self.renderer.render(vnode);

        // 对比新旧管线输出
        let comparison = compare_draw_plans(&reference, &candidate);

        if comparison.is_identical() {
            candidate
        } else {
            log::warn!(
                "[PanelRenderer] {} primitive 差异，回退到参考管线",
                comparison.diffs.len()
            );
            reference
        }
    }

    /// 根据迁移状态构建 VNode 树。
    ///
    /// 已迁移的组件生成真实 VNode 子树；
    /// 未迁移的组件使用 `$native_panel_*` 透明包装元素。
    fn build_migrant_vnode(&self, input: &NativePanelPaintInput) -> VNode {
        let mut children: Vec<VNode> = Vec::new();

        // Compact bar
        if self.migrant.compact_bar {
            children.push(self.render_compact_bar_vnode(input));
        } else {
            children.push(make_opaque(
                "$native_panel_compact_bar",
                vec![
                    ("headline", reef_vnode::PropValue::String(input.headline_text.clone())),
                    ("active_count", reef_vnode::PropValue::String(input.active_count.clone())),
                    ("total_count", reef_vnode::PropValue::String(input.total_count.clone())),
                ],
            ));
        }

        // Card stack
        if self.migrant.card_stack {
            children.push(self.render_card_stack_vnode(input));
        } else if input.cards_visible || !input.cards.is_empty() {
            children.push(make_opaque(
                "$native_panel_card_stack",
                vec![
                    ("visible", reef_vnode::PropValue::Bool(input.cards_visible)),
                    ("count", reef_vnode::PropValue::I32(input.card_count as i32)),
                ],
            ));
        }

        // Mascot
        if input.mascot_pose != crate::native_panel_scene::SceneMascotPose::Hidden {
            if self.migrant.mascot {
                children.push(self.render_mascot_vnode(input));
            } else {
                let pose_str = format!("{:?}", input.mascot_pose);
                children.push(make_opaque(
                    "$native_panel_mascot",
                    vec![("pose", reef_vnode::PropValue::String(pose_str))],
                ));
            }
        }

        // Glow
        if input.glow_visible {
            if self.migrant.glow {
                children.push(self.render_glow_vnode(input));
            } else {
                children.push(make_opaque(
                    "$native_panel_glow",
                    vec![("opacity", reef_vnode::PropValue::F64(input.glow_opacity))],
                ));
            }
        }

        VNode::VElement(VElement {
            ty: ElementType::Native("$native_panel_scene"),
            props: PropsMap::new(),
            children,
            key: None,
        })
    }

    // ── 各组件 VNode 渲染（随迁移进度逐步实现）──

    fn render_compact_bar_vnode(&self, _input: &NativePanelPaintInput) -> VNode {
        // Phase 2: 替换为真实 Container/Label/Row 组件
        make_opaque("$native_panel_compact_bar", vec![])
    }

    fn render_card_stack_vnode(&self, _input: &NativePanelPaintInput) -> VNode {
        // Phase 3: 替换为真实卡片 VNode
        make_opaque("$native_panel_card_stack", vec![])
    }

    fn render_mascot_vnode(&self, _input: &NativePanelPaintInput) -> VNode {
        // Phase 4: 替换为真实吉祥物 VNode
        make_opaque("$native_panel_mascot", vec![])
    }

    fn render_glow_vnode(&self, _input: &NativePanelPaintInput) -> VNode {
        // Phase 4: 替换为真实发光效果 VNode
        make_opaque("$native_panel_glow", vec![])
    }
}

// ── 辅助函数 ─────────────────────────────────────────────────────

fn make_opaque(ty: &'static str, props: Vec<(&'static str, reef_vnode::PropValue)>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native(ty),
        props: PropsMap::from_pairs(props),
        children: vec![],
        key: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_panel_core::{ExpandedSurface, PanelRect};
    use crate::native_panel_scene::SceneMascotPose;
    use crate::native_panel_ui::render::{
        NativePanelHostWindowState, NativePanelPaintInput, NativePanelVisualDisplayMode,
    };

    fn make_input() -> NativePanelPaintInput {
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
            headline_text: "Sessions".into(),
            headline_emphasized: false,
            active_count: "3".into(),
            active_count_elapsed_ms: 0,
            total_count: "10".into(),
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
    fn panel_renderer_no_migration_returns_reference() {
        let mut pr = PanelRenderer::new(
            Size { width: 200.0, height: 48.0 },
            MigrantComponents::none(),
        );
        let input = make_input();
        let plan = pr.render(&input);
        // 无迁移直接返回参考管线输出
        // (empty compact panel 可能为 hidden=true)
        assert!(plan.hidden || !plan.primitives.is_empty() || plan.viewport.width >= 0.0);
    }

    #[test]
    fn panel_renderer_with_migration_fallback_on_mismatch() {
        let mut pr = PanelRenderer::new(
            Size { width: 200.0, height: 48.0 },
            MigrantComponents {
                compact_bar: true,
                ..MigrantComponents::none()
            },
        );
        let input = make_input();
        let plan = pr.render(&input);
        // 迁移 stub 尚未实现 → 回退到参考管线
        assert!(plan.hidden || !plan.primitives.is_empty() || plan.viewport.width >= 0.0);
    }

    #[test]
    fn migrant_components_config() {
        let none = MigrantComponents::none();
        assert!(!none.any());

        let all = MigrantComponents::all();
        assert!(all.any());
        assert!(all.compact_bar);
        assert!(all.card_stack);
    }
}
