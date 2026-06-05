use reef_core::color::Color;
use reef_core::geometry::Rect;
use reef_vnode::PropsMap;

/// A node in the platform scene graph, produced by the reconciler's
/// HostConfig and consumed by layout + paint passes.
///
/// Analogous to a DOM node in React DOM — represents an instantiated
/// platform element with resolved layout geometry.
#[derive(Clone, Debug)]
pub struct SceneNode {
    /// Stable identifier matching the HostInstanceId from reconciliation.
    pub id: u64,
    /// The native element type name.
    pub ty: String,
    /// Resolved properties (after HostConfig instantiation).
    pub props: PropsMap,
    /// Computed layout frame (set by layout pass).
    pub frame: Rect,
    /// Child nodes.
    pub children: Vec<SceneNode>,
    /// Whether this node and its subtree should be clipped.
    pub clip_children: bool,
    /// Flexible growth factor (for flexbox-like layout).
    pub flex: f64,
}

impl SceneNode {
    pub fn new(id: u64, ty: &str, props: PropsMap) -> Self {
        let flex = props.get("flex").and_then(|v| {
            if let reef_vnode::PropValue::F64(f) = v { Some(*f) } else { None }
        }).unwrap_or(0.0);
        Self {
            id,
            ty: ty.to_string(),
            props,
            frame: Rect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            children: Vec::new(),
            clip_children: matches!(ty, "container" | "row" | "column" | "stack"),
            flex,
        }
    }

    /// Get a string prop value.
    pub fn prop_str(&self, key: &str) -> Option<String> {
        self.props.get(key).and_then(|v| {
            if let reef_vnode::PropValue::String(s) = v {
                Some(s.clone())
            } else {
                None
            }
        })
    }

    /// Get an f64 prop value.
    pub fn prop_f64(&self, key: &str) -> Option<f64> {
        self.props.get(key).and_then(|v| {
            if let reef_vnode::PropValue::F64(f) = v { Some(*f) } else { None }
        })
    }

    /// Get a color prop value.
    pub fn prop_color(&self, key: &str) -> Option<Color> {
        self.props.get(key).and_then(|v| {
            if let reef_vnode::PropValue::Color(c) = v { Some(*c) } else { None }
        })
    }

    /// Get an i32 prop value.
    pub fn prop_i32(&self, key: &str) -> Option<i32> {
        self.props.get(key).and_then(|v| {
            if let reef_vnode::PropValue::I32(i) = v { Some(*i) } else { None }
        })
    }

    /// Get a bool prop value.
    pub fn prop_bool(&self, key: &str) -> Option<bool> {
        self.props.get(key).and_then(|v| {
            if let reef_vnode::PropValue::Bool(b) = v { Some(*b) } else { None }
        })
    }

    /// Recursively count nodes in the subtree.
    pub fn node_count(&self) -> usize {
        1 + self.children.iter().map(|c| c.node_count()).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_node_new() {
        let node = SceneNode::new(1, "container", PropsMap::new());
        assert_eq!(node.id, 1);
        assert_eq!(node.ty, "container");
        assert!(node.clip_children);
        assert_eq!(node.frame, Rect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 });
    }

    #[test]
    fn scene_node_non_clipping_types() {
        let label = SceneNode::new(2, "label", PropsMap::new());
        assert!(!label.clip_children);
    }

    #[test]
    fn scene_node_prop_accessors() {
        let mut props = PropsMap::new();
        props.insert("color", Color::rgb(255, 0, 0));
        props.insert("radius", 12.0_f64);
        props.insert("label", "hello");
        props.insert("count", 5_i32);
        props.insert("visible", true);

        let node = SceneNode::new(3, "label", props);
        assert_eq!(node.prop_color("color"), Some(Color::rgb(255, 0, 0)));
        assert_eq!(node.prop_f64("radius"), Some(12.0));
        assert_eq!(node.prop_str("label"), Some("hello".into()));
        assert_eq!(node.prop_i32("count"), Some(5));
        assert_eq!(node.prop_bool("visible"), Some(true));
    }

    #[test]
    fn scene_node_count() {
        let mut parent = SceneNode::new(1, "container", PropsMap::new());
        parent.children.push(SceneNode::new(2, "label", PropsMap::new()));
        parent.children.push(SceneNode::new(3, "label", PropsMap::new()));
        assert_eq!(parent.node_count(), 3);
    }
}
