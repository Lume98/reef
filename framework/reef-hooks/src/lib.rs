pub mod fiber;
pub mod state;
pub mod effect;
pub mod context;

pub use fiber::FiberId;
pub use state::use_state;
pub use effect::use_effect;
pub use context::{create_context, use_context, Context};

use std::cell::RefCell;
use std::collections::HashMap;

// ── Hook Registry ─────────────────────────────────────────────────

/// Per-fiber hook storage: maps slot index → boxed value.
#[derive(Default)]
pub(crate) struct FiberHookStorage {
    /// Slot-indexed values.
    pub slots: Vec<Box<dyn std::any::Any>>,
    /// Pending effects to run after commit.
    pub pending_effects: Vec<EffectRecord>,
}

pub(crate) struct EffectRecord {
    pub setup: Box<dyn Fn() -> Option<Box<dyn FnOnce()>>>,
    pub cleanup: Option<Box<dyn FnOnce()>>,
    pub deps_valid: bool,
}

thread_local! {
    pub(crate) static HOOK_REGISTRY: RefCell<HookRegistry> = RefCell::new(HookRegistry::new());
}

/// Global hook state: fiber → slots, current fiber tracking, scheduler.
pub(crate) struct HookRegistry {
    /// Per-fiber storage.
    pub fibers: HashMap<FiberId, FiberHookStorage>,
    /// Fiber currently being rendered (set by reconciler).
    pub current_fiber: Option<FiberId>,
    /// Slot counter for the current fiber (incremented per hook call).
    pub current_slot: usize,
    /// Scheduler callback — set by reconciler to trigger re-render.
    pub scheduler: Option<Box<dyn Fn(FiberId)>>,
}

impl HookRegistry {
    pub fn new() -> Self {
        Self {
            fibers: HashMap::new(),
            current_fiber: None,
            current_slot: 0,
            scheduler: None,
        }
    }

    /// Begin rendering a fiber — resets slot counter.
    pub fn begin_fiber(&mut self, id: FiberId) {
        self.current_fiber = Some(id);
        self.current_slot = 0;
    }

    /// End rendering current fiber.
    pub fn end_fiber(&mut self) {
        self.current_fiber = None;
        self.current_slot = 0;
    }

    /// Get next slot and ensure storage exists.
    pub fn next_slot(&mut self) -> usize {
        let slot = self.current_slot;
        self.current_slot += 1;
        slot
    }

    /// Set a value at (fiber, slot).
    pub fn set_value(&mut self, fiber: FiberId, slot: usize, value: Box<dyn std::any::Any>) {
        let storage = self.fibers.entry(fiber).or_default();
        if slot >= storage.slots.len() {
            storage.slots.resize_with(slot + 1, || Box::new(()));
        }
        storage.slots[slot] = value;
    }

    /// Get (fiber, slot) value by downcasting to T.
    pub fn get_value<T: 'static>(&self, fiber: FiberId, slot: usize) -> Option<&T> {
        self.fibers.get(&fiber)?.slots.get(slot)?.downcast_ref::<T>()
    }

    /// Schedule re-render for a fiber.
    pub fn schedule_update(&self, fiber: FiberId) {
        if let Some(ref sched) = self.scheduler {
            (sched)(fiber);
        }
    }

    /// Set the scheduler callback (called by reconciler).
    pub fn set_scheduler(&mut self, sched: Box<dyn Fn(FiberId)>) {
        self.scheduler = Some(sched);
    }
}

// ── Public scheduler setter ───────────────────────────────────────

/// Set the global scheduler callback used by `use_state` setters to
/// trigger re-renders. Called by the reconciler during initialization.
pub fn set_scheduler(sched: Box<dyn Fn(FiberId)>) {
    HOOK_REGISTRY.with(|reg| reg.borrow_mut().set_scheduler(sched));
}

/// Return the currently rendering fiber ID, or `None` if called outside
/// a render context (useful for testing / stubs).
pub fn current_fiber_id() -> Option<FiberId> {
    HOOK_REGISTRY.with(|reg| reg.borrow().current_fiber)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_fiber_lifecycle() {
        let mut reg = HookRegistry::new();
        let id = FiberId(42);

        assert!(reg.current_fiber.is_none());
        reg.begin_fiber(id);
        assert_eq!(reg.current_fiber, Some(id));
        assert_eq!(reg.current_slot, 0);

        let s1 = reg.next_slot();
        let s2 = reg.next_slot();
        assert_eq!(s1, 0);
        assert_eq!(s2, 1);

        reg.end_fiber();
        assert!(reg.current_fiber.is_none());
    }

    #[test]
    fn registry_set_and_get_value() {
        let mut reg = HookRegistry::new();
        let id = FiberId(1);

        reg.set_value(id, 0, Box::new(42_i32));
        reg.set_value(id, 1, Box::new("hello".to_string()));

        assert_eq!(reg.get_value::<i32>(id, 0), Some(&42));
        assert_eq!(reg.get_value::<String>(id, 1), Some(&"hello".to_string()));
        assert_eq!(reg.get_value::<i32>(id, 99), None);
    }
}
