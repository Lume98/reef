use crate::router::matcher::match_path;
use std::cell::RefCell;
use std::collections::HashMap;

/// Information about the current route.
#[derive(Clone, Debug, PartialEq)]
pub struct RouteInfo {
    /// The full path of the current route.
    pub path: String,
    /// Extracted path parameters.
    pub params: HashMap<String, String>,
    /// Query string parameters.
    pub query: HashMap<String, String>,
}

thread_local! {
    /// Current route path (set by Router).
    pub(crate) static CURRENT_PATH: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Get the current route path from the nearest Router context.
pub fn use_current_path() -> String {
    CURRENT_PATH.with(|p| p.borrow().clone())
}

/// Get the full route info including params extracted from the active route pattern.
pub fn use_route() -> RouteInfo {
    let path = use_current_path();
    RouteInfo {
        path,
        params: HashMap::new(),
        query: HashMap::new(),
    }
}

/// Set the current route path (called by Router on navigation).
pub(crate) fn set_current_path(path: &str) {
    CURRENT_PATH.with(|p| {
        *p.borrow_mut() = path.to_string();
    });
}

/// Determine if a route pattern matches the current path, and if so return
/// the matched params.
pub fn match_route(pattern: &str) -> Option<HashMap<String, String>> {
    let path = use_current_path();
    let route_match = match_path(&path, pattern);
    if route_match.matched {
        Some(route_match.params)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_path_default_is_empty() {
        let path = use_current_path();
        assert_eq!(path, "");
    }

    #[test]
    fn set_and_get_current_path() {
        set_current_path("/settings");
        assert_eq!(use_current_path(), "/settings");
    }

    #[test]
    fn route_info_from_current_path() {
        set_current_path("/users/42");
        let info = use_route();
        assert_eq!(info.path, "/users/42");
    }

    #[test]
    fn match_route_returns_params() {
        set_current_path("/users/42");
        let params = match_route("/users/:id");
        assert!(params.is_some());
        assert_eq!(params.unwrap().get("id").unwrap(), "42");
    }

    #[test]
    fn match_route_returns_none_on_mismatch() {
        set_current_path("/settings");
        let params = match_route("/users/:id");
        assert!(params.is_none());
    }
}
