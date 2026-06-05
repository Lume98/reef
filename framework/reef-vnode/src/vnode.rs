use crate::props::PropsMap;
use std::ptr::fn_addr_eq;

/// A virtual DOM node.
///
/// Mirrors React's concept of a ReactNode — the return value of component
/// render functions and the building block of the declarative UI tree.
#[derive(Clone, Debug)]
pub enum VNode {
    /// A platform native element or composite component.
    VElement(VElement),
    /// A plain text leaf node.
    VText(String),
    /// An empty/placeholder node (renders nothing).
    VEmpty,
    /// Multiple children without an extra wrapper element.
    VFragment(Vec<VNode>),
}

impl PartialEq for VNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VNode::VElement(a), VNode::VElement(b)) => a == b,
            (VNode::VText(a), VNode::VText(b)) => a == b,
            (VNode::VEmpty, VNode::VEmpty) => true,
            (VNode::VFragment(a), VNode::VFragment(b)) => a == b,
            _ => false,
        }
    }
}

/// A virtual element — analogous to `ReactElement`.
///
/// Carries the element type identifier, its immutable properties, child nodes,
/// and an optional reconciliation key.
#[derive(Clone, Debug)]
pub struct VElement {
    /// Identifies whether this is a native platform element or a function component.
    pub ty: ElementType,
    /// Immutable properties passed to this element.
    pub props: PropsMap,
    /// Child nodes.
    pub children: Vec<VNode>,
    /// Optional key for efficient reconciliation.
    pub key: Option<String>,
}

impl PartialEq for VElement {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
            && self.props == other.props
            && self.children == other.children
            && self.key == other.key
    }
}

/// The type identifier of a virtual element.
#[derive(Clone, Debug)]
pub enum ElementType {
    /// A native platform element (e.g. "container", "label", "row", "column").
    Native(&'static str),
    /// A user-defined function component.
    Function(fn(&PropsMap) -> VNode),
}

impl PartialEq for ElementType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ElementType::Native(a), ElementType::Native(b)) => a == b,
            (ElementType::Function(a), ElementType::Function(b)) => fn_addr_eq(*a, *b),
            _ => false,
        }
    }
}

// Helper constructors for ergonomic VNode creation without macros.

pub fn element(ty: &'static str, props: PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native(ty),
        props,
        children,
        key: None,
    })
}

pub fn element_with_key(
    ty: &'static str,
    props: PropsMap,
    children: Vec<VNode>,
    key: &str,
) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native(ty),
        props,
        children,
        key: Some(key.to_string()),
    })
}

pub fn fragment(children: Vec<VNode>) -> VNode {
    VNode::VFragment(children)
}

pub fn text(content: &str) -> VNode {
    VNode::VText(content.to_string())
}

pub fn empty() -> VNode {
    VNode::VEmpty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_native_element() {
        let el = element("container", PropsMap::new(), vec![]);
        match el {
            VNode::VElement(ref e) => {
                assert_eq!(e.ty, ElementType::Native("container"));
                assert!(e.children.is_empty());
                assert!(e.key.is_none());
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn create_element_with_key() {
        let el = element_with_key("label", PropsMap::new(), vec![], "title");
        match el {
            VNode::VElement(ref e) => {
                assert_eq!(e.key.as_deref(), Some("title"));
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn create_fragment() {
        let frag = fragment(vec![
            element("label", PropsMap::new(), vec![]),
            text("hello"),
        ]);
        match frag {
            VNode::VFragment(ref children) => assert_eq!(children.len(), 2),
            _ => panic!("expected VFragment"),
        }
    }

    #[test]
    fn text_and_empty_vnodes() {
        assert_eq!(text("hi"), VNode::VText("hi".to_string()));
        assert_eq!(empty(), VNode::VEmpty);
    }

    #[test]
    fn function_component_element_type() {
        fn my_component(_props: &PropsMap) -> VNode {
            VNode::VEmpty
        }
        let func = ElementType::Function(my_component);
        assert_ne!(func, ElementType::Native("container"));
    }

    #[test]
    fn vnode_partial_eq() {
        let a = text("hello");
        let b = text("hello");
        let c = text("world");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(empty(), empty());
        assert_ne!(text("x"), empty());
    }
}
