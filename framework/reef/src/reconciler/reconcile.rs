use crate::hooks::FiberId;
use crate::reconciler::arena::FiberArena;
use crate::reconciler::fiber::{EffectTag, ElementTypeRef, FiberNode};
use crate::vnode::{ElementType, VNode};
use std::collections::HashMap;

/// Reconcile new VNode children against the existing child fiber chain.
///
/// Walks the new children list and produces a new sibling-linked chain of
/// fiber nodes. Mutates `wip_fiber` to point to the first new child.
///
/// Uses a key-based diff algorithm:
/// - Same key + same type → UPDATE (reuse fiber, set pending props)
/// - Same key + diff type → REPLACE (delete old, place new)
/// - New key not in old tree → PLACEMENT
/// - Old key not in new tree → DELETION
pub fn reconcile_children(
    arena: &mut FiberArena,
    wip_id: FiberId,
    new_children: Vec<VNode>,
    _ty: &ElementTypeRef,
) {
    // Collect old children from the alternate fiber's child chain
    let old_children = collect_old_children(arena, wip_id);

    // Build old key → fiber index map
    let mut old_map: HashMap<Option<String>, FiberId> = HashMap::new();
    for (_idx, old_id) in &old_children {
        let old = arena.get(*old_id);
        old_map.insert(old.key.clone(), *old_id);
    }

    let mut prev_child: Option<FiberId> = None;
    let mut first_child: Option<FiberId> = None;
    // Track processed old fibers to detect deletions
    let mut remaining_old: Vec<FiberId> = old_children.iter().map(|(_, id)| *id).collect();

    for (new_index, child_vnode) in new_children.into_iter().enumerate() {
        match child_vnode {
            VNode::VEmpty => continue,
            VNode::VText(text) => {
                let new_fiber = create_text_fiber(arena, &text, wip_id, new_index);
                old_map.retain(|_, id| *id != new_fiber);
                remaining_old.retain(|id| *id != new_fiber);

                if let Some(prev) = prev_child {
                    arena.get_mut(prev).sibling = Some(new_fiber);
                } else {
                    first_child = Some(new_fiber);
                }
                prev_child = Some(new_fiber);
            }
            VNode::VElement(el) => {
                let el_key = el.key.clone();
                let el_ty = &el.ty;
                let el_props = el.props.clone();
                let el_children = el.children.clone();

                // Try to find a matching old fiber by key, then by type
                let old_id = old_map.remove(&el_key).or_else(|| {
                    // Fallback: no key match — check type-matched from remaining
                    remaining_old
                        .iter()
                        .position(|id| {
                            let f = arena.get(*id);
                            types_match(&f.element_type, el_ty)
                        })
                        .map(|pos| remaining_old.remove(pos))
                });

                let new_fiber = if let Some(old_id) = old_id {
                    // REUSE: update existing fiber with new props
                    let old = arena.get(old_id);
                    let same = types_match(&old.element_type, el_ty);
                    if same {
                        // UPDATE: reuse fiber
                        arena.get_mut(old_id).pending_props = el_props;
                        arena.get_mut(old_id).effect_tag = EffectTag::Update;
                        arena.get_mut(old_id).key = el_key;
                        arena.get_mut(old_id).return_to = Some(wip_id);
                        // Store children for later reconciliation
                        if !el_children.is_empty() {
                            arena.get_mut(old_id).pending_vnode_children = Some(el_children);
                        }
                        old_id
                    } else {
                        // Type mismatch — delete old, create new
                        arena.get_mut(old_id).effect_tag = EffectTag::Deletion;
                        create_element_fiber_with_children(
                            arena,
                            el_ty,
                            el_props,
                            el_key,
                            el_children,
                            wip_id,
                            new_index,
                        )
                    }
                } else {
                    // PLACEMENT: create new fiber
                    create_element_fiber_with_children(
                        arena,
                        el_ty,
                        el_props,
                        el_key,
                        el_children,
                        wip_id,
                        new_index,
                    )
                };

                remaining_old.retain(|id| *id != new_fiber);

                if let Some(prev) = prev_child {
                    arena.get_mut(prev).sibling = Some(new_fiber);
                } else {
                    first_child = Some(new_fiber);
                }
                prev_child = Some(new_fiber);
            }
            VNode::VFragment(children) => {
                // Flatten fragment children — reconcile them inline
                // For simplicity, collect and recurse
                let mut inner_prev = prev_child;
                for inner_child in children {
                    match inner_child {
                        VNode::VEmpty => continue,
                        VNode::VText(text) => {
                            let f = create_text_fiber(arena, &text, wip_id, new_index);
                            remaining_old.retain(|id| *id != f);
                            if let Some(prev) = inner_prev {
                                arena.get_mut(prev).sibling = Some(f);
                            } else if first_child.is_none() {
                                first_child = Some(f);
                            }
                            inner_prev = Some(f);
                        }
                        VNode::VElement(el) => {
                            let f = create_element_fiber_with_children(
                                arena,
                                &el.ty,
                                el.props,
                                el.key,
                                el.children,
                                wip_id,
                                new_index,
                            );
                            remaining_old.retain(|id| *id != f);
                            if let Some(prev) = inner_prev {
                                arena.get_mut(prev).sibling = Some(f);
                            } else if first_child.is_none() {
                                first_child = Some(f);
                            }
                            inner_prev = Some(f);
                        }
                        VNode::VFragment(_) => {
                            // Nested fragments — skip for now, Phase 3 limitation
                            continue;
                        }
                    }
                }
                prev_child = inner_prev;
            }
        }
    }

    // Mark remaining old fibers as deleted
    for old_id in remaining_old {
        arena.get_mut(old_id).effect_tag = EffectTag::Deletion;
    }

    // Update wip fiber's child pointer
    arena.get_mut(wip_id).child = first_child;
}

