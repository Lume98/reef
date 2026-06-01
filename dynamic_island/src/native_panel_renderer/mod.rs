//! 原生面板渲染协调层。
//!
//! 这里不直接绑定 Win32 绘制 API，而是负责把场景模型转换为表现模型、视觉计划和渲染命令，
//! 并维护 hover/click/settings/transition 等运行时交互状态。平台实现通过内部 facade
//! 复用这些纯逻辑。

#![allow(dead_code, unused_imports)]

// UI 计算来自 echoisland-ui；本 crate 只保留运行时、宿主和平台协调。
pub(crate) use echoisland_ui::native_panel_ui::{
    descriptor as descriptors, presentation as action_button_visual_spec,
    presentation as card_visual_spec, presentation as completion_glow_visual_spec,
    presentation as mascot_sprite_spec, presentation as mascot_visual_spec,
    presentation as presentation_model, render as animation_plan, render as animation_scheduler,
    render as render_commands, render as transition_controller, visual as visual_primitives,
};

pub(crate) mod visual_plan {
    pub(crate) use echoisland_ui::native_panel_ui::visual::{
        native_panel_visual_card_input_from_scene_card,
        native_panel_visual_card_input_from_scene_card_with_height,
        NativePanelVisualActionButtonInput, NativePanelVisualCardBadgeInput,
        NativePanelVisualCardBodyLineInput, NativePanelVisualCardBodyRole,
        NativePanelVisualCardInput, NativePanelVisualCardRowInput, NativePanelVisualCardStyle,
        NativePanelVisualDisplayMode, NativePanelVisualPlan, NativePanelVisualPlanInput,
    };

    pub(crate) fn resolve_native_panel_visual_plan(
        input: &NativePanelVisualPlanInput,
    ) -> NativePanelVisualPlan {
        disable_mascot_sprite_in_tests();
        echoisland_ui::native_panel_ui::visual::resolve_native_panel_visual_plan(input)
    }

    pub(crate) fn resolve_native_panel_compact_bar_visual_plan(
        input: &NativePanelVisualPlanInput,
    ) -> NativePanelVisualPlan {
        disable_mascot_sprite_in_tests();
        echoisland_ui::native_panel_ui::visual::resolve_native_panel_compact_bar_visual_plan(input)
    }

    #[cfg(test)]
    fn disable_mascot_sprite_in_tests() {
        std::env::set_var("ECHOISLAND_MASCOT_SPRITE", "0");
    }

    #[cfg(not(test))]
    fn disable_mascot_sprite_in_tests() {}
}

mod close_preservation;
mod env_flags;
mod host_runtime_facade;
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

#[cfg(test)]
mod runtime_tests;
#[cfg(test)]
mod test_fixtures;

// 内部门面：按职责重导出子模块能力，减少 Windows 平台层对具体文件路径的耦合。
pub(crate) mod facade {
    // 平台事件和运行时命令派发。
    pub(crate) mod command {
        pub(crate) use super::super::descriptors::{
            NativePanelPlatformEvent, NativePanelPointerInput, NativePanelPointerInputOutcome,
            NativePanelRuntimeCommandCapability, NativePanelRuntimeCommandHandler,
        };
        #[cfg(feature = "tauri-host")]
        pub(crate) use super::super::host_runtime_facade::NativePanelRuntimeDispatchMode;
        pub(crate) use super::super::runtime_click::dispatch_queued_native_panel_platform_events_with_handler;
        #[cfg(feature = "tauri-host")]
        pub(crate) use super::super::runtime_commands::{
            dispatch_drained_native_panel_platform_events_with_app_handle,
            dispatch_native_panel_click_command_with_app_handle,
            dispatch_native_panel_platform_events_with_app_handle,
            execute_native_panel_cycle_display_command,
            run_native_panel_pointer_input_with_queued_command_dispatch,
            run_native_panel_runtime_with_queued_command_dispatch,
            spawn_native_panel_platform_event_dispatch_loop,
            spawn_native_panel_platform_loops_with_event_dispatch,
        };
        pub(crate) use super::super::runtime_commands::{
            execute_native_panel_cycle_island_width_command,
            execute_native_panel_cycle_language_command,
            execute_native_panel_debug_mode_trigger_command,
            execute_native_panel_settings_surface_command,
            execute_native_panel_toggle_completion_sound_command,
            execute_native_panel_toggle_mascot_command,
        };
    }

