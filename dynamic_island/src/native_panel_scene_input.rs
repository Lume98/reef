//! 面板场景输入适配层。
//!
//! 这里只保留对业务层构造函数的薄 reexport，避免把显示器选择和设置推导逻辑继续放在
//! 适配层里。

#![allow(unused_imports)]

pub(crate) use crate::business::{
    native_panel_runtime_input_context_from_display_options,
    native_panel_runtime_input_context_from_display_options_with_screen_frame,
    native_panel_runtime_input_descriptor_from_app_settings,
    native_panel_runtime_input_descriptor_from_context,
    native_panel_runtime_input_descriptor_from_display_options_with_screen_frame,
    panel_display_options_from_display_options, panel_scene_build_input_from_app_settings,
    panel_scene_build_input_from_display_options,
    resolve_next_display_selection_update_from_display_options,
    resolve_panel_selected_display_index, resolve_selected_display_index_from_display_options,
    NativePanelDisplaySelectionUpdate,
};
