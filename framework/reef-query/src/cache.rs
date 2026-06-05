use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Time-to-live for cached query results.
const DEFAULT_TTL: Duration = Duration::from_secs(60);

struct CacheEntry {
    data: Box<dyn Any>,
    expires_at: Instant,
    loading: bool,
}

thread_local! {
    static QUERY_CACHE: RefCell<HashMap<String, CacheEntry>> = RefCell::new(HashMap::new());
}

/// Store a value in the query cache under the given key.
pub fn cache_query<T: 'static>(key: &str, value: T, ttl: Option<Duration>) {
    let ttl = ttl.unwrap_or(DEFAULT_TTL);
    QUERY_CACHE.with(|cache| {
        cache.borrow_mut().insert(
            key.to_string(),
            CacheEntry {
                data: Box::new(value),
                expires_at: Instant::now() + ttl,
                loading: false,
            },
        );
    });
}

/// Get a cached value by key. Returns `None` if not found or expired.
pub fn get_cached<T: 'static>(key: &str) -> Option<T> where T: Clone {
    QUERY_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let expired = cache.get(key).map_or(false, |e| e.expires_at <= Instant::now() && !e.loading);
        if expired {
            cache.remove(key);
            return None;
        }
        cache.get(key)
            .and_then(|entry| entry.data.downcast_ref::<T>())
            .cloned()
    })
}

/// Invalidate (remove) a cached query result.
pub fn invalidate_query(key: &str) {
    QUERY_CACHE.with(|cache| {
        cache.borrow_mut().remove(key);
    });
}

/// Invalidate all cached queries.
pub fn invalidate_all() {
    QUERY_CACHE.with(|cache| {
        cache.borrow_mut().clear();
    });
}

/// Prefetch a query result and store it in the cache.
pub fn prefetch_query<T: 'static>(key: &str, value: T) {
    cache_query(key, value, None);
}

/// Get the number of cached entries.
pub fn cache_size() -> usize {
    QUERY_CACHE.with(|cache| cache.borrow().len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_and_retrieve() {
        cache_query("user:1", "Alice".to_string(), None);
        let result: Option<String> = get_cached("user:1");
        assert_eq!(result, Some("Alice".to_string()));
    }

    #[test]
    fn cache_miss() {
        let result: Option<String> = get_cached("nonexistent");
        assert_eq!(result, None);
    }

    #[test]
    fn invalidate_single() {
        cache_query("key", 42_i32, None);
        invalidate_query("key");
        let result: Option<i32> = get_cached("key");
        assert_eq!(result, None);
    }

    #[test]
    fn invalidate_all_entries() {
        cache_query("a", 1_i32, None);
        cache_query("b", 2_i32, None);
        invalidate_all();
        assert_eq!(cache_size(), 0);
    }

    #[test]
    fn prefetch_then_retrieve() {
        prefetch_query("settings", "dark_mode".to_string());
        let result: Option<String> = get_cached("settings");
        assert_eq!(result, Some("dark_mode".to_string()));
    }

    #[test]
    fn cache_with_different_types() {
        cache_query("count", 42_i32, None);
        cache_query("name", "Reef".to_string(), None);

        assert_eq!(get_cached::<i32>("count"), Some(42));
        assert_eq!(get_cached::<String>("name"), Some("Reef".to_string()));
    }
}
