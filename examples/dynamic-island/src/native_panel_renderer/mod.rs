//! 原生面板渲染协调层。
//!
//! 这里不直接绑定 Win32 绘制 API，而是负责把场景模型转换为表现模型、视觉计划和渲染命令，
//! 并维护 hover/click/settings/transition 等运行时交互状态。平台实现通过内部 facade
//! 复用这些纯逻辑。

#![allow(dead_code, unused_imports)]

// UI 计算来自 reef-ui；本 crate 只保留运行时、宿主和平台协调。
pub(crate) use reef_ui::native_panel_ui::{
    descriptor as descriptors, presentation as action_button_visual_spec,
    presentation as card_visual_spec, presentation as completion_glow_visual_spec,
    presentation as mascot_sprite_spec, presentation as mascot_visual_spec,
    presentation as presentation_model, render as animation_plan, render as animation_scheduler,
    render as render_commands, render as transition_controller, rendering as rendering_backend,
    visual as visual_primitives,
};

mod close_preservation;
mod env_flags;
mod host_runtime_command;
mod host_runtime_dispatch;
mod host_runtime_facade;
mod host_runtime_state;
mod platform_adapter;
mod renderer_backend;

// 运行时同步：快照、输入、悬停、点击、设置面板和平台命令的协调。
mod runtime_backend;
mod runtime_click;
mod runtime_commands;
mod runtime_hover;
mod runtime_interaction;
mod runtime_pointer_input;
mod runtime_polling;
mod runtime_render_payload;
mod runtime_scene_cache;
mod runtime_scene_sync;
mod runtime_settings_surface;
mod runtime_transition_slots;
mod shell_command;
mod shell_pump;
mod shell_state;
mod traits;

// 快照测试框架：固定视觉输出，防止布局和动画计划意外漂移。
pub(crate) mod snapshot_testing;

#[cfg(test)]
mod snapshot_tests;
mod window_message_pump;

pub(crate) mod facade;
#[cfg(test)]
mod runtime_tests;
#[cfg(test)]
mod test_fixtures;
