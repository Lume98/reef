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
    pub use super::render_bundle::{
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

pub mod components {
    pub use super::component_models::{
        build_native_panel_component_tree, build_native_panel_component_tree_from_presentation,
        build_native_panel_component_tree_from_presentation_and_cards,
        build_native_panel_component_tree_from_visual_plan, NativePanelLayoutSpacing,
    };

    pub mod base {
        pub use super::super::component_models::{
            NativePanelComponent, NativePanelComponentTree, NativePanelPanelColors,
        };
    }

    pub mod container {
        pub use super::super::component_models::NativePanelContainerComponent;
    }

    pub mod content {
        pub use super::super::component_models::{
            NativePanelCompactBarComponent, NativePanelSessionCardComponent,
            NativePanelSettingRowComponent, NativePanelStackComponent,
        };
    }

    pub mod decoration {
        pub use super::super::component_models::NativePanelMastheadComponent;
    }

    pub use base::*;
    pub use container::*;
    pub use content::*;
    pub use decoration::*;
}

pub mod render {
    pub use super::animation_plan::*;
    pub use super::animation_scheduler::*;
    pub use super::render_bundle::*;
    pub use super::transition_controller::*;
}

pub mod rendering {
    pub use super::rendering_backend::*;
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