    // 宿主窗口、指针区域和时间线描述符。
    pub(crate) mod descriptor {
        pub(crate) use super::super::descriptors::{
            native_panel_host_window_descriptor, native_panel_host_window_frame,
            native_panel_pointer_inside_for_input, native_panel_pointer_inside_regions,
            native_panel_pointer_state_at_point, native_panel_timeline_descriptor,
            native_panel_timeline_descriptor_for_animation, resolve_native_panel_interaction_plan,
            resolve_native_panel_pointer_regions, NativePanelEdgeAction,
            NativePanelEdgeActionFrames, NativePanelHostWindowDescriptor,
            NativePanelHostWindowState, NativePanelInteractionPlan, NativePanelPointerInput,
            NativePanelPointerPointState, NativePanelPointerRegion, NativePanelPointerRegionInput,
            NativePanelPointerRegionKind, NativePanelRuntimeInputContext,
            NativePanelRuntimeInputDescriptor, NativePanelTimelineDescriptor,
        };
        pub(crate) use super::super::host_runtime_facade::NativePanelComputedHostWindow;
    }

    // 环境开关解析。
    pub(crate) mod env {
        pub(crate) use super::super::env_flags::{
            native_panel_enabled_from_env_value, native_panel_feature_enabled_from_env_value,
        };
    }

    // 宿主窗口状态同步和渲染器缓存访问。
    pub(crate) mod host {
        pub(crate) use super::super::host_runtime_facade::{
            create_native_panel_via_host_controller, hide_native_panel_via_host_controller,
            native_panel_host_display_reposition,
            native_panel_host_display_reposition_from_input_descriptor,
            reposition_native_panel_host_from_input_descriptor_via_controller,
            set_native_panel_host_shared_body_height_via_controller,
            sync_runtime_host_display_reposition_in_state, sync_runtime_host_screen_frame_in_state,
            sync_runtime_host_shared_body_height_in_state, sync_runtime_host_timeline_in_state,
            sync_runtime_host_visibility_in_state, sync_runtime_pointer_regions_in_state,
            NativePanelHostDisplayReposition, NativePanelRuntimeHostController,
            NativePanelRuntimeHostState,
        };
        pub(crate) use super::super::renderer_backend::native_panel_presentation_cards_visible;
        pub(crate) use super::super::traits::{NativePanelHost, NativePanelSceneHost};
    }

    // Hover、click、轮询和设置面板交互。
    pub(crate) mod interaction {
        pub(crate) use super::super::descriptors::NativePanelRuntimeCommandHandler;
        pub(crate) use super::super::runtime_click::dispatch_native_panel_click_command_at_point_with_handler;
        pub(crate) use super::super::runtime_hover::{
            sync_native_panel_hover_expansion_state_for_state,
            sync_native_panel_hover_interaction_and_rerender_at_point_with_input_descriptor,
            sync_native_panel_hover_interaction_and_rerender_for_inside_with_input_descriptor,
            sync_native_panel_hover_interaction_and_rerender_for_pointer_input_with_input_descriptor,
            sync_native_panel_hover_interaction_at_point_for_state,
            sync_native_panel_hover_interaction_for_pointer_input_for_state,
            sync_native_panel_hover_interaction_for_state,
        };
        pub(crate) use super::super::runtime_interaction::{
            native_panel_click_state_slots, record_native_panel_focus_click_session,
            resolve_native_panel_last_focus_click, NativePanelClickStateBridge,
            NativePanelCoreStateBridge, NativePanelHostBehaviorCommand,
            NativePanelHostBehaviorPlan, NativePanelHostInteractionStateBridge,
            NativePanelHostPollingInteractionResult, NativePanelHoverFallbackFrames,
            NativePanelHoverSyncResult, NativePanelPointerInputRuntimeBridge,
            NativePanelPointerRegionInteractionBridge, NativePanelPollingHostFacts,
            NativePanelPrimaryPointerStateBridge, NativePanelQueuedPlatformEventBridge,
            NativePanelSettingsSurfaceSnapshotUpdate, NativePanelSettingsSurfaceToggleResult,
        };
        pub(crate) use super::super::runtime_pointer_input::{
            handle_native_panel_pointer_input_with_handler,
            handle_optional_native_panel_pointer_input_with_handler,
        };
        pub(crate) use super::super::runtime_polling::{
            native_panel_interactive_inside_from_host_facts,
            native_panel_polling_interaction_input_from_host_facts,
            resolve_native_panel_host_behavior_plan, resolve_native_panel_hover_fallback_frames,
            resolve_native_panel_stable_compact_hover_frame,
            sync_native_panel_host_behavior_for_interactive_inside,
            sync_native_panel_host_polling_interaction_for_state,
            sync_native_panel_host_polling_interaction_from_host_facts_for_state,
            sync_native_panel_mouse_passthrough_for_interactive_inside,
        };
        pub(crate) use super::super::runtime_settings_surface::{
            resolve_native_panel_settings_surface_snapshot_update_for_state,
            toggle_native_panel_settings_surface_for_state,
        };
        pub(crate) use super::super::runtime_transition_slots::sync_native_panel_hover_and_refresh_for_runtime;
    }

