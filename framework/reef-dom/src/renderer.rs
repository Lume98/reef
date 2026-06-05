use crate::host_config::ReefDomConfig;
use crate::layout::layout_scene;
use crate::paint::paint_scene_to_plan;
use reef_core::geometry::Size;
use reef_draw::primitive::DrawPlan;
use reef_layout::Constraints;
use reef_reconciler::arena::FiberArena;
use reef_reconciler::fiber::{EffectTag, ElementTypeRef, FiberNode};
use reef_reconciler::work_loop::WorkLoop;
use reef_hooks::FiberId;
use reef_vnode::VNode;
use std::time::{Duration, Instant};

/// A high-level renderer that transforms VNode trees into DrawPlans.
///
/// Internally pipelines:
///   1. Fiber reconciliation (VNode → fiber tree + effects)
///   2. Commit (effects → ReefDomConfig scene tree)
///   3. Layout (constraints-based positioning)
///   4. Paint (SceneNode → DrawPrimitives → DrawPlan)
///
/// # Example
/// ```ignore
/// let mut renderer = ReefRenderer::new(Size { width: 200.0, height: 100.0 });
/// let plan = renderer.render(rsx! {
///     <container color={Color::rgb(18, 18, 22)} radius={12.0}>
///         <label text={"Hello"} />
///     </container>
/// });
/// // plan is a DrawPlan ready for DrawBackend::submit_frame()
/// ```
pub struct ReefRenderer {
    arena: FiberArena,
    dom_config: ReefDomConfig,
    work_loop: WorkLoop,
    wip_root: Option<FiberId>,
    viewport: Size,
    /// If true, resets the entire pipeline each frame (no incremental updates).
    fresh_each_frame: bool,
}

impl ReefRenderer {
    /// Create a new renderer with the given viewport size.
    pub fn new(viewport: Size) -> Self {
        Self {
            arena: FiberArena::new(),
            dom_config: ReefDomConfig::new(),
            work_loop: WorkLoop::new(),
            wip_root: None,
            viewport,
            fresh_each_frame: true,
        }
    }

    /// Render a VNode tree into a DrawPlan.
    ///
    /// This runs the full reconciliation + layout + paint pipeline.
    pub fn render(&mut self, vnode: VNode) -> DrawPlan {
        // Reset for fresh frame if enabled
        if self.fresh_each_frame {
            self.reset();
        }

        // Initialize root fiber on first render
        if self.wip_root.is_none() {
            let root_id = self.create_root_fiber();
            self.wip_root = Some(root_id);
            self.work_loop.wip_root = Some(root_id);
        }

        let root_id = self.wip_root.unwrap();

        // 1. Reconcile root with new VNode
        self.work_loop
            .reconcile_root(&mut self.arena, root_id, vec![vnode]);

        // 2. Process all fibers (render phase + commit phase)
        let deadline = Instant::now() + Duration::from_millis(16);
        self.work_loop
            .work_loop(&mut self.arena, &deadline, &mut self.dom_config);

        // 3. Build scene tree from committed state
        let scene = match self.dom_config.build_scene_tree() {
            Some(scene) => scene,
            None => {
                return DrawPlan::with_viewport(self.viewport);
            }
        };

        // 4. Layout
        let mut scene = scene;
        let constraints = Constraints::loose(self.viewport);
        layout_scene(&mut scene, constraints);

        // 5. Paint to DrawPlan
        paint_scene_to_plan(&scene, self.viewport)
    }

    /// Resize the viewport.
    pub fn set_viewport(&mut self, size: Size) {
        self.viewport = size;
    }

    /// Get the current viewport size.
    pub fn viewport(&self) -> Size {
        self.viewport
    }

    /// Reset the entire pipeline (clear arena and dom config).
    pub fn reset(&mut self) {
        self.arena = FiberArena::new();
        self.dom_config = ReefDomConfig::new();
        self.work_loop = WorkLoop::new();
        self.wip_root = None;
    }

