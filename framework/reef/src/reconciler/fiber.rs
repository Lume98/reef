use crate::hooks::FiberId;
use crate::reconciler::host_config::HostInstanceId;
use crate::vnode::{ElementType, PropsMap, VNode};
use std::ptr::fn_addr_eq;

/// Simplified element type for storage in fiber nodes.
/// Avoids storing function pointers inline — instead uses a stable ref.
#[derive(Clone, Debug, PartialEq)]
pub enum ElementTypeRef {
    /// A native platform element.
    Native(&'static str),
    /// A function component (stored as a stable ID — the function
    /// is called via a side-channel during reconciliation).
    Function,
}

/// Tag describing what operation the commit phase must perform.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EffectTag {
    NoEffect,
    Placement,
    Update,
    Deletion,
}

/// A fiber node — the fundamental unit of work in the reconciler.
///
/// Each fiber represents a component instance or native element in the
/// UI tree. Fibers form a tree via `child`, `sibling`, `return_to`, and
/// link to their previous-frame counterpart via `alternate`.
#[derive(Clone, Debug)]
pub struct FiberNode {
    pub id: FiberId,

    // ── Element info ──────────────────────────────────────────────
    /// The type of element this fiber represents.
    pub element_type: ElementTypeRef,
    /// Reconciliation key.
    pub key: Option<String>,
    /// Props pending reconciliation — set by the caller before work begins.
    pub pending_props: PropsMap,
    /// Props from the previous reconciliation.
    pub memoized_props: PropsMap,

    // ── Tree links (arena indices) ────────────────────────────────
    /// First child fiber.
    pub child: Option<FiberId>,
    /// Next sibling fiber.
    pub sibling: Option<FiberId>,
    /// Parent fiber.
    pub return_to: Option<FiberId>,

    // ── Effect list (singly-linked list within the fiber tree) ────
    pub effect_tag: EffectTag,
    pub first_effect: Option<FiberId>,
    pub last_effect: Option<FiberId>,
    pub next_effect: Option<FiberId>,

    // ── Alternate (previous-frame fiber for diffing) ──────────────
    pub alternate: Option<FiberId>,

    // ── Host instance (set during commit) ─────────────────────────
    /// The host platform instance ID, set during commit phase.
    pub host_instance_id: Option<HostInstanceId>,

    // ── Pending VNode children (set during reconciliation) ─────────
    /// Children VNodes pending reconciliation when this fiber is processed
    /// by the work loop. Supports multi-level fiber tree construction.
    pub pending_vnode_children: Option<Vec<VNode>>,

    // ── Component function pointer ─────────────────────────────────
    /// For Function fibers, stores the actual function to call during render.
    pub component_fn: Option<fn(&PropsMap) -> VNode>,
}

impl FiberNode {
    /// Create a new fiber node with the given element type.
    pub fn new(element_type: ElementTypeRef) -> Self {
        Self {
            id: FiberId::ROOT, // placeholder, will be set by arena
            element_type,
            key: None,
            pending_props: PropsMap::new(),
            memoized_props: PropsMap::new(),
            child: None,
            sibling: None,
            return_to: None,
            effect_tag: EffectTag::NoEffect,
            first_effect: None,
            last_effect: None,
            next_effect: None,
            alternate: None,
            host_instance_id: None,
            pending_vnode_children: None,
            component_fn: None,
        }
    }

    /// Create a fiber from a VNode's ElementType.
    pub fn from_element_type(ty: &ElementType) -> Self {
        match ty {
            ElementType::Native(name) => Self::new(ElementTypeRef::Native(name)),
            ElementType::Function(_) => Self::new(ElementTypeRef::Function),
        }
    }
}

impl From<&ElementType> for ElementTypeRef {
    fn from(ty: &ElementType) -> Self {
        match ty {
            ElementType::Native(name) => ElementTypeRef::Native(name),
            ElementType::Function(_) => ElementTypeRef::Function,
        }
    }
}

impl ElementTypeRef {
    /// Returns true if this is a Function variant.
    pub fn is_function(&self) -> bool {
        matches!(self, ElementTypeRef::Function)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fiber_node_new_native() {
        let f = FiberNode::new(ElementTypeRef::Native("container"));
        assert_eq!(f.element_type, ElementTypeRef::Native("container"));
        assert_eq!(f.effect_tag, EffectTag::NoEffect);
        assert!(f.child.is_none());
        assert!(f.sibling.is_none());
        assert!(f.return_to.is_none());
        assert!(f.alternate.is_none());
    }

    #[test]
    fn fiber_node_from_element_type() {
        let f = FiberNode::from_element_type(&ElementType::Native("label"));
        assert_eq!(f.element_type, ElementTypeRef::Native("label"));

        let f2 =
            FiberNode::from_element_type(&ElementType::Function(|_| crate::vnode::VNode::VEmpty));
        assert_eq!(f2.element_type, ElementTypeRef::Function);
    }

    #[test]
    fn effect_tag_ordering() {
        assert!(EffectTag::NoEffect < EffectTag::Placement);
    }
}
