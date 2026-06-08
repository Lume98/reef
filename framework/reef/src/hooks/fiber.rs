use std::sync::atomic::{AtomicU64, Ordering};

/// A unique identifier for a fiber node in the reconciler tree.
///
/// Fiber IDs are assigned monotonically and are used by the hook registry
/// to associate hook state with specific component instances.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FiberId(pub u64);

static NEXT_FIBER_ID: AtomicU64 = AtomicU64::new(1);

impl FiberId {
    /// Allocate a new globally-unique fiber ID.
    ///
    /// The reconciler calls this when creating a new fiber node.
    pub fn allocate() -> Self {
        FiberId(NEXT_FIBER_ID.fetch_add(1, Ordering::Relaxed))
    }

    /// The root fiber always has ID 0.
    pub const ROOT: FiberId = FiberId(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fiber_id_allocation() {
        let a = FiberId::allocate();
        let b = FiberId::allocate();
        assert_ne!(a, b);
        assert_eq!(FiberId::ROOT, FiberId(0));
    }

    #[test]
    fn fiber_id_clone_and_copy() {
        let a = FiberId(42);
        let b = a;
        assert_eq!(a, b);
        let c = a.clone();
        assert_eq!(a, c);
    }
}
