//! 原生面板平台无关 UI 计算。
//!
//! 这里只维护宿主窗口描述、表现模型、视觉计划和渲染命令等纯 UI 输出。
//! Tauri 生命周期、平台线程、窗口消息和 Direct2D/DirectWrite 绘制继续留在宿主层。

mod action_button_visual_spec;
mod animation_plan;
mod animation_scheduler;
mod card_visual_spec;
mod completion_glow_visual_spec;
mod component_models;
mod descriptors;
mod env_flags;
mod mascot_sprite_spec;
mod mascot_visual_spec;
mod presentation_model;
mod render_bundle;
mod rendering_backend;
mod transition_controller;
mod visual_plan;
mod visual_primitives;
mod widget_bridge;
mod widget_migration;

pub mod render;

pub mod descriptor {
    pub use super::render::descriptor::*;
}

pub mod presentation {
    pub use super::render::presentation::*;
}

pub mod components {
    pub use super::render::components::*;
}

pub mod rendering {
    pub use super::render::rendering::*;
}

pub mod visual {
    pub use super::render::visual::*;
}