    // 场景到表现模型/视觉规格的转换。
    pub(crate) mod presentation {
        pub(crate) use super::super::action_button_visual_spec::{
            action_button_transition_progress_from_compact_width,
            action_button_visual_frame_for_phase, resolve_action_button_visibility_spec,
            ActionButtonVisibilitySpecInput,
        };
        pub(crate) use super::super::card_visual_spec::{
            card_visual_action_hint_layout, card_visual_badge_layout, card_visual_body_layout,
            card_visual_body_line_paint_spec, card_visual_content_layout,
            card_visual_content_visibility_phase, card_visual_header_text_paint_spec,
            card_visual_settings_row_layout, card_visual_shell_border_color,
            card_visual_shell_fill_color, card_visual_single_line_text_box_frame,
            card_visual_spec_from_scene_card_with_height, card_visual_staggered_phase,
            card_visual_tool_pill_layout, CardVisualAnimationSpec, CardVisualBadgeRole,
            CardVisualBadgeSpec, CardVisualBodyRole, CardVisualBodySpec, CardVisualColorSpec,
            CardVisualRowSpec, CardVisualShellSpec, CardVisualSpec, CardVisualStyle,
        };
        pub(crate) use super::super::completion_glow_visual_spec::{
            resolve_completion_glow_image_slices, resolve_completion_glow_visual_spec,
            CompletionGlowImageSliceSpec, CompletionGlowVisualSpecInput,
            COMPLETION_GLOW_IMAGE_HEIGHT, COMPLETION_GLOW_IMAGE_RADIUS,
            COMPLETION_GLOW_IMAGE_WIDTH, COMPLETION_GLOW_SLICE_LEFT, COMPLETION_GLOW_SLICE_RIGHT,
            COMPLETION_GLOW_VISIBLE_THRESHOLD,
        };
        pub(crate) use super::super::mascot_sprite_spec::{
            parse_mascot_sprite_manifest, resolve_mascot_sprite_animation_key,
            resolve_mascot_sprite_frame, validate_mascot_sprite_manifest, MascotSpriteAnimationKey,
            MascotSpriteAnimationManifest, MascotSpriteFrameInput, MascotSpriteFrameSpec,
            MascotSpriteManifest, MascotSpriteSheetSpec,
        };
        pub(crate) use super::super::mascot_visual_spec::{
            resolve_mascot_visual_spec, MascotCompletionBadgeVisualSpec, MascotEllipseVisualSpec,
            MascotMessageBubbleVisualSpec, MascotTextVisualSpec, MascotVisualSpec,
            MascotVisualSpecInput,
        };
        pub(crate) use super::super::presentation_model::{
            estimated_scene_card_height, estimated_scene_content_height_for_card_width,
            native_panel_visual_display_mode_from_presentation,
            native_panel_visual_plan_input_from_presentation, resolve_native_panel_presentation,
            resolve_native_panel_presentation_model_for_scene,
            resolve_native_panel_snapshot_render_plan_for_scene,
            NativePanelActionButtonPresentation, NativePanelActionButtonsPresentation,
            NativePanelCardStackPresentation, NativePanelCompactBarPresentation,
            NativePanelGlowPresentation, NativePanelMascotPresentation,
            NativePanelPresentationMetrics, NativePanelPresentationModel,
            NativePanelResolvedPresentation, NativePanelShellPresentation,
            NativePanelSnapshotRenderPlan,
        };
        pub(crate) use super::super::render_commands::{
            NativePanelActionButtonCommand, NativePanelCardStackCommand,
            NativePanelCompactBarCommand,
        };
        pub(crate) use super::super::visual_plan::{
            native_panel_visual_card_input_from_scene_card, NativePanelVisualActionButtonInput,
            NativePanelVisualCardBadgeInput, NativePanelVisualCardBodyLineInput,
            NativePanelVisualCardBodyRole, NativePanelVisualCardInput,
            NativePanelVisualCardRowInput, NativePanelVisualCardStyle,
            NativePanelVisualDisplayMode, NativePanelVisualPlanInput,
        };
        pub(crate) use super::super::visual_primitives::NativePanelVisualColor;
    }

