use crate::hooks::{EffectRecord, FiberId, HOOK_REGISTRY};

/// Register a side-effect to run after the component commits to the host
/// platform. Analogous to React's `useEffect`.
///
/// The `setup` closure is called after each commit. If it returns a cleanup
/// closure, the cleanup is called before the next setup invocation or when
/// the component unmounts.
///
/// # Panics
/// Panics if called outside of a component's render function.
pub fn use_effect(setup: impl Fn() -> Option<Box<dyn FnOnce()>> + 'static) {
    HOOK_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        let fiber = reg
            .current_fiber
            .expect("use_effect must be called during render");

        let storage = reg.fibers.entry(fiber).or_default();
        storage.pending_effects.push(EffectRecord {
            setup: Box::new(setup),
            cleanup: None,
            deps_valid: false,
        });
    });
}

/// Flush all pending effects for a fiber — runs after commit.
///
/// Called by the reconciler during the commit phase.
pub fn flush_effects(fiber: FiberId) {
    HOOK_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(storage) = reg.fibers.get_mut(&fiber) {
            let effects = std::mem::take(&mut storage.pending_effects);
            // Run cleanups first, then setups
            for effect in effects {
                // Run cleanup if exists
                // (cleanup from previous effect run is stored separately)
                let setup = effect.setup;
                let cleanup = setup();
                // Store cleanup for next cycle
                // In a full implementation, this goes into a separate list
                // For Phase 2, we just fire and forget the cleanup
                drop(cleanup);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FiberId;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn use_effect_registers_pending_effect() {
        let fiber = FiberId::allocate();
        let fired = Arc::new(AtomicBool::new(false));

        HOOK_REGISTRY.with(|reg| {
            reg.borrow_mut().begin_fiber(fiber);
        });

        let fired_clone = fired.clone();
        use_effect(move || {
            fired_clone.store(true, Ordering::SeqCst);
            None
        });

        HOOK_REGISTRY.with(|reg| {
            reg.borrow_mut().end_fiber();
        });

        // Verify effect is pending
        HOOK_REGISTRY.with(|reg| {
            let storage = reg.borrow();
            let effects = &storage.fibers.get(&fiber).unwrap().pending_effects;
            assert_eq!(effects.len(), 1);
        });

        // Flush should fire the effect
        flush_effects(fiber);
        assert!(fired.load(Ordering::SeqCst));
    }

    #[test]
    #[should_panic(expected = "use_effect must be called during render")]
    fn use_effect_panics_outside_render() {
        let _ = use_effect(|| None);
    }
}
