use std::marker::PhantomData;
use crate::{HOOK_REGISTRY, FiberId};

/// A handle that can update a `use_state` value and trigger re-render.
///
/// Created by `use_state` and returned alongside the current value.
/// Cloning produces another handle to the same state slot.
#[derive(Debug)]
pub struct StateSetter<T> {
    pub(crate) fiber_id: FiberId,
    pub(crate) slot: usize,
    _phantom: PhantomData<T>,
}

impl<T> Clone for StateSetter<T> {
    fn clone(&self) -> Self {
        Self {
            fiber_id: self.fiber_id,
            slot: self.slot,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for StateSetter<T> {}

impl<T: 'static> StateSetter<T> {
    /// Replace the current state value and schedule a re-render.
    pub fn set(&self, value: T) {
        HOOK_REGISTRY.with(|reg| {
            let mut reg = reg.borrow_mut();
            reg.set_value(self.fiber_id, self.slot, Box::new(HookStateValue(value)));
            reg.schedule_update(self.fiber_id);
        });
    }

    /// Update the current state by applying a function to the existing value.
    pub fn update(&self, f: impl FnOnce(&T) -> T) {
        HOOK_REGISTRY.with(|reg| {
            let mut reg = reg.borrow_mut();
            let new_value = {
                let current = reg
                    .get_value::<HookStateValue<T>>(self.fiber_id, self.slot)
                    .map(|v| &v.0);
                match current {
                    Some(val) => f(val),
                    None => return,
                }
            };
            reg.set_value(self.fiber_id, self.slot, Box::new(HookStateValue(new_value)));
            reg.schedule_update(self.fiber_id);
        });
    }
}

/// Internal wrapper stored in the hook registry slot.
#[derive(Debug)]
pub(crate) struct HookStateValue<T>(pub T);

/// Declare a state variable, analogous to React's `useState`.
///
/// Returns `(current_value, setter)`. Calling `setter.set(new_value)` or
/// `setter.update(fn)` replaces the value and schedules a re-render of the
/// owning component.
///
/// # Panics
/// Panics if called outside of a component's render function (no current fiber).
pub fn use_state<T: Clone + 'static>(initial: T) -> (T, StateSetter<T>) {
    HOOK_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        let fiber = reg.current_fiber.expect("use_state must be called during render");
        let slot = reg.next_slot();

        // Initialize if this slot hasn't been written to yet
        if reg.get_value::<HookStateValue<T>>(fiber, slot).is_none() {
            reg.set_value(fiber, slot, Box::new(HookStateValue(initial)));
        }

        let current = reg
            .get_value::<HookStateValue<T>>(fiber, slot)
            .map(|v| v.0.clone())
            .expect("use_state slot just initialized");

        let setter = StateSetter {
            fiber_id: fiber,
            slot,
            _phantom: PhantomData,
        };

        (current, setter)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HOOK_REGISTRY;

    #[test]
    fn use_state_initial_value() {
        let fiber = FiberId::allocate();
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().begin_fiber(fiber));

        let (count, _setter) = use_state(42_i32);
        assert_eq!(count, 42);

        HOOK_REGISTRY.with(|reg| reg.borrow_mut().end_fiber());
    }

    #[test]
    fn use_state_persists_across_renders() {
        let fiber = FiberId::allocate();

        // First render
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().begin_fiber(fiber));
        let (count, setter) = use_state(0_i32);
        assert_eq!(count, 0);
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().end_fiber());

        // Update via setter
        setter.set(10);

        // Second render
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().begin_fiber(fiber));
        let (count, _) = use_state::<i32>(0_i32); // initial ignored on subsequent renders
        assert_eq!(count, 10);
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().end_fiber());
    }

    #[test]
    fn use_state_update_fn() {
        let fiber = FiberId::allocate();

        HOOK_REGISTRY.with(|reg| reg.borrow_mut().begin_fiber(fiber));
        let (_, setter) = use_state(0_i32);
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().end_fiber());

        setter.update(|prev| prev + 5);

        HOOK_REGISTRY.with(|reg| reg.borrow_mut().begin_fiber(fiber));
        let (count, _) = use_state::<i32>(0);
        assert_eq!(count, 5);
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().end_fiber());
    }

    #[test]
    fn setter_clone_and_copy() {
        let fiber = FiberId::allocate();

        HOOK_REGISTRY.with(|reg| reg.borrow_mut().begin_fiber(fiber));
        let (_, setter) = use_state(0_i32);
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().end_fiber());

        let cloned = setter; // copy
        cloned.set(7);

        HOOK_REGISTRY.with(|reg| reg.borrow_mut().begin_fiber(fiber));
        let (count, _) = use_state::<i32>(0);
        assert_eq!(count, 7);
        HOOK_REGISTRY.with(|reg| reg.borrow_mut().end_fiber());
    }

    #[test]
    #[should_panic(expected = "use_state must be called during render")]
    fn use_state_panics_outside_render() {
        let _ = use_state::<i32>(0);
    }
}
