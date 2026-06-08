//! 灵动岛业务层 façade。
//!
//! 这里收拢当前项目的业务规则和状态入口，避免视图层直接散落地依赖各个内部模块。

mod display_selection;
mod input;

pub(crate) use display_selection::{
    resolve_next_display_selection_update_from_display_options,
    resolve_panel_selected_display_index, resolve_selected_display_index_from_display_options,
    NativePanelDisplaySelectionUpdate,
};
pub(crate) use input::{
    native_panel_runtime_input_context_from_display_options,
    native_panel_runtime_input_context_from_display_options_with_screen_frame,
    native_panel_runtime_input_descriptor_from_app_settings,
    native_panel_runtime_input_descriptor_from_context,
    native_panel_runtime_input_descriptor_from_display_options_with_screen_frame,
    panel_display_options_from_display_options, panel_scene_build_input_from_app_settings,
    panel_scene_build_input_from_display_options,
};

#[cfg(test)]
mod tests;
