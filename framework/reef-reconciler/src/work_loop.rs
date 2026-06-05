use crate::arena::FiberArena;
use crate::fiber::{EffectTag, ElementTypeRef};
use crate::host_config::{HostConfig, HostInstanceId};
use crate::reconcile::reconcile_children;
use reef_hooks::{begin_fiber, end_fiber, FiberId};
use reef_vnode::{ElementType, VNode};

/// Result of a single work loop iteration.
#[derive(Debug, PartialEq, Eq)]
pub enum WorkResult {
    /// All work is done for this frame.
    Completed,
    /// Work was interrupted by the deadline — yields to the scheduler.
    Yield,
}

/// The reconciler's work loop — processes fiber units of work with
/// deadline-based yielding.
///
/// Two phases:
/// 1. **Render phase** (interruptible): traverse fiber tree, reconcile VNodes,
///    run hooks, collect effects.
/// 2. **Commit phase** (not interruptible): apply effects to the host platform.
pub struct WorkLoop {
    /// The next fiber to process, or `None` if all work is done.
    pub next_unit_of_work: Option<FiberId>,
    /// The root fiber of the work-in-progress tree.
    pub wip_root: Option<FiberId>,
    /// The host instance ID for the wip root.
    pub root_instance_id: Option<HostInstanceId>,
    /// Whether we're in the middle of a commit phase.
    committing: bool,
}

impl WorkLoop {
    pub fn new() -> Self {
        Self {
            next_unit_of_work: None,
            wip_root: None,
            root_instance_id: None,
            committing: false,
        }
    }

    /// Perform one unit of work on the given fiber.
    ///
    /// Returns the next fiber to process, or `None` if the render phase
    /// is complete (commit should follow).
    pub fn perform_unit_of_work(
        &mut self,
        arena: &mut FiberArena,
        fiber_id: FiberId,
        ty: &ElementTypeRef,
    ) -> Option<FiberId> {
        // Set current fiber for hooks
        begin_fiber(fiber_id);

        match ty {
            ElementTypeRef::Function => {
                // Function component: call the stored fn ptr to get VNode tree
                let fn_ptr = arena.get(fiber_id).component_fn;
                if let Some(f) = fn_ptr {
                    let props = arena.get(fiber_id).pending_props.clone();
                    let vnode = f(&props);
                    // Store the ENTIRE returned VNode as a single child fiber
                    // This ensures the returned Native element gets a fiber + host instance
                    match &vnode {
                        reef_vnode::VNode::VEmpty => {}
                        _ => {
                            arena.get_mut(fiber_id).pending_vnode_children = Some(vec![vnode]);
                        }
                    }
                }
            }
            ElementTypeRef::Native(_) => {}
        }

        // Reconcile pending VNode children if any
        let has_pending = arena.get(fiber_id).pending_vnode_children.is_some();
        if has_pending {
            let children = arena.get_mut(fiber_id).pending_vnode_children.take().unwrap_or_default();
            if !children.is_empty() {
                reconcile_children(arena, fiber_id, children, ty);
            }
        }

        end_fiber();

        // If this fiber has a child, process child next (DFS down)
        let f = arena.get(fiber_id);
        if f.child.is_some() {
            return f.child;
        }

        // Otherwise, bubble up
        self.bubble_up(arena, fiber_id)
    }

    /// Walk up the fiber tree to find the next unit of work.
    ///
    /// Returns the next sibling when found, or walks up to find a parent
    /// with a sibling, or returns `None` when reaching the root.
    fn bubble_up(&self, arena: &FiberArena, fiber_id: FiberId) -> Option<FiberId> {
        let mut current = fiber_id;
        loop {
            let fiber = arena.get(current);
            if fiber.sibling.is_some() {
                return fiber.sibling;
            }
            match fiber.return_to {
                Some(parent) => current = parent,
                None => return None, // Reached the root
            }
        }
    }

