//! 原生面板统一渲染 facade。
//!
//! 这个模块把视觉 primitive、视觉计划、表现模型、渲染命令、渲染后端和组件构建能力
//! 收拢到单一入口，供上层直接依赖。

pub use super::action_button_visual_spec::*;
pub use super::animation_plan::*;
pub use super::animation_scheduler::*;
pub use super::card_visual_spec::*;
pub use super::completion_glow_visual_spec::*;
pub use super::component_models::*;
pub use super::descriptors::*;
pub use super::mascot_sprite_spec::*;
pub use super::mascot_visual_spec::*;
pub use super::presentation_model::*;
pub use super::render_bundle::{
    resolve_native_panel_frame_submission_for_render_command_bundle,
    resolve_native_panel_render_command_bundle, NativePanelActionButtonCommand,
    NativePanelCardStackCommand, NativePanelCompactBarCommand, NativePanelRenderBundle,
};
pub use super::rendering_backend::{
    native_panel_frame_submission_from_visual_plan, native_panel_submit_visual_plan, DrawBackend,
    FrameSubmission,
};
pub use super::transition_controller::*;
pub use super::visual_plan::{
    native_panel_visual_card_input_from_scene_card,
    native_panel_visual_card_input_from_scene_card_with_height,
    resolve_native_panel_compact_bar_visual_plan, resolve_native_panel_visual_plan,
    NativePanelPaintInput, NativePanelVisualActionButtonInput, NativePanelVisualCardBadgeInput,
    NativePanelVisualCardBodyLineInput, NativePanelVisualCardBodyRole, NativePanelVisualCardInput,
    NativePanelVisualCardRowInput, NativePanelVisualCardStyle, NativePanelVisualDisplayMode,
};
pub use super::visual_primitives::{
    draw_point, draw_rect, native_panel_visual_plain_text_frame,
    native_panel_visual_text_box_height, native_panel_visual_text_box_height_for_role,
    native_panel_visual_text_frame, NativePanelVisualColor, NativePanelVisualShoulderSide,
    NativePanelVisualTextAlignment, NativePanelVisualTextRole, NativePanelVisualTextWeight,
};

pub mod descriptor {
    pub use super::super::descriptors::*;
}

pub mod presentation {
    pub use super::super::action_button_visual_spec::*;
    pub use super::super::card_visual_spec::*;
    pub use super::super::completion_glow_visual_spec::*;
    pub use super::super::mascot_sprite_spec::*;
    pub use super::super::mascot_visual_spec::*;
    pub use super::super::presentation_model::*;
    pub use super::super::render_bundle::{
        NativePanelActionButtonCommand, NativePanelCardStackCommand, NativePanelCompactBarCommand,
    };
    pub use super::super::visual_plan::{
        native_panel_visual_card_input_from_scene_card, NativePanelPaintInput,
        NativePanelVisualActionButtonInput, NativePanelVisualCardBadgeInput,
        NativePanelVisualCardBodyLineInput, NativePanelVisualCardBodyRole,
        NativePanelVisualCardInput, NativePanelVisualCardRowInput, NativePanelVisualCardStyle,
        NativePanelVisualDisplayMode,
    };
    pub use super::super::visual_primitives::NativePanelVisualColor;
}

pub mod components {
    pub use super::super::component_models::{
        build_native_panel_component_tree, build_native_panel_component_tree_from_presentation,
        build_native_panel_component_tree_from_presentation_and_cards,
        build_native_panel_component_tree_from_visual_plan, NativePanelCompactBarComponent,
        NativePanelComponent, NativePanelComponentTree, NativePanelContainerComponent,
        NativePanelLayoutSpacing, NativePanelMastheadComponent, NativePanelPanelColors,
        NativePanelSessionCardComponent, NativePanelSettingRowComponent, NativePanelStackComponent,
    };

    pub mod base {
        pub use super::super::super::component_models::{
            NativePanelComponent, NativePanelComponentTree, NativePanelPanelColors,
        };
    }

    pub mod container {
        pub use super::super::super::component_models::NativePanelContainerComponent;
    }

    pub mod content {
        pub use super::super::super::component_models::{
            NativePanelCompactBarComponent, NativePanelSessionCardComponent,
            NativePanelSettingRowComponent, NativePanelStackComponent,
        };
    }

    pub mod decoration {
        pub use super::super::super::component_models::NativePanelMastheadComponent;
    }
}

pub mod rendering {
    pub use super::super::rendering_backend::*;
}

pub mod visual {
    pub use super::super::visual_plan::{
        native_panel_visual_card_input_from_scene_card,
        native_panel_visual_card_input_from_scene_card_with_height,
        resolve_native_panel_compact_bar_visual_plan, resolve_native_panel_visual_plan,
        NativePanelPaintInput, NativePanelVisualActionButtonInput, NativePanelVisualCardBadgeInput,
        NativePanelVisualCardBodyLineInput, NativePanelVisualCardBodyRole,
        NativePanelVisualCardInput, NativePanelVisualCardRowInput, NativePanelVisualCardStyle,
        NativePanelVisualDisplayMode,
    };
    pub use super::super::visual_primitives::{
        draw_point, draw_rect, native_panel_visual_plain_text_frame,
        native_panel_visual_text_box_height, native_panel_visual_text_box_height_for_role,
        native_panel_visual_text_frame, NativePanelVisualColor, NativePanelVisualShoulderSide,
        NativePanelVisualTextAlignment, NativePanelVisualTextRole, NativePanelVisualTextWeight,
    };
}