// ── Helpers ───────────────────────────────────────────────────────

fn collect_old_children(arena: &FiberArena, wip_id: FiberId) -> Vec<(usize, FiberId)> {
    let mut result = Vec::new();
    if let Some(alt_id) = arena.get(wip_id).alternate {
        let alt = arena.get(alt_id);
        let mut cursor = alt.child;
        let mut index = 0;
        // Walk through the alternate's children (its `child` → siblings chain)
        while let Some(child_id) = cursor {
            // The child in alternate is in the old arena; for a fresh reconciliation,
            // we look at the actual child chain. But for matching, we need to check
            // if the child was already reconciled (cloned) or if it's still the original.
            //
            // For simplicity: if the alternate's child exists and has no effect_tag yet,
            // it's an "old child" eligible for reuse.
            if arena.get(child_id).effect_tag != EffectTag::Deletion {
                result.push((index, child_id));
            }
            cursor = arena.get(child_id).sibling;
            index += 1;
        }
    }
    result
}

fn types_match(fiber_ty: &ElementTypeRef, vnode_ty: &ElementType) -> bool {
    match (fiber_ty, vnode_ty) {
        (ElementTypeRef::Native(a), ElementType::Native(b)) => a == b,
        (ElementTypeRef::Function, ElementType::Function(_)) => true,
        _ => false,
    }
}

fn create_text_fiber(
    arena: &mut FiberArena,
    _text: &str,
    parent_id: FiberId,
    _index: usize,
) -> FiberId {
    let mut fiber = FiberNode::new(ElementTypeRef::Native("#text"));
    fiber.key = None;
    fiber.return_to = Some(parent_id);
    fiber.effect_tag = EffectTag::Placement;
    let id = arena.alloc(fiber);
    // Fix: set the ID after allocation
    arena.get_mut(id).id = id;
    id
}

fn create_element_fiber(
    arena: &mut FiberArena,
    ty: &ElementType,
    props: crate::vnode::PropsMap,
    key: Option<String>,
    parent_id: FiberId,
    _index: usize,
) -> FiberId {
    let element_ref: ElementTypeRef = ty.into();
    let mut fiber = FiberNode::new(element_ref);
    fiber.pending_props = props;
    fiber.key = key;
    fiber.return_to = Some(parent_id);
    fiber.effect_tag = EffectTag::Placement;
    // Store function pointer for Function types
    if let ElementType::Function(f) = ty {
        fiber.component_fn = Some(*f);
    }
    let id = arena.alloc(fiber);
    arena.get_mut(id).id = id;
    id
}

