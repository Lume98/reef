//! 原生面板平台无关 UI 计算。
//!
//! 这里只维护宿主窗口描述、表现模型、视觉计划和渲染命令等纯 UI 输出。
//! Tauri 生命周期、平台线程、窗口消息和 Direct2D/DirectWrite 绘制继续留在宿主层。

mod action_button_visual_spec;
mod animation_plan;
mod animation_scheduler;
mod card_visual_spec;
mod completion_glow_visual_spec;
mod descriptors;
mod env_flags;
mod mascot_sprite_spec;
mod mascot_visual_spec;
mod presentation_model;
mod render_commands;
mod transition_controller;
mod visual_plan;
mod visual_primitives;

pub mod descriptor {
    pub use super::descriptors::*;
}

pub mod presentation {
    pub use super::action_button_visual_spec::*;
    pub use super::card_visual_spec::*;
    pub use super::completion_glow_visual_spec::*;
    pub use super::mascot_sprite_spec::*;
    pub use super::mascot_visual_spec::*;
    pub use super::presentation_model::*;
    pub use super::render_commands::{
        NativePanelActionButtonCommand, NativePanelCardStackCommand, NativePanelCompactBarCommand,
    };
    pub use super::visual_plan::{
        native_panel_visual_card_input_from_scene_card, NativePanelVisualActionButtonInput,
        NativePanelVisualCardBadgeInput, NativePanelVisualCardBodyLineInput,
        NativePanelVisualCardBodyRole, NativePanelVisualCardInput, NativePanelVisualCardRowInput,
        NativePanelVisualCardStyle, NativePanelVisualDisplayMode, NativePanelVisualPlanInput,
    };
    pub use super::visual_primitives::NativePanelVisualColor;
}

pub mod render {
    pub use super::animation_plan::*;
    pub use super::animation_scheduler::*;
    pub use super::render_commands::*;
    pub use super::transition_controller::*;
}

pub mod visual {
    pub use super::visual_plan::{
        native_panel_visual_card_input_from_scene_card,
        native_panel_visual_card_input_from_scene_card_with_height,
        resolve_native_panel_compact_bar_visual_plan, resolve_native_panel_visual_plan,
        NativePanelVisualActionButtonInput, NativePanelVisualCardBadgeInput,
        NativePanelVisualCardBodyLineInput, NativePanelVisualCardBodyRole,
        NativePanelVisualCardInput, NativePanelVisualCardRowInput, NativePanelVisualCardStyle,
        NativePanelVisualDisplayMode, NativePanelVisualPlanInput,
    };
    pub use super::visual_primitives::*;
}
