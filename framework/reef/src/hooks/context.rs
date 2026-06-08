use crate::hooks::{FiberId, HOOK_REGISTRY};
use crate::vnode::{PropsMap, VElement, VNode};
use std::any::Any;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

/// A typed context identifier, analogous to React's `createContext`.
///
/// Create one with `create_context::<T>("name")` and access the value
/// in child components with `use_context`.
///
/// The value is provided higher in the tree via `context_provider`.
pub struct Context<T> {
    pub(crate) name: &'static str,
    pub(crate) id: u64,
    _phantom: PhantomData<T>,
}

impl<T> Clone for Context<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for Context<T> {}

impl<T> std::fmt::Debug for Context<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("name", &self.name)
            .field("id", &self.id)
            .finish()
    }
}

static NEXT_CONTEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Create a typed context, analogous to React's `createContext`.
///
/// ```ignore
/// let theme_ctx = create_context::<Theme>("theme");
/// ```
pub fn create_context<T: 'static>(name: &'static str) -> Context<T> {
    Context {
        name,
        id: NEXT_CONTEXT_ID.fetch_add(1, Ordering::Relaxed),
        _phantom: PhantomData,
    }
}

// ── Provider side-channel ─────────────────────────────────────────

struct ProviderEntry {
    name: &'static str,
    value: Box<dyn Any>,
}

thread_local! {
    static PROVIDER_STACK: RefCell<Vec<ProviderEntry>> = const { RefCell::new(Vec::new()) };
}

/// Push a provider value onto the stack (called by reconciler).
pub(crate) fn push_provider_value(name: &'static str, value: Box<dyn Any>) {
    PROVIDER_STACK.with(|stack| {
        stack.borrow_mut().push(ProviderEntry { name, value });
    });
}

/// Pop the most recent provider from the stack (called by reconciler).
pub(crate) fn pop_provider() {
    PROVIDER_STACK.with(|stack| {
        stack.borrow_mut().pop();
    });
}

/// Wrap children with a context value, analogous to React's `<Context.Provider>`.
///
/// The value will be accessible to descendant components via `use_context`.
pub fn context_provider<T: 'static>(ctx: Context<T>, children: Vec<VNode>) -> VNode {
    VNode::VElement(VElement {
        ty: crate::vnode::ElementType::Native("$context_provider"),
        props: {
            let mut p = PropsMap::new();
            p.insert("__ctx_name", ctx.name);
            p.insert("__ctx_id", ctx.id as i32);
            p
        },
        children: vec![VNode::VFragment(children)],
        key: None,
    })
}

/// Access a context value provided by an ancestor, analogous to React's `useContext`.
///
/// Returns the value if a matching provider exists in the ancestor tree.
/// The reconciler pushes provider values onto the stack during tree traversal.
///
/// # Panics
/// Panics if called outside of a component's render function.
pub fn use_context<T: Clone + 'static>(ctx: &Context<T>) -> Option<T> {
    let _fiber = HOOK_REGISTRY.with(|reg| {
        reg.borrow()
            .current_fiber
            .expect("use_context must be called during render")
    });

    PROVIDER_STACK.with(|stack| {
        let stack = stack.borrow();
        for entry in stack.iter().rev() {
            if entry.name == ctx.name {
                return entry.value.downcast_ref::<T>().cloned();
            }
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct Theme {
        pub bg_color: (u8, u8, u8),
    }

    #[test]
    fn context_create_debug() {
        let ctx = create_context::<Theme>("theme");
        assert_eq!(ctx.name, "theme");
        assert!(ctx.id > 0);
    }

    #[test]
    fn context_provider_and_use() {
        let ctx = create_context::<i32>("answer");
        let fiber = FiberId::allocate();

        HOOK_REGISTRY.with(|reg| reg.borrow_mut().begin_fiber(fiber));
        push_provider_value(ctx.name, Box::new(42_i32));

        let value = use_context(&ctx);
        assert_eq!(value, Some(42_i32));

        pop_provider();
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().end_fiber());
    }

    #[test]
    fn context_multiple_nested_providers() {
        let theme_ctx = create_context::<Theme>("theme");
        let answer_ctx = create_context::<i32>("answer");
        let fiber = FiberId::allocate();

        HOOK_REGISTRY.with(|reg| reg.borrow_mut().begin_fiber(fiber));

        push_provider_value(
            theme_ctx.name,
            Box::new(Theme {
                bg_color: (0, 0, 0),
            }),
        );
        push_provider_value(answer_ctx.name, Box::new(42_i32));

        assert_eq!(use_context(&answer_ctx), Some(42_i32));
        assert_eq!(
            use_context(&theme_ctx),
            Some(Theme {
                bg_color: (0, 0, 0)
            })
        );

        pop_provider();
        pop_provider();

        HOOK_REGISTRY.with(|reg| reg.borrow_mut().end_fiber());
    }

    #[test]
    fn context_provider_creates_vnode() {
        let ctx = create_context::<&'static str>("greeting");
        let vnode = context_provider(ctx, vec![VNode::VText("child".into())]);

        match vnode {
            VNode::VElement(ref el) => {
                assert_eq!(
                    el.ty,
                    crate::vnode::ElementType::Native("$context_provider")
                );
                assert_eq!(el.children.len(), 1);
                if let VNode::VFragment(ref frag) = el.children[0] {
                    assert_eq!(frag.len(), 1);
                    assert_eq!(frag[0], VNode::VText("child".into()));
                } else {
                    panic!("expected VFragment wrapper");
                }
            }
            _ => panic!("expected VElement"),
        }
    }

    #[test]
    fn context_no_provider_returns_none() {
        let ctx = create_context::<String>("missing");
        let fiber = FiberId::allocate();

        HOOK_REGISTRY.with(|reg| reg.borrow_mut().begin_fiber(fiber));
        let value = use_context(&ctx);
        assert_eq!(value, None::<String>);
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().end_fiber());
    }
}
