use std::collections::HashMap;

/// Result of matching a URL path against a route pattern.
#[derive(Clone, Debug, PartialEq)]
pub struct RouteMatch {
    /// Whether the path matched.
    pub matched: bool,
    /// Extracted path parameters (e.g. `:id` in `/users/:id`).
    pub params: HashMap<String, String>,
}

/// Match a path against a route pattern.
///
/// Supports:
/// - Exact match: `/settings` matches `/settings`
/// - Parameterized: `/users/:id` matches `/users/42`
/// - Wildcard: `/files/*` matches `/files/a/b/c`
///
/// # Examples
/// ```
/// use reef::router::match_path;
///
/// let m = match_path("/users/42", "/users/:id");
/// assert!(m.matched);
/// assert_eq!(m.params.get("id").unwrap(), "42");
/// ```
pub fn match_path(path: &str, pattern: &str) -> RouteMatch {
    let path_segments: Vec<&str> = path.trim_matches('/').split('/').collect();
    let pattern_segments: Vec<&str> = pattern.trim_matches('/').split('/').collect();

    let mut params = HashMap::new();

    for (i, pattern_seg) in pattern_segments.iter().enumerate() {
        if *pattern_seg == "*" {
            // Wildcard matches everything from this point
            return RouteMatch {
                matched: true,
                params,
            };
        }

        let path_seg = match path_segments.get(i) {
            Some(s) => *s,
            None => {
                return RouteMatch {
                    matched: false,
                    params: HashMap::new(),
                };
            }
        };

        if pattern_seg.starts_with(':') {
            // Named parameter — capture value
            let param_name = &pattern_seg[1..];
            params.insert(param_name.to_string(), path_seg.to_string());
        } else if *pattern_seg != path_seg {
            // Literal segment mismatch
            return RouteMatch {
                matched: false,
                params: HashMap::new(),
            };
        }
    }

    // If pattern has fewer segments than path and no wildcard, partial match
    // Only exact matches are valid (unless last segment was wildcard)
    let matched = path_segments.len() == pattern_segments.len();
    RouteMatch { matched, params }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match() {
        let m = match_path("/settings", "/settings");
        assert!(m.matched);
        assert!(m.params.is_empty());
    }

    #[test]
    fn exact_match_with_trailing_slash() {
        let m = match_path("/settings/", "/settings");
        assert!(m.matched);
    }

    #[test]
    fn no_match() {
        let m = match_path("/settings", "/home");
        assert!(!m.matched);
    }

    #[test]
    fn parameterized_match() {
        let m = match_path("/users/42", "/users/:id");
        assert!(m.matched);
        assert_eq!(m.params.get("id").unwrap(), "42");
    }

    #[test]
    fn multiple_parameters() {
        let m = match_path("/users/42/posts/10", "/users/:userId/posts/:postId");
        assert!(m.matched);
        assert_eq!(m.params.get("userId").unwrap(), "42");
        assert_eq!(m.params.get("postId").unwrap(), "10");
    }

    #[test]
    fn wildcard_match() {
        let m = match_path("/files/a/b/c", "/files/*");
        assert!(m.matched);
        assert!(m.params.is_empty());
    }

    #[test]
    fn wildcard_at_root() {
        let m = match_path("/anything/here", "/*");
        assert!(m.matched);
    }

    #[test]
    fn no_match_different_length() {
        let m = match_path("/a/b/c", "/a/b");
        assert!(!m.matched);
    }

    #[test]
    fn no_match_wrong_segment() {
        let m = match_path("/a/x", "/a/b");
        assert!(!m.matched);
    }

    #[test]
    fn empty_path_matches_root() {
        let m = match_path("", "/");
        assert!(m.matched);
    }
}
