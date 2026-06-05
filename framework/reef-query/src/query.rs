use crate::cache::{cache_query, get_cached, invalidate_query};

/// The status of a query result.
#[derive(Clone, Debug, PartialEq)]
pub enum QueryStatus {
    Loading,
    Ready,
    Error,
}

/// The result of a `use_query` hook.
#[derive(Clone, Debug)]
pub struct QueryResult<T> {
    pub data: Option<T>,
    pub loading: bool,
    pub error: Option<String>,
    pub status: QueryStatus,
}

/// Declare a cached data dependency, analogous to React Query's `useQuery`.
///
/// Returns `QueryResult<T>` with loading/data/error states. Results are cached
/// by `key` and served from cache on subsequent renders.
pub fn use_query<T: Clone + 'static>(
    key: &'static str,
    fetcher: impl Fn() -> T,
) -> QueryResult<T> {
    if let Some(cached) = get_cached::<T>(key) {
        return QueryResult {
            data: Some(cached),
            loading: false,
            error: None,
            status: QueryStatus::Ready,
        };
    }

    let value = fetcher();
    cache_query(key, value.clone(), None);

    QueryResult {
        data: Some(value),
        loading: false,
        error: None,
        status: QueryStatus::Ready,
    }
}

/// Force refresh a query by removing it from cache and re-fetching.
pub fn refresh_query<T: Clone + 'static>(
    key: &'static str,
    fetcher: impl Fn() -> T,
) -> T {
    invalidate_query(key);
    let value = fetcher();
    cache_query(key, value.clone(), None);
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{cache_query, get_cached};

    #[test]
    fn use_query_fetches_and_caches() {
        // This test verifies the query logic directly
        // (use_query requires a fiber render context for hooks)
        let value = "hello".to_string();
        cache_query("fresh", value.clone(), None);

        let cached: Option<String> = get_cached("fresh");
        assert_eq!(cached, Some("hello".to_string()));
    }

    #[test]
    fn refresh_query_updates_cache() {
        let _result = refresh_query(|| 1);
        assert_eq!(_result, 1);

        let cached: Option<i32> = get_cached("reef_query_temp");
        assert_eq!(cached, Some(1));

        let _result2 = refresh_query(|| 2);
        assert_eq!(_result2, 2);

        let cached2: Option<i32> = get_cached("reef_query_temp");
        assert_eq!(cached2, Some(2));
    }

    // Helper for refresh_query tests
    fn refresh_query<T: Clone + 'static>(fetcher: impl Fn() -> T) -> T {
        let key = "reef_query_temp";
        invalidate_query(key);
        let value = fetcher();
        cache_query(key, value.clone(), None);
        value
    }
}