/// Create an element fiber and store its VNode children for later reconciliation.
fn create_element_fiber_with_children(
    arena: &mut FiberArena,
    ty: &ElementType,
    props: crate::vnode::PropsMap,
    key: Option<String>,
    children: Vec<VNode>,
    parent_id: FiberId,
    _index: usize,
) -> FiberId {
    let id = create_element_fiber(arena, ty, props, key, parent_id, _index);
    if !children.is_empty() {
        arena.get_mut(id).pending_vnode_children = Some(children);
    }
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vnode::PropsMap;

    #[test]
    fn reconcile_empty_children() {
        let mut arena = FiberArena::new();
        let root = arena.alloc(FiberNode::new(ElementTypeRef::Native("root")));
        arena.get_mut(root).id = root;

        reconcile_children(&mut arena, root, vec![], &ElementTypeRef::Native("root"));

        let root_ref = arena.get(root);
        assert!(root_ref.child.is_none());
    }

    #[test]
    fn reconcile_single_child_placement() {
        let mut arena = FiberArena::new();
        let root = arena.alloc(FiberNode::new(ElementTypeRef::Native("root")));
        arena.get_mut(root).id = root;

        let child = VNode::VElement(crate::vnode::VElement {
            ty: ElementType::Native("label"),
            props: PropsMap::new(),
            children: vec![],
            key: None,
        });

        reconcile_children(
            &mut arena,
            root,
            vec![child],
            &ElementTypeRef::Native("root"),
        );

        let root_ref = arena.get(root);
        assert!(root_ref.child.is_some());

        let child_id = root_ref.child.unwrap();
        let child_fiber = arena.get(child_id);
        assert_eq!(child_fiber.element_type, ElementTypeRef::Native("label"));
        assert_eq!(child_fiber.effect_tag, EffectTag::Placement);
        assert_eq!(child_fiber.return_to, Some(root));
    }

    #[test]
    fn reconcile_multiple_children() {
        let mut arena = FiberArena::new();
        let root = arena.alloc(FiberNode::new(ElementTypeRef::Native("root")));
        arena.get_mut(root).id = root;

        let children = vec![
            VNode::VElement(crate::vnode::VElement {
                ty: ElementType::Native("label"),
                props: PropsMap::new(),
                children: vec![],
                key: Some("a".into()),
            }),
            VNode::VElement(crate::vnode::VElement {
                ty: ElementType::Native("container"),
                props: PropsMap::new(),
                children: vec![],
                key: Some("b".into()),
            }),
        ];

        reconcile_children(&mut arena, root, children, &ElementTypeRef::Native("root"));

        let root_ref = arena.get(root);
        let first = root_ref.child.unwrap();
        assert_eq!(arena.get(first).key.as_deref(), Some("a"));

        let second = arena.get(first).sibling.unwrap();
        assert_eq!(arena.get(second).key.as_deref(), Some("b"));
        assert!(arena.get(second).sibling.is_none());
    }

    #[test]
    fn reconcile_text_children() {
        let mut arena = FiberArena::new();
        let root = arena.alloc(FiberNode::new(ElementTypeRef::Native("root")));
        arena.get_mut(root).id = root;

        reconcile_children(
            &mut arena,
            root,
            vec![VNode::VText("hello".into())],
            &ElementTypeRef::Native("root"),
        );

        let child = arena.get(root).child.unwrap();
        assert_eq!(
            arena.get(child).element_type,
            ElementTypeRef::Native("#text")
        );
        assert_eq!(arena.get(child).effect_tag, EffectTag::Placement);
    }

    #[test]
    fn reconcile_skips_empty() {
        let mut arena = FiberArena::new();
        let root = arena.alloc(FiberNode::new(ElementTypeRef::Native("root")));
        arena.get_mut(root).id = root;

        reconcile_children(
            &mut arena,
            root,
            vec![VNode::VEmpty, VNode::VText("real".into()), VNode::VEmpty],
            &ElementTypeRef::Native("root"),
        );

        // Should only have one child (the text node)
        let child = arena.get(root).child.unwrap();
        assert_eq!(
            arena.get(child).element_type,
            ElementTypeRef::Native("#text")
        );
        assert!(arena.get(child).sibling.is_none());
    }
}
