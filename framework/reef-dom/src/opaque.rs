use reef_draw::primitive::DrawPrimitive;
use std::cell::RefCell;
use std::collections::HashMap;

/// Thread-local storage for opaque draw plan primitives.
///
/// Allows passing pre-rendered primitives through the VNode → SceneNode → paint
/// pipeline without modifying VElement or SceneNode structs.
///
/// Usage:
/// 1. Store primitives: `OPAQUE_PLANS.with(|p| p.borrow_mut().insert(id, prims));`
/// 2. Reference in VNode prop: `("__opaque_id", PropValue::I32(id))`
/// 3. Look up in paint: `OPAQUE_PLANS.with(|p| p.borrow().get(&id).cloned())`
thread_local! {
    pub(crate) static OPAQUE_PLANS: RefCell<HashMap<i32, Vec<DrawPrimitive>>> =
        RefCell::new(HashMap::new());
}

/// Register opaque primitives and return their ID.
pub fn register_opaque_plan(primitives: Vec<DrawPrimitive>) -> i32 {
    let id = OPAQUE_PLANS.with(|p| {
        let mut map = p.borrow_mut();
        let id = map.len() as i32 + 1;
        map.insert(id, primitives);
        id
    });
    id
}

/// Look up opaque primitives by ID.
pub fn get_opaque_plan(id: i32) -> Option<Vec<DrawPrimitive>> {
    OPAQUE_PLANS.with(|p| p.borrow().get(&id).cloned())
}

/// Remove opaque primitives by ID (cleanup).
pub fn remove_opaque_plan(id: i32) {
    OPAQUE_PLANS.with(|p| {
        p.borrow_mut().remove(&id);
    });
}

/// Clear all opaque plans (called at the start of each frame).
pub fn clear_opaque_plans() {
    OPAQUE_PLANS.with(|p| {
        p.borrow_mut().clear();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::geometry::Rect;
    use reef_core::color::Color;

    #[test]
    fn register_and_retrieve() {
        let prims = vec![DrawPrimitive::RoundRect {
            frame: Rect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 },
            radius: 2.0,
            color: Color::WHITE,
            alpha: 1.0,
        }];
        let id = register_opaque_plan(prims.clone());
        let retrieved = get_opaque_plan(id);
        assert_eq!(retrieved, Some(prims));
    }

    #[test]
    fn remove_clears_plan() {
        let id = register_opaque_plan(vec![]);
        assert!(get_opaque_plan(id).is_some());
        remove_opaque_plan(id);
        assert!(get_opaque_plan(id).is_none());
    }
}
