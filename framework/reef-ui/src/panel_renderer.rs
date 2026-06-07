//! 灵动岛逐组件迁移渲染器。
//!
//! 将 `NativePanelPaintInput` 同时通过新旧两条管线渲染，逐 primitive 对比，
//! 零差异才切换到新管线。每个灵动岛组件可以独立控制迁移状态。

use crate::native_panel_ui::render::{
    resolve_native_panel_visual_plan, resolve_native_panel_widget_draw_plan, NativePanelPaintInput,
};
use crate::scene_bridge::compare_draw_plans;
use reef_core::geometry::Size;
use reef_draw::primitive::DrawPlan;

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
    migrant: MigrantComponents,
    viewport: Size,
}

impl PanelRenderer {
    /// 创建新的 PanelRenderer。
    pub fn new(viewport: Size, migrant: MigrantComponents) -> Self {
        Self { migrant, viewport }
    }

    /// 设置视口大小。
    pub fn set_viewport(&mut self, viewport: Size) {
        self.viewport = viewport;
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

        let _ = self.viewport;

        // 有组件迁移 → 通过组件库适配层渲染候选 DrawPlan
        let candidate = resolve_native_panel_widget_draw_plan(input);

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_panel_core::{ExpandedSurface, PanelRect};
    use crate::native_panel_scene::SceneMascotPose;
    use crate::native_panel_ui::render::{
        resolve_native_panel_widget_draw_plan, NativePanelHostWindowState, NativePanelPaintInput,
        NativePanelVisualCardBadgeInput, NativePanelVisualCardInput, NativePanelVisualCardStyle,
        NativePanelVisualDisplayMode,
    };

    fn make_input() -> NativePanelPaintInput {
        NativePanelPaintInput {
            window_state: NativePanelHostWindowState {
                visible: true,
                ..NativePanelHostWindowState::default()
            },
            display_mode: NativePanelVisualDisplayMode::Compact,
            surface: ExpandedSurface::Default,
            panel_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 48.0,
            },
            compact_bar_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 48.0,
            },
            left_shoulder_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            right_shoulder_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            shoulder_progress: 0.0,
            content_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            card_stack_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            card_stack_content_height: 0.0,
            shell_frame: PanelRect {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 48.0,
            },
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
            Size {
                width: 200.0,
                height: 48.0,
            },
            MigrantComponents::none(),
        );
        let input = make_input();
        let reference = resolve_native_panel_visual_plan(&input);

        let plan = pr.render(&input);

        assert_eq!(plan, reference);
    }

    #[test]
    fn migration_returns_widget_candidate_when_identical() {
        let mut input = make_input();
        input.display_mode = NativePanelVisualDisplayMode::Hidden;
        input.window_state.visible = false;

        let mut pr = PanelRenderer::new(
            Size {
                width: 200.0,
                height: 48.0,
            },
            MigrantComponents::all(),
        );
        let plan = pr.render(&input);
        let widget_plan = resolve_native_panel_widget_draw_plan(&input);

        assert_eq!(plan, widget_plan);
    }

    #[test]
    fn migration_falls_back_to_reference_when_candidate_differs() {
        let input = make_input();
        let reference = resolve_native_panel_visual_plan(&input);
        let widget_plan = resolve_native_panel_widget_draw_plan(&input);
        let precondition = compare_draw_plans(&reference, &widget_plan);
        assert!(
            !precondition.is_identical(),
            "test fixture must exercise fallback path"
        );

        let mut pr = PanelRenderer::new(
            Size {
                width: 200.0,
                height: 48.0,
            },
            MigrantComponents::all(),
        );
        let plan = pr.render(&input);

        assert_eq!(plan, reference);
    }

    #[test]
    fn expanded_card_input_is_safe_to_render_through_migration_gate() {
        let mut input = make_input();
        input.display_mode = NativePanelVisualDisplayMode::Expanded;
        input.headline_text = "Sessions".into();
        input.active_count = "3".into();
        input.total_count = "10".into();
        input.cards_visible = true;
        input.card_count = 1;
        input.cards = vec![NativePanelVisualCardInput {
            style: NativePanelVisualCardStyle::Default,
            title: "Test Card".into(),
            subtitle: Some("sub".into()),
            body: None,
            badge: Some(NativePanelVisualCardBadgeInput {
                text: "new".into(),
                emphasized: false,
            }),
            source_badge: None,
            body_prefix: None,
            body_lines: vec![],
            action_hint: Some("click".into()),
            rows: vec![],
            height: 120.0,
            collapsed_height: 40.0,
            compact: false,
            removing: false,
        }];
        input.compact_bar_frame = PanelRect {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 48.0,
        };
        input.shell_frame = PanelRect {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 300.0,
        };
        input.card_stack_frame = PanelRect {
            x: 0.0,
            y: 48.0,
            width: 300.0,
            height: 200.0,
        };
        input.card_stack_content_height = 200.0;
        input.content_frame = PanelRect {
            x: 0.0,
            y: 48.0,
            width: 300.0,
            height: 200.0,
        };
        input.panel_frame = PanelRect {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 300.0,
        };
        input.separator_visibility = 1.0;

        let reference = resolve_native_panel_visual_plan(&input);

        let mut pr = PanelRenderer::new(
            Size {
                width: 200.0,
                height: 48.0,
            },
            MigrantComponents::all(),
        );
        let plan = pr.render(&input);

        assert_eq!(plan, reference);
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
