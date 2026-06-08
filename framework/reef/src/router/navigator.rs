use std::cell::RefCell;
use std::collections::VecDeque;

/// Navigation controller returned by `use_navigator`.
///
/// Provides programmatic navigation between routes.
#[derive(Clone)]
pub struct Navigator {
    /// Push a new path onto the history stack.
    pub push_fn: fn(&str),
    /// Replace the current path without adding history.
    pub replace_fn: fn(&str),
    /// Go back in history.
    pub back_fn: fn(),
    /// Go forward in history.
    pub forward_fn: fn(),
}

impl std::fmt::Debug for Navigator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Navigator").finish()
    }
}

impl Navigator {
    /// Navigate to a path, adding it to the history stack.
    pub fn push(&self, path: &str) {
        (self.push_fn)(path);
    }

    /// Replace the current location without adding history.
    pub fn replace(&self, path: &str) {
        (self.replace_fn)(path);
    }

    /// Navigate back in history.
    pub fn back(&self) {
        (self.back_fn)();
    }

    /// Navigate forward in history.
    pub fn forward(&self) {
        (self.forward_fn)();
    }

    /// Create a no-op navigator (for placeholder/testing).
    pub fn noop() -> Self {
        Self {
            push_fn: NOOP,
            replace_fn: NOOP,
            back_fn: NOOP_VOID,
            forward_fn: NOOP_VOID,
        }
    }
}

thread_local! {
    pub(crate) static HISTORY: RefCell<VecDeque<String>> = const { RefCell::new(VecDeque::new()) };
    pub(crate) static HISTORY_INDEX: RefCell<isize> = const { RefCell::new(-1) };
}

static NOOP: fn(&str) = |_| {};
static NOOP_VOID: fn() = || {};

/// Internal: push a path onto the history stack.
fn push_path(path: &str) {
    let idx = HISTORY_INDEX.with(|i| *i.borrow());
    HISTORY.with(|h| {
        let mut history = h.borrow_mut();
        let keep = (idx + 1) as usize;
        while history.len() > keep {
            history.pop_back();
        }
        history.push_back(path.to_string());
        HISTORY_INDEX.with(|i| *i.borrow_mut() = (history.len() - 1) as isize);
    });
}

fn replace_path(path: &str) {
    let idx = HISTORY_INDEX.with(|i| *i.borrow());
    HISTORY.with(|h| {
        let mut history = h.borrow_mut();
        if idx >= 0 && (idx as usize) < history.len() {
            history[idx as usize] = path.to_string();
        } else {
            history.push_back(path.to_string());
            HISTORY_INDEX.with(|i| *i.borrow_mut() = (history.len() - 1) as isize);
        }
    });
}

fn go_back() {
    HISTORY_INDEX.with(|idx| {
        let mut idx = idx.borrow_mut();
        if *idx > 0 {
            *idx -= 1;
        }
    });
}

fn go_forward() {
    HISTORY_INDEX.with(|idx| {
        let mut idx = idx.borrow_mut();
        HISTORY.with(|h| {
            let history = h.borrow();
            if (*idx as usize) < history.len().saturating_sub(1) {
                *idx += 1;
            }
        });
    });
}

// Static function pointers for Navigator
static PUSH_FN: fn(&str) = push_path;
static REPLACE_FN: fn(&str) = replace_path;
static BACK_FN: fn() = go_back;
static FORWARD_FN: fn() = go_forward;

/// Get the navigator hook — returns a `Navigator` connected to the router's
/// history state.
///
/// Must be called within a `Router` provider context.
pub fn use_navigator() -> Navigator {
    Navigator {
        push_fn: PUSH_FN,
        replace_fn: REPLACE_FN,
        back_fn: BACK_FN,
        forward_fn: FORWARD_FN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigator_push_and_history() {
        let nav = use_navigator();
        nav.push("/page1");
        nav.push("/page2");

        HISTORY.with(|h| {
            let history = h.borrow();
            assert_eq!(history.len(), 2);
            assert_eq!(history[0], "/page1");
            assert_eq!(history[1], "/page2");
        });
    }

    #[test]
    fn navigator_back_and_forward() {
        let nav = use_navigator();
        nav.push("/a");
        nav.push("/b");
        nav.back();

        let idx = HISTORY_INDEX.with(|i| *i.borrow());
        assert_eq!(idx, 0);

        nav.forward();
        let idx = HISTORY_INDEX.with(|i| *i.borrow());
        assert_eq!(idx, 1);
    }

    #[test]
    fn navigator_replace() {
        let nav = use_navigator();
        nav.push("/old");
        nav.replace("/new");

        HISTORY.with(|h| {
            let history = h.borrow();
            assert_eq!(history.len(), 1);
            assert_eq!(history[0], "/new");
        });
    }

    #[test]
    fn navigator_noop() {
        let nav = Navigator::noop();
        nav.push("/any");
        nav.back();
        nav.forward();
        // Should not panic
    }
}
