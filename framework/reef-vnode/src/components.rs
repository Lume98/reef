//! 声明式组件工厂函数。
//!
//! 这些函数与 `rsx!` 宏的大写标签配合使用：
//!
//! ```ignore
//! rsx! {
//!     <Container color={Color::rgb(18, 18, 22)} radius={12}>
//!         <Label text={"Hello"} />
//!     </Container>
//! }
//! ```
//!
//! 宏将大写标签（`Container`、`Label`）转换为对应的函数调用。
//!
//! # 命名说明
//! 函数使用 PascalCase 以匹配 JSX 组件命名规范，允许 `#[allow(non_snake_case)]`。

#![allow(non_snake_case)]

use crate::props::PropsMap;
use crate::vnode::{ElementType, VElement, VNode};

/// A container element — rendered as a rounded rectangle with optional clipping.
///
/// Props: `color`, `radius`, `border_color`, `border_width`, `padding`, `min_width`, `min_height`, `alpha`
pub fn Container(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("container"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// A text label element — renders text with optional formatting.
///
/// Props: `text`, `color`, `font_size`, `weight`, `alignment`, `alpha`
pub fn Label(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("label"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// A horizontal layout container — arranges children in a row.
///
/// Props: `gap`
pub fn Row(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("row"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// A vertical layout container — arranges children in a column.
///
/// Props: `gap`
pub fn Column(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("column"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// A stack layout — children overlap, each taking full available space.
///
/// Props: (none specific to stack)
pub fn Stack(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("stack"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// An image element — renders a bitmap/sprite.
///
/// Props: `key` (string identifier), `source_rect`, `opacity`
pub fn Image(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("image"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// An icon element — renders a rectangular image keyed by name.
///
/// Props: `icon` (string name), `size`, `color`
pub fn Icon(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("icon"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// A spacer — takes up available space in a row/column.
///
/// Props: `min_width`, `min_height`
pub fn Spacer(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("spacer"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// A divider / separator line.
///
/// Props: `color`, `thickness`, `margin`
pub fn Divider(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("divider"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// A button — renders a tappable container.
///
/// Props: all container props + `on_click`
pub fn Button(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("button"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// An inline code block — renders as a small pill with monospace text.
///
/// Props: `text`, `color`, `background`
pub fn CodeBlock(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("codeblock"),
        props: props.clone(),
        children,
        key: None,
    })
}

/// A badge / pill — renders a small rounded label.
///
/// Props: `text`, `color`, `background`
pub fn Badge(props: &PropsMap, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: ElementType::Native("badge"),
        props: props.clone(),
        children,
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

        let vnode = Container(&props, vec![]);
        match vnode {
            VNode::VElement(ref el) => {
                assert_eq!(el.ty, ElementType::Native("container"));
                assert!(el.props.get("color").is_some());
                assert!(el.props.get("radius").is_some());
                assert!(el.children.is_empty());
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn label_creates_native_element() {
        let mut props = PropsMap::new();
        props.insert("text", "Hello");

        let vnode = Label(&props, vec![]);
        match vnode {
            VNode::VElement(ref el) => {
                assert_eq!(el.ty, ElementType::Native("label"));
                assert_eq!(
                    el.props.get("text"),
                    Some(&crate::PropValue::String("Hello".into()))
                );
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn row_with_children() {
        let props = PropsMap::new();
        let child = Label(&{
            let mut p = PropsMap::new();
            p.insert("text", "item");
            p
        }, vec![]);

        let vnode = Row(&props, vec![child]);
        match vnode {
            VNode::VElement(ref el) => {
                assert_eq!(el.ty, ElementType::Native("row"));
                assert_eq!(el.children.len(), 1);
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn column_with_gap() {
        let mut props = PropsMap::new();
        props.insert("gap", 8.0_f64);

        let vnode = Column(&props, vec![]);
        match vnode {
            VNode::VElement(ref el) => {
                assert_eq!(el.ty, ElementType::Native("column"));
                assert_eq!(el.props.get("gap"), Some(&crate::PropValue::F64(8.0)));
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn stack_overlapping_children() {
        let props = PropsMap::new();
        let vnode = Stack(&props, vec![
            Label(&PropsMap::new(), vec![]),
            Label(&PropsMap::new(), vec![]),
        ]);
        match vnode {
            VNode::VElement(ref el) => {
                assert_eq!(el.ty, ElementType::Native("stack"));
                assert_eq!(el.children.len(), 2);
            }
            _ => panic!("expected VElement"),
        }
    }
}