    /// Reconcile the root fiber's children with new VNodes.
    ///
    /// Called externally with the new VNode tree from a component render.
    pub fn reconcile_root(
        &mut self,
        arena: &mut FiberArena,
        root_id: FiberId,
        new_children: Vec<VNode>,
    ) {
        let root_ty = arena.get(root_id).element_type.clone();
        reconcile_children(arena, root_id, new_children, &root_ty);
        self.next_unit_of_work = arena.get(root_id).child;
    }

    /// The main work loop — processes fibers until the deadline or completion.
    ///
    /// After the render phase completes, automatically enters the commit phase.
    pub fn work_loop(
        &mut self,
        arena: &mut FiberArena,
        deadline: &std::time::Instant,
        host: &mut dyn HostConfig,
    ) -> WorkResult {
        // Render phase (interruptible)
        while let Some(fiber_id) = self.next_unit_of_work {
            if std::time::Instant::now() >= *deadline {
                return WorkResult::Yield;
            }

            let ty = arena.get(fiber_id).element_type.clone();
            self.next_unit_of_work = self.perform_unit_of_work(arena, fiber_id, &ty);
        }

        // Render phase complete — commit
        if self.next_unit_of_work.is_none() && !self.committing {
            self.committing = true;
            if let Some(root_id) = self.wip_root {
                self.commit_root(arena, root_id, host);
            }
            self.committing = false;
        }

        WorkResult::Completed
    }

    /// Walk the effect list and apply mutations to the host.
    ///
    /// Uses a DFS traversal of the fiber tree to apply effects in order.
    /// For Placement: creates host instance, stores ID on fiber, appends to parent.
    /// For Update: calls update_instance on the host.
    /// For Deletion: calls remove_instance on the host.
    fn commit_root(
        &mut self,
        arena: &mut FiberArena,
        root_id: FiberId,
        host: &mut dyn HostConfig,
    ) {
        self.commit_fiber(arena, root_id, host);
    }

    fn commit_fiber(
        &mut self,
        arena: &mut FiberArena,
        fiber_id: FiberId,
        host: &mut dyn HostConfig,
    ) {
        let (effect_tag, element_type) = {
            let f = arena.get(fiber_id);
            (f.effect_tag, f.element_type.clone())
        };

        match effect_tag {
            EffectTag::Placement => {
                match &element_type {
                    ElementTypeRef::Native(_) => {
                        let (ty, props, return_to) = {
                            let f = arena.get(fiber_id);
                            (ElementType::Native(
                                if let ElementTypeRef::Native(n) = &f.element_type { n } else { "" }
                            ), f.pending_props.clone(), f.return_to)
                        };
                        let inst = host.create_instance(&ty, &props);
                        arena.get_mut(fiber_id).host_instance_id = Some(inst);

                        // Append to parent's host instance
                        if let Some(rt) = return_to {
                            if let Some(parent_inst) = find_parent_host_instance(arena, rt) {
                                host.append_child(parent_inst, inst);
                            }
                        }
                    }
                    ElementTypeRef::Function => {
                        // Function components don't produce host instances
                    }
                }
                self.commit_children(arena, fiber_id, host);
            }
            EffectTag::Update => {
                let props = arena.get(fiber_id).pending_props.clone();
                if let Some(inst) = arena.get(fiber_id).host_instance_id {
                    match &element_type {
                        ElementTypeRef::Native(_) => {
                            host.update_instance(inst, &props);
                        }
                        ElementTypeRef::Function => {}
                    }
                }
                self.commit_children(arena, fiber_id, host);
            }
            EffectTag::Deletion => {
                if let Some(inst) = arena.get(fiber_id).host_instance_id {
                    host.remove_instance(inst);
                }
                // Don't recurse into deleted subtree
            }
            EffectTag::NoEffect => {
                self.commit_children(arena, fiber_id, host);
            }
        }
    }

    fn commit_children(&mut self, arena: &mut FiberArena, fiber_id: FiberId, host: &mut dyn HostConfig) {
        let child = arena.get(fiber_id).child;
        let mut cursor = child;
        while let Some(cid) = cursor {
            self.commit_fiber(arena, cid, host);
            cursor = arena.get(cid).sibling;
        }
    }
}

