use crate::dom::scene::SceneNode;
use crate::reconciler::host_config::{HostConfig, HostInstanceId};
use crate::vnode::{ElementType, PropsMap};
use std::collections::HashMap;

/// The Reef DOM host config — converts reconciler effects into a
/// `SceneNode` tree that can be laid out and painted.
///
/// Analogous to React DOM's `HostConfig` implementation.
pub struct ReefDomConfig {
    /// All instantiated scene nodes, keyed by HostInstanceId.
    nodes: HashMap<u64, SceneNode>,
    /// Parent-child relationships: child_id → parent_id.
    parents: HashMap<u64, u64>,
    /// Roots (nodes without parents) — typically just one.
    roots: Vec<u64>,
    /// Next ID to assign.
    next_id: u64,
}

impl ReefDomConfig {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            parents: HashMap::new(),
            roots: Vec::new(),
            next_id: 1,
        }
    }

    /// Get the root scene node (for layout/paint).
    pub fn root_node(&self) -> Option<&SceneNode> {
        self.roots.first().and_then(|id| self.nodes.get(id))
    }

    /// Get a mutable reference to a node.
    pub fn node_mut(&mut self, id: HostInstanceId) -> Option<&mut SceneNode> {
        self.nodes.get_mut(&id.0)
    }

    /// Rebuild the full scene tree as a single rooted `SceneNode`.
    /// Returns the root node with all children attached.
    pub fn build_scene_tree(&self) -> Option<SceneNode> {
        let root_id = *self.roots.first()?;
        self.build_subtree(root_id)
    }

    fn build_subtree(&self, id: u64) -> Option<SceneNode> {
        let mut node = self.nodes.get(&id)?.clone();
        // Collect direct children
        let mut child_ids: Vec<u64> = self
            .parents
            .iter()
            .filter(|(_, &p)| p == id)
            .map(|(c, _)| *c)
            .collect();
        child_ids.sort();
        node.children = child_ids
            .iter()
            .filter_map(|cid| self.build_subtree(*cid))
            .collect();
        Some(node)
    }
}

impl HostConfig for ReefDomConfig {
    fn create_instance(&mut self, ty: &ElementType, props: &PropsMap) -> HostInstanceId {
        let id = HostInstanceId(self.next_id);
        self.next_id += 1;

        let type_name = match ty {
            ElementType::Native(name) => name.to_string(),
            ElementType::Function(_) => "$component".to_string(),
        };

        let node = SceneNode::new(id.0, &type_name, props.clone());
        self.nodes.insert(id.0, node);

        // If no parent yet, consider it a root
        if !self.parents.contains_key(&id.0) {
            self.roots.push(id.0);
        }

        id
    }

    fn append_child(&mut self, parent: HostInstanceId, child: HostInstanceId) {
        // Record parent-child relationship
        self.parents.insert(child.0, parent.0);
        // Remove child from roots if it was there
        self.roots.retain(|r| *r != child.0);

        // Build the tree structure by setting children on the parent node
        if let Some(_parent_node) = self.nodes.get_mut(&parent.0) {
            // Ensure the child node exists
            if self.nodes.contains_key(&child.0) {
                // We'll rebuild the full tree later from parents map
            }
        }
    }

    fn update_instance(&mut self, instance: HostInstanceId, props: &PropsMap) {
        if let Some(node) = self.nodes.get_mut(&instance.0) {
            node.props = props.clone();
        }
    }

    fn remove_instance(&mut self, instance: HostInstanceId) {
        self.nodes.remove(&instance.0);
        if let Some(_parent_id) = self.parents.remove(&instance.0) {
            // Update parent's children
        }
        self.roots.retain(|r| *r != instance.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vnode::{ElementType, PropsMap};

    #[test]
    fn create_container_instance() {
        let mut config = ReefDomConfig::new();
        let id = config.create_instance(&ElementType::Native("container"), &PropsMap::new());
        let node = config.nodes.get(&id.0).unwrap();
        assert_eq!(node.ty, "container");
    }

    #[test]
    fn create_and_append() {
        let mut config = ReefDomConfig::new();
        let parent = config.create_instance(&ElementType::Native("container"), &PropsMap::new());
        let child = config.create_instance(&ElementType::Native("label"), &PropsMap::new());

        config.append_child(parent, child);

        assert_eq!(config.parents.get(&child.0), Some(&parent.0));
        assert!(config.roots.contains(&parent.0));
        assert!(!config.roots.contains(&child.0));
    }

    #[test]
    fn update_instance_props() {
        let mut config = ReefDomConfig::new();
        let id = config.create_instance(&ElementType::Native("label"), &PropsMap::new());

        let mut new_props = PropsMap::new();
        new_props.insert("text", "updated");
        config.update_instance(id, &new_props);

        let node = config.nodes.get(&id.0).unwrap();
        assert_eq!(node.prop_str("text"), Some("updated".into()));
    }

    #[test]
    fn remove_instance() {
        let mut config = ReefDomConfig::new();
        let id = config.create_instance(&ElementType::Native("container"), &PropsMap::new());
        config.remove_instance(id);
        assert!(config.nodes.is_empty());
        assert!(config.roots.is_empty());
    }

    #[test]
    fn build_scene_tree() {
        let mut config = ReefDomConfig::new();
        let parent = config.create_instance(&ElementType::Native("container"), &PropsMap::new());
        let child = config.create_instance(&ElementType::Native("label"), &PropsMap::new());
        config.append_child(parent, child);

        let tree = config.build_scene_tree();
        assert!(tree.is_some());
        let root = tree.unwrap();
        assert_eq!(root.ty, "container");
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].ty, "label");
    }
}