    // 动画计划、渲染命令和场景缓存。
    pub(crate) mod renderer {
        pub(crate) use super::super::animation_plan::{
            resolve_native_panel_animation_plan, resolve_native_panel_close_presentation_plan,
            resolve_native_panel_status_close_preservation_plan,
            resolve_native_panel_transition_lifecycle_plan, NativePanelAnimationPlan,
            NativePanelCardStackAnimationPlan, NativePanelClosePresentationInput,
            NativePanelClosePresentationPlan, NativePanelCloseTrigger,
            NativePanelStatusClosePreservationInput, NativePanelStatusClosePreservationPlan,
            NativePanelTransitionCardPhase, NativePanelTransitionLifecyclePlan,
        };
        pub(crate) use super::super::animation_scheduler::{
            NativePanelAnimationFrame, NativePanelAnimationFrameScheduler,
            NativePanelAnimationTarget,
        };
        pub(crate) use super::super::close_preservation::{
            apply_native_panel_preserved_close_presentation_slots,
            native_panel_runtime_render_state_from_preserved_scene,
            native_panel_status_close_preservation_active,
            native_panel_status_close_scene_has_cards,
            resolve_native_panel_preserved_status_close_scene,
            resolve_native_panel_preserved_status_close_scene_for_snapshot,
        };
        pub(crate) use super::super::render_commands::{
            resolve_native_panel_render_command_bundle, NativePanelRenderCommandBundle,
        };
        pub(crate) use super::super::renderer_backend::{
            cache_host_window_descriptor_on_renderer, cache_host_window_state_on_renderer,
            cache_pointer_regions_on_renderer, cache_render_command_bundle_on_renderer,
            cache_scene_runtime_on_renderer, cache_timeline_descriptor_on_renderer,
            resolve_and_cache_presentation_from_scene_cache_on_renderer,
            resolve_cached_presentation_model, sync_cached_presentation_model_slot,
            sync_cached_visibility_on_renderer, NativePanelCachedRendererBackend,
        };
        pub(crate) use super::super::runtime_backend::cache_runtime_scene_sync_result;
        pub(crate) use super::super::runtime_interaction::NativePanelSceneRuntimeBridge;
        pub(crate) use super::super::runtime_scene_cache::{
            build_native_panel_scene_for_state_bridge_with_input, cache_render_command_bundle,
            cache_render_command_bundle_for_state_bridge_with_input,
            cache_render_command_bundle_with_key, cache_scene_runtime_with_key,
            cached_runtime_render_state, cached_scene, native_panel_runtime_scene_cache_key,
            native_panel_runtime_scene_cache_key_for_state_bridge,
            resolve_and_cache_native_panel_presentation_for_state_bridge_with_input,
            resolve_current_native_panel_presentation_model_for_state_bridge_with_input,
            resolve_current_native_panel_render_command_bundle_for_state_bridge_with_input,
            resolve_native_panel_presentation_model_for_state_bridge_and_snapshot_with_input,
            resolve_native_panel_render_command_bundle_for_state_bridge_and_snapshot_with_input,
            resolve_native_panel_runtime_render_state_for_state_bridge_with_input,
            resolve_native_panel_scene_for_state_bridge_and_snapshot_with_input,
            resolve_native_panel_scene_for_state_bridge_with_input,
            resolve_native_panel_snapshot_render_plan_for_state_bridge_snapshot_with_input,
            NativePanelRuntimeSceneCache, NativePanelRuntimeSceneMutableStateBridge,
            NativePanelRuntimeSceneStateBridge,
        };
        pub(crate) use super::super::runtime_scene_sync::sync_runtime_scene_bundle_from_state_input;
        pub(crate) use super::super::traits::NativePanelRenderer;
    }

