//! 声明式组件工厂函数。
//!
//! 每个组件函数签名：`fn(&PropsMap) -> VNode`
//!
//! 与 `rsx!` 宏配合使用：
//!
//! ```ignore
//! rsx! {
//!     <Container color={Color::rgb(18, 18, 22)} radius={12}>
//!         <Label text={"Hello"} />
//!     </Container>
//! }
//! ```
//!
//! 宏将大写标签（`Container`、`Label`）转换为 `ElementType::Function(Container)`，
//! 由 reconciler 在 work loop 中调用。

#![allow(non_snake_case)]

use crate::props::{PropValue, PropsMap};
use crate::vnode::{ElementType, VElement, VNode};
use std::ptr::fn_addr_eq;

/// A typed component interface for user-defined declarative components.
///
/// The renderer stores function components as `ElementType::Function`; this
/// trait is an optional typed wrapper for code that wants a named component
/// object with explicit props.
pub trait Component: Send + 'static {
    type Props: Send + 'static;

    fn render(&self, props: &Self::Props) -> VNode;
}

/// A type-erased function component pointer.
#[derive(Clone, Copy, Debug)]
pub struct FunctionComponent(pub fn(&PropsMap) -> VNode);

impl FunctionComponent {
    pub fn call(&self, props: &PropsMap) -> VNode {
        (self.0)(props)
    }
}

impl PartialEq for FunctionComponent {
    fn eq(&self, other: &Self) -> bool {
        fn_addr_eq(self.0, other.0)
    }
}

impl From<FunctionComponent> for ElementType {
    fn from(component: FunctionComponent) -> Self {
        ElementType::Function(component.0)
    }
}

/// Create a `FunctionComponent` from a named function.
#[macro_export]
macro_rules! component_fn {
    ($fn:path) => {
        $crate::FunctionComponent($fn)
    };
}

fn children_from_props(props: &PropsMap) -> Vec<VNode> {
    props
        .get("__children")
        .and_then(|v| {
            if let PropValue::VNodeList(c) = v {
                Some(c.clone())
            } else {
                None
            }
        })
        .unwrap_or_default()
}

/// A container element — rendered as a rounded rectangle with optional clipping.
pub fn Container(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("container"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// A text label element.
pub fn Label(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("label"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// A horizontal layout container.
pub fn Row(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("row"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// A vertical layout container.
pub fn Column(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("column"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// A stack layout — children overlap.
pub fn Stack(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("stack"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// An image element.
pub fn Image(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("image"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// An icon element.
pub fn Icon(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("icon"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// A spacer.
pub fn Spacer(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("spacer"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// A divider / separator line.
pub fn Divider(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("divider"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// A button — tappable container.
pub fn Button(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("button"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// An inline code block.
pub fn CodeBlock(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("codeblock"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

/// A badge / pill.
pub fn Badge(props: &PropsMap) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("badge"),
        props: props.clone(),
        children: children_from_props(props),
        key: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::color::Color;

    #[test]
    fn container_creates_native_element() {
        let mut props = PropsMap::new();
        props.insert("color", Color::rgb(18, 18, 22));
        props.insert("radius", 12.0_f64);

        let vnode = Container(&props);
        match vnode {
            VNode::VElement(ref el) => {
                assert_eq!(el.ty, ElementType::Native("container"));
                assert!(el.props.get("color").is_some());
                assert!(el.props.get("radius").is_some());
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn label_creates_native_element() {
        let mut props = PropsMap::new();
        props.insert("text", "Hello");

        let vnode = Label(&props);
        match vnode {
            VNode::VElement(ref el) => {
                assert_eq!(el.ty, ElementType::Native("label"));
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn container_with_children_from_props() {
        let mut props = PropsMap::new();
        props.insert("color", Color::rgb(18, 18, 22));
        props.insert(
            "__children",
            PropValue::VNodeList(vec![VNode::VText("child".into())]),
        );

        let vnode = Container(&props);
        match vnode {
            VNode::VElement(ref el) => {
                assert_eq!(el.children.len(), 1);
                assert_eq!(el.children[0], VNode::VText("child".into()));
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn empty_children_by_default() {
        let props = PropsMap::new();
        let vnode = Label(&props);
        match vnode {
            VNode::VElement(ref el) => assert!(el.children.is_empty()),
            _ => panic!("expected VElement"),
        }
    }

    fn noop_component(_props: &PropsMap) -> VNode {
        VNode::VEmpty
    }

    #[test]
    fn function_component_call() {
        let component = FunctionComponent(noop_component);
        assert_eq!(component.call(&PropsMap::new()), VNode::VEmpty);
    }

    #[test]
    fn component_macro_creates_function_component() {
        let component = component_fn!(noop_component);
        assert_eq!(component, FunctionComponent(noop_component));
    }

    struct Greeting;

    struct GreetingProps {
        text: String,
    }

    impl Component for Greeting {
        type Props = GreetingProps;

        fn render(&self, props: &Self::Props) -> VNode {
            VNode::VText(props.text.clone())
        }
    }

    #[test]
    fn component_trait_render() {
        let component = Greeting;
        let vnode = component.render(&GreetingProps {
            text: "Hello".into(),
        });
        assert_eq!(vnode, VNode::VText("Hello".into()));
    }
}
