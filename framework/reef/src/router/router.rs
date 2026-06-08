use crate::router::matcher::match_path;
use crate::router::navigator::{HISTORY, HISTORY_INDEX};
use crate::router::route::set_current_path;
use crate::vnode::{PropsMap, VElement, VNode};

/// Context key for the router's current path.
pub(crate) static ROUTER_PATH_CONTEXT: &str = "reef.router.path";

/// Create a Router component that provides navigation state to its children.
///
/// The Router wraps children with a context provider that exposes the current
/// route path and a Navigator. Child `Route` components read from this context
/// to conditionally render.
///
/// # Example
/// ```ignore
/// rsx! {
///     <Router>
///         <Route path="/settings" component={SettingsPage} />
///         <Route path="/status" component={StatusPage} />
///     </Router>
/// }
/// ```
pub fn router(children: Vec<VNode>) -> VNode {
    // Initialize current path from history or default to "/"
    let current_path = HISTORY.with(|h| {
        let history = h.borrow();
        let idx = HISTORY_INDEX.with(|i| *i.borrow());
        if idx >= 0 && (idx as usize) < history.len() {
            history[idx as usize].clone()
        } else {
            "/".to_string()
        }
    });

    set_current_path(&current_path);

    // Create a Provider VNode using the $context_provider pattern
    VNode::VElement(VElement {
        ty: crate::vnode::ElementType::Native("$context_provider"),
        props: {
            let mut p = PropsMap::new();
            p.insert("__ctx_name", ROUTER_PATH_CONTEXT);
            p.insert("__path", current_path.as_str());
            p
        },
        children: vec![VNode::VFragment(children)],
        key: None,
    })
}

/// Create a Route component that conditionally renders based on path matching.
///
/// The `component` closure is called only when the current path matches
/// the given `path` pattern.
///
/// # Example
/// ```ignore
/// route("/settings/:tab", || {
///     rsx! { <label text="Settings Page" /> }
/// })
/// ```
pub fn route(path: &str, component: fn() -> VNode) -> VNode {
    let current_path = crate::router::route::use_current_path();
    let route_match = match_path(&current_path, path);

    if route_match.matched {
        component()
    } else {
        VNode::VEmpty
    }
}

/// Navigate programmatically by setting the current path and history.
pub fn navigate_to(path: &str) {
    use crate::router::navigator::use_navigator;
    let nav = use_navigator();
    nav.push(path);
    set_current_path(path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::route::set_current_path;

    #[test]
    fn router_creates_context_provider() {
        let children = vec![VNode::VText("content".into())];
        let result = router(children);

        match &result {
            VNode::VElement(el) => {
                assert_eq!(
                    el.ty,
                    crate::vnode::ElementType::Native("$context_provider")
                );
                assert_eq!(el.children.len(), 1);
                if let VNode::VFragment(ref frag) = el.children[0] {
                    assert_eq!(frag.len(), 1);
                    assert_eq!(frag[0], VNode::VText("content".into()));
                } else {
                    panic!("expected VFragment wrapper");
                }
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn route_matches_and_renders_component() {
        set_current_path("/settings");

        fn settings_page() -> VNode {
            VNode::VText("Settings".into())
        }

        let result = route("/settings", settings_page);
        assert_eq!(result, VNode::VText("Settings".into()));
    }

    #[test]
    fn route_does_not_match() {
        set_current_path("/home");

        fn settings_page() -> VNode {
            VNode::VText("Settings".into())
        }

        let result = route("/settings", settings_page);
        assert_eq!(result, VNode::VEmpty);
    }
}
