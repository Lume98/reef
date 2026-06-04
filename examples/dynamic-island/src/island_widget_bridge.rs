//! 兼容门面：继续从 example 路径暴露灵动岛 page / bridge API。

pub use reef_native_panel_core::{
    build_dynamic_island, build_dynamic_island_page_state, build_island_widget,
    dynamic_island_target_for_hit_target, island_render_overrides, render_dynamic_island_page,
    resolve_dynamic_island_action, resolve_dynamic_island_effect,
    resolve_dynamic_island_gesture_effect, resolve_dynamic_island_platform_event,
    resolve_dynamic_island_target_action, resolve_dynamic_island_target_effect,
    resolve_dynamic_island_transition_request, DynamicIslandPageState, DynamicIslandRuntimeAction,
    DynamicIslandRuntimeEffect,
};
