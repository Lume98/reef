use crate::fiber::FiberNode;
use reef_hooks::FiberId;

/// A Vec-based arena for fiber node storage.
///
/// Fibers are referenced by `FiberId` (a `u64` index), which avoids
/// self-referential lifetime issues and enables safe tree mutation.
#[derive(Debug, Default)]
pub struct FiberArena {
    pub(crate) nodes: Vec<FiberNode>,
}

impl FiberArena {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Allocate a new fiber node and return its ID.
    pub fn alloc(&mut self, node: FiberNode) -> FiberId {
        let id = FiberId(self.nodes.len() as u64);
        self.nodes.push(node);
        id
    }

    /// Get an immutable reference to a fiber node by its ID.
    pub fn get(&self, id: FiberId) -> &FiberNode {
        &self.nodes[id.0 as usize]
    }

    /// Get a mutable reference to a fiber node by its ID.
    pub fn get_mut(&mut self, id: FiberId) -> &mut FiberNode {
        &mut self.nodes[id.0 as usize]
    }

    /// Number of allocated nodes.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true if no nodes have been allocated.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiber::{EffectTag, ElementTypeRef};

    #[test]
    fn arena_alloc_and_get() {
        let mut arena = FiberArena::new();
        let id = arena.alloc(FiberNode::new(ElementTypeRef::Native("container")));
        let node = arena.get(id);
        assert_eq!(node.element_type, ElementTypeRef::Native("container"));
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn arena_mutate() {
        let mut arena = FiberArena::new();
        let id = arena.alloc(FiberNode::new(ElementTypeRef::Native("label")));
        arena.get_mut(id).effect_tag = EffectTag::Placement;
        assert_eq!(arena.get(id).effect_tag, EffectTag::Placement);
    }
}