    // 运行时后端和渲染负载派发。
    pub(crate) mod runtime {
        #[cfg(feature = "tauri-host")]
        pub(crate) use super::super::host_runtime_facade::dispatch_native_panel_runtime_payload_with_handles;
        #[cfg(feature = "tauri-host")]
        pub(crate) use super::super::runtime_backend::{
            current_native_panel_runtime_backend,
            reposition_native_panel_to_selected_display_then_refresh,
            NativePanelPlatformRuntimeBackendFacade, NativePanelPlatformRuntimeFacadeApi,
            NativePanelRuntimeBackend,
        };
        pub(crate) use super::super::runtime_backend::{
            sync_runtime_scene_bundle_from_input_descriptor, NativePanelRuntimeSceneSyncResult,
        };
        #[cfg(feature = "tauri-host")]
        pub(crate) use super::super::runtime_render_payload::dispatch_native_panel_runtime_render_payload_if_available;
        pub(crate) use super::super::runtime_render_payload::{
            native_panel_runtime_render_payload_state_from_animation_plan,
            resolve_native_panel_runtime_render_payload_for_state,
            NativePanelRuntimeRenderPayloadState, NativePanelRuntimeRenderPayloadStateBridge,
        };
        pub(crate) use super::super::runtime_scene_sync::{
            rerender_runtime_scene_sync_result_to_host_for_runtime_with_input_descriptor,
            sync_runtime_scene_bundle_for_runtime_with_input,
            toggle_native_panel_settings_surface_and_rerender_for_runtime_with_input_descriptor,
        };
        pub(crate) use super::super::runtime_transition_slots::{
            apply_native_panel_hover_sync_result_for_runtime,
            apply_native_panel_runtime_scene_sync_result_for_runtime,
            apply_native_panel_settings_surface_toggle_result_for_runtime,
        };
    }

    // 平台线程、窗口消息泵和宿主 shell 生命周期。
    pub(crate) mod shell {
        #[cfg(feature = "tauri-host")]
        pub(crate) use super::super::platform_adapter::{
            dispatch_native_panel_on_platform_thread, NativePanelPlatformThreadAdapter,
        };
        pub(crate) use super::super::platform_adapter::{
            native_panel_has_raw_window_handle, sync_native_panel_raw_window_handle,
            NativePanelPlatformWindowHandleAdapter,
        };
        pub(crate) use super::super::shell_command::{
            apply_native_panel_host_shell_command, NativePanelHostShellCommand,
            NativePanelHostShellCommandBackend,
        };
        pub(crate) use super::super::shell_pump::{
            pump_native_panel_host_shell_runtime, NativePanelHostShellRuntimePump,
        };
        pub(crate) use super::super::shell_state::{
            NativePanelHostShellLifecycle, NativePanelHostShellState,
        };
        pub(crate) use super::super::window_message_pump::{
            pump_native_panel_platform_window_messages, NativePanelPlatformWindowMessage,
            NativePanelPlatformWindowMessagePump,
        };
    }

    // 过渡请求的排队、派发和完成清理。
    pub(crate) mod transition {
        pub(crate) use super::super::transition_controller::{
            clear_pending_native_panel_transition_request,
            dispatch_native_panel_transition_request_or_fallback_via,
            dispatch_native_panel_transition_request_with_snapshot_via,
            native_panel_transition_request_for_snapshot_sync,
            pending_native_panel_transition_if_active, resolve_native_panel_animation_timeline,
            take_pending_native_panel_transition_after_completed, NativePanelPendingTransition,
            NativePanelTransitionRequest,
        };
    }

    // 视觉计划和底层 primitive 工具。
    pub(crate) mod visual {
        pub(crate) use super::super::visual_plan::{
            resolve_native_panel_compact_bar_visual_plan, resolve_native_panel_visual_plan,
        };
        pub(crate) use super::super::visual_primitives::{
            native_panel_visual_compact_shoulder_primitive,
            native_panel_visual_completion_glow_primitive,
            native_panel_visual_mascot_body_primitive,
            native_panel_visual_mascot_ellipse_primitive,
            native_panel_visual_mascot_ellipse_primitives_by_role,
            native_panel_visual_mascot_round_rect_primitive,
            native_panel_visual_mascot_sprite_primitive, native_panel_visual_mascot_text_primitive,
            native_panel_visual_text_box_height, native_panel_visual_text_box_height_for_role,
            native_panel_visual_text_primitive_by_role, native_panel_visual_text_primitive_by_text,
            NativePanelVisualColor, NativePanelVisualMascotEllipseRole,
            NativePanelVisualMascotRoundRectRole, NativePanelVisualMascotTextRole,
            NativePanelVisualPlan, NativePanelVisualPrimitive, NativePanelVisualShoulderSide,
            NativePanelVisualTextAlignment, NativePanelVisualTextRole, NativePanelVisualTextWeight,
        };
    }

    #[cfg(test)]
    pub(crate) mod testing {
        pub(crate) use super::super::test_fixtures::{
            test_native_panel_runtime_input_descriptor, test_panel_scene, test_pending_permission,
            test_pending_question, test_preserved_status_close_scene, test_runtime_snapshot,
            test_runtime_snapshot_with_counts, test_session_snapshot,
        };
    }
}