    fn create_root_fiber(&mut self) -> FiberId {
        let mut root = FiberNode::new(ElementTypeRef::Native("$root"));
        root.effect_tag = EffectTag::Placement;
        let id = self.arena.alloc(root);
        self.arena.get_mut(id).id = id;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::color::Color;
    use reef_vnode::{ElementType, PropsMap, VElement};

    #[test]
    fn renderer_creates_draw_plan() {
        let mut renderer = ReefRenderer::new(Size {
            width: 200.0,
            height: 100.0,
        });

        let vnode = VNode::VElement(VElement {
            ty: ElementType::Native("container"),
            props: {
                let mut p = PropsMap::new();
                p.insert("color", Color::rgb(18, 18, 22));
                p.insert("radius", 12.0_f64);
                p
            },
            children: vec![VNode::VElement(VElement {
                ty: ElementType::Native("label"),
                props: {
                    let mut p = PropsMap::new();
                    p.insert("text", "Hello");
                    p
                },
                children: vec![],
                key: None,
            })],
            key: None,
        });

        let plan = renderer.render(vnode);

        assert!(!plan.hidden);
        assert_eq!(plan.viewport.width, 200.0);
        // Should produce primitives: RoundRect for container + Clip + Text + ClipEnd
        assert!(!plan.primitives.is_empty(), "expected primitives in plan");
    }

    #[test]
    fn renderer_multiple_frames() {
        let mut renderer = ReefRenderer::new(Size {
            width: 100.0,
            height: 50.0,
        });

        // First frame: colored container (no children)
        let plan1 = renderer.render(VNode::VElement(VElement {
            ty: ElementType::Native("container"),
            props: {
                let mut p = PropsMap::new();
                p.insert("color", Color::rgb(18, 18, 22));
                p
            },
            children: vec![],
            key: None,
        }));
        assert!(plan1.primitives.len() >= 1, "expected at least 1 primitive, got {}", plan1.primitives.len());

        // Second frame: different color
        let plan2 = renderer.render(VNode::VElement(VElement {
            ty: ElementType::Native("container"),
            props: {
                let mut p = PropsMap::new();
                p.insert("color", Color::rgb(255, 0, 0));
                p
            },
            children: vec![],
            key: None,
        }));
        assert!(plan2.primitives.len() >= 1, "expected at least 1 primitive, got {}", plan2.primitives.len());
    }

    #[test]
    fn renderer_empty_vnode() {
        let mut renderer = ReefRenderer::new(Size {
            width: 100.0,
            height: 100.0,
        });

        let plan = renderer.render(VNode::VEmpty);
        // Empty VNode should produce a plan with no primitives
        assert_eq!(plan.primitives.len(), 0);
        assert_eq!(plan.viewport.width, 100.0);
    }

    #[test]
    fn renderer_with_reef_dom_macro() {
        // This test uses rsx! to verify the full pipeline works
        let mut renderer = ReefRenderer::new(Size {
            width: 300.0,
            height: 200.0,
        });

        let vnode = reef_view_macros::rsx! {
            <container color={Color::rgb(18, 18, 22)} radius={16.0}>
                <label text={"Hello Reef"} />
            </container>
        };

        let plan = renderer.render(vnode);
        eprintln!("  plan has {} primitives", plan.primitives.len());
        for (i, p) in plan.primitives.iter().enumerate() {
            eprintln!("  primitive[{}]: {:?}", i, p);
        }
        assert!(!plan.primitives.is_empty(), "expected non-empty plan, got {} primitives", plan.primitives.len());

        // Verify we got the expected primitive types
        let has_round_rect = plan.primitives.iter().any(|p| matches!(p, reef_draw::DrawPrimitive::RoundRect { .. }));
        let has_text = plan.primitives.iter().any(|p| matches!(p, reef_draw::DrawPrimitive::Text { .. }));

        assert!(has_round_rect, "expected RoundRect in rendered plan");
        assert!(has_text, "expected Text in rendered plan");
    }
}