/// Walk up the fiber tree to find the nearest ancestor with a host instance.
fn find_parent_host_instance(arena: &FiberArena, mut fiber_id: FiberId) -> Option<HostInstanceId> {
    loop {
        let f = arena.get(fiber_id);
        if let Some(inst) = f.host_instance_id {
            return Some(inst);
        }
        match f.return_to {
            Some(parent) => fiber_id = parent,
            None => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::FiberArena;
    use crate::fiber::FiberNode;
    use std::time::{Duration, Instant};

    struct TestHost;

    impl HostConfig for TestHost {
        fn create_instance(&mut self, _ty: &ElementType, _props: &reef_vnode::PropsMap) -> HostInstanceId {
            HostInstanceId(0)
        }
        fn append_child(&mut self, _parent: HostInstanceId, _child: HostInstanceId) {}
        fn update_instance(&mut self, _instance: HostInstanceId, _props: &reef_vnode::PropsMap) {}
        fn remove_instance(&mut self, _instance: HostInstanceId) {}
    }

    #[test]
    fn work_loop_immediate_complete() {
        let mut arena = FiberArena::new();
        let root = arena.alloc(FiberNode::new(ElementTypeRef::Native("root")));
        arena.get_mut(root).id = root;

        let mut wl = WorkLoop::new();
        wl.wip_root = Some(root);
        wl.next_unit_of_work = Some(root);

        let deadline = Instant::now() + Duration::from_secs(1);
        let mut host = TestHost;
        let result = wl.work_loop(&mut arena, &deadline, &mut host);

        assert_eq!(result, WorkResult::Completed);
    }

    #[test]
    fn work_loop_yields_on_deadline() {
        let mut arena = FiberArena::new();
        let root = arena.alloc(FiberNode::new(ElementTypeRef::Native("root")));
        arena.get_mut(root).id = root;

        let mut wl = WorkLoop::new();
        wl.wip_root = Some(root);
        wl.next_unit_of_work = Some(root);

        // Deadline already past → should yield immediately
        let deadline = Instant::now() - Duration::from_millis(1);
        let mut host = TestHost;
        let result = wl.work_loop(&mut arena, &deadline, &mut host);

        assert_eq!(result, WorkResult::Yield);
    }

    #[test]
    fn bubble_up_to_sibling() {
        let mut arena = FiberArena::new();
        let root = arena.alloc(FiberNode::new(ElementTypeRef::Native("root")));
        arena.get_mut(root).id = root;

        let child_a = arena.alloc(FiberNode::new(ElementTypeRef::Native("label")));
        arena.get_mut(child_a).id = child_a;
        arena.get_mut(child_a).return_to = Some(root);

        let child_b = arena.alloc(FiberNode::new(ElementTypeRef::Native("container")));
        arena.get_mut(child_b).id = child_b;
        arena.get_mut(child_b).return_to = Some(root);

        arena.get_mut(child_a).sibling = Some(child_b);
        arena.get_mut(root).child = Some(child_a);

        let wl = WorkLoop::new();
        // child_a's sibling is child_b
        let next = wl.bubble_up(&arena, child_a);
        assert_eq!(next, Some(child_b));
    }

    #[test]
    fn work_loop_reconciles_root() {
        let mut arena = FiberArena::new();
        let root = arena.alloc(FiberNode::new(ElementTypeRef::Native("root")));
        arena.get_mut(root).id = root;

        let mut wl = WorkLoop::new();
        wl.wip_root = Some(root);

        wl.reconcile_root(
            &mut arena,
            root,
            vec![VNode::VElement(reef_vnode::VElement {
                ty: ElementType::Native("label"),
                props: reef_vnode::PropsMap::new(),
                children: vec![],
                key: None,
            })],
        );

        // Should have a child
        let root_ref = arena.get(root);
        assert!(root_ref.child.is_some());
        // next_unit_of_work should be the child
        assert!(wl.next_unit_of_work.is_some());
    }
}
