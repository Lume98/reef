use std::ptr::fn_addr_eq;
use reef_vnode::{PropsMap, VNode};

/// A function component — the fundamental building block of declarative UI.
///
/// Analogous to React's function component pattern:
/// ```ignore
/// fn MyComponent(props: &MyProps) -> VNode {
///     rsx! { <label text={props.text} /> }
/// }
/// ```
///
/// Components are stateless functions that receive immutable props and return
/// a `VNode` tree. State is managed externally via hooks (Phase 2).
pub trait Component: Send + 'static {
    /// The props type consumed by this component.
    type Props: Send + 'static;

    /// Render the component into a VNode tree given the provided props.
    fn render(&self, props: &Self::Props) -> VNode;
}

/// A type-erased function component pointer, stored in `VElement` when the
/// element references a user-defined component rather than a native element.
#[derive(Clone, Copy, Debug)]
pub struct FunctionComponent(pub fn(&PropsMap) -> VNode);

impl PartialEq for FunctionComponent {
    fn eq(&self, other: &Self) -> bool {
        fn_addr_eq(self.0, other.0)
    }
}

impl FunctionComponent {
    /// Call this component with the given props and return the produced VNode tree.
    pub fn call(&self, props: &PropsMap) -> VNode {
        (self.0)(props)
    }
}

impl From<FunctionComponent> for reef_vnode::ElementType {
    fn from(fc: FunctionComponent) -> Self {
        reef_vnode::ElementType::Function(fc.0)
    }
}

/// Convenience macro for creating a `FunctionComponent` from a named function.
///
/// Usage:
/// ```ignore
/// fn MyButton(props: &PropsMap) -> VNode { ... }
/// component_fn!(MyButton)
/// ```
#[macro_export]
macro_rules! component_fn {
    ($fn:path) => {
        $crate::FunctionComponent($fn)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_vnode::VNode;

    fn noop_component(_props: &PropsMap) -> VNode {
        VNode::VEmpty
    }

    #[test]
    fn function_component_call() {
        let fc = FunctionComponent(noop_component);
        let result = fc.call(&PropsMap::new());
        assert_eq!(result, VNode::VEmpty);
    }

    #[test]
    fn function_component_clone_and_debug() {
        let fc = FunctionComponent(noop_component);
        let cloned = fc.clone();
        assert_eq!(fc, cloned);
    }

    #[test]
    fn component_macro() {
        let fc = component_fn!(noop_component);
        assert_eq!(fc, FunctionComponent(noop_component));
    }

    struct MyProps {
        text: String,
    }

    struct Greeting;

    impl Component for Greeting {
        type Props = MyProps;

        fn render(&self, props: &Self::Props) -> VNode {
            VNode::VText(props.text.clone())
        }
    }

    #[test]
    fn component_trait_render() {
        let comp = Greeting;
        let props = MyProps {
            text: "Hello".into(),
        };
        let result = comp.render(&props);
        assert_eq!(result, VNode::VText("Hello".into()));
    }
}
