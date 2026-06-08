use crate::presentation::descriptor::{
    NativePanelRuntimeInputContext, NativePanelRuntimeInputDescriptor,
};
use crate::{
    display_settings::DisplayOption,
    scene::{
        panel_display_option_state_with_width_support, PanelDisplayOptionState,
        PanelInteractionProfile,
    },
    state::{PanelRect, PanelSettingsState},
};

use super::{
    resolve_panel_selected_display_index, resolve_selected_display_index_from_display_options,
    AppSettings,
};

pub(crate) fn panel_settings_state_from_app_settings(
    selected_display_index: usize,
    settings: &AppSettings,
) -> PanelSettingsState {
    PanelSettingsState {
        selected_display_index,
        island_width_preset: settings.island_width_preset,
        completion_sound_enabled: settings.completion_sound_enabled,
        mascot_enabled: settings.mascot_enabled,
        debug_mode_enabled: settings.debug_mode_enabled,
        language: settings.language,
    }
}

pub(crate) fn panel_display_options_from_display_options(
    displays: &[DisplayOption],
) -> Vec<PanelDisplayOptionState> {
    displays
        .iter()
        .map(|display| {
            panel_display_option_state_with_width_support(
                display.index,
                display.key.clone(),
                &display.name,
                display.width,
                display.height,
                display.supports_wide_island,
            )
        })
        .collect()
}

pub(crate) fn panel_scene_build_input_from_app_settings(
    display_options: Vec<PanelDisplayOptionState>,
    selected_display_index: usize,
    settings: &AppSettings,
) -> crate::scene::PanelSceneBuildInput {
    panel_scene_build_input_from_parts(
        display_options,
        panel_settings_state_from_app_settings(selected_display_index, settings),
        env!("CARGO_PKG_VERSION").to_string(),
        crate::updater_service::current_update_status(),
        PanelInteractionProfile::Standalone,
    )
}

pub(crate) fn native_panel_runtime_input_descriptor_from_app_settings(
    display_options: Vec<PanelDisplayOptionState>,
    selected_display_index: usize,
    settings: &AppSettings,
    screen_frame: Option<PanelRect>,
) -> NativePanelRuntimeInputDescriptor {
    native_panel_runtime_input_descriptor_from_parts(
        display_options,
        panel_settings_state_from_app_settings(selected_display_index, settings),
        screen_frame,
        env!("CARGO_PKG_VERSION").to_string(),
        crate::updater_service::current_update_status(),
        PanelInteractionProfile::Standalone,
    )
}

pub(crate) fn native_panel_runtime_input_descriptor_from_context(
    settings: &AppSettings,
    context: NativePanelRuntimeInputContext,
) -> NativePanelRuntimeInputDescriptor {
    let selected_display_index = context.selected_display_index;
    native_panel_runtime_input_descriptor_from_context_parts(
        context,
        panel_settings_state_from_app_settings(selected_display_index, settings),
        env!("CARGO_PKG_VERSION").to_string(),
        crate::updater_service::current_update_status(),
        PanelInteractionProfile::Standalone,
    )
}

pub(crate) fn panel_scene_build_input_from_display_options(
    displays: &[DisplayOption],
    settings: &AppSettings,
    fallback_index: Option<usize>,
) -> crate::scene::PanelSceneBuildInput {
    let selected_display_index =
        resolve_selected_display_index_from_display_options(displays, settings, fallback_index);
    panel_scene_build_input_from_app_settings(
        panel_display_options_from_display_options(displays),
        selected_display_index,
        settings,
    )
}

pub(crate) fn native_panel_runtime_input_descriptor_from_display_options_with_screen_frame(
    displays: &[DisplayOption],
    settings: &AppSettings,
    fallback_index: Option<usize>,
    screen_frame_for_selected_index: impl FnOnce(usize) -> Option<PanelRect>,
) -> NativePanelRuntimeInputDescriptor {
    let context = native_panel_runtime_input_context_from_display_options_with_screen_frame(
        panel_display_options_from_display_options(displays),
        settings,
        fallback_index,
        screen_frame_for_selected_index,
    );
    native_panel_runtime_input_descriptor_from_context(settings, context)
}

fn panel_scene_build_input_from_parts(
    display_options: Vec<PanelDisplayOptionState>,
    settings: PanelSettingsState,
    app_version: String,
    update_status: crate::updater_service::AppUpdateStatus,
    interaction_profile: PanelInteractionProfile,
) -> crate::scene::PanelSceneBuildInput {
    crate::scene::PanelSceneBuildInput {
        display_options: sanitize_panel_display_options(display_options),
        settings,
        app_version,
        update_status,
        interaction_profile,
    }
}

fn native_panel_runtime_input_descriptor_from_parts(
    display_options: Vec<PanelDisplayOptionState>,
    settings: PanelSettingsState,
    screen_frame: Option<PanelRect>,
    app_version: String,
    update_status: crate::updater_service::AppUpdateStatus,
    interaction_profile: PanelInteractionProfile,
) -> NativePanelRuntimeInputDescriptor {
    NativePanelRuntimeInputDescriptor {
        scene_input: panel_scene_build_input_from_parts(
            display_options,
            settings,
            app_version,
            update_status,
            interaction_profile,
        ),
        screen_frame,
    }
}

fn native_panel_runtime_input_descriptor_from_context_parts(
    context: NativePanelRuntimeInputContext,
    settings: PanelSettingsState,
    app_version: String,
    update_status: crate::updater_service::AppUpdateStatus,
    interaction_profile: PanelInteractionProfile,
) -> NativePanelRuntimeInputDescriptor {
    native_panel_runtime_input_descriptor_from_parts(
        context.display_options,
        settings,
        context.screen_frame,
        app_version,
        update_status,
        interaction_profile,
    )
}

fn sanitize_panel_display_options(
    display_options: Vec<PanelDisplayOptionState>,
) -> Vec<PanelDisplayOptionState> {
    if display_options.is_empty() {
        vec![crate::scene::fallback_panel_display_option()]
    } else {
        display_options
    }
}

pub(crate) fn native_panel_runtime_input_context_from_display_options(
    display_options: Vec<PanelDisplayOptionState>,
    settings: &AppSettings,
    fallback_index: Option<usize>,
    screen_frame: Option<PanelRect>,
) -> NativePanelRuntimeInputContext {
    let selected_display_index = resolve_panel_selected_display_index(
        &display_options
            .iter()
            .map(|display| display.key.clone())
            .collect::<Vec<_>>(),
        settings,
        fallback_index,
    );

    NativePanelRuntimeInputContext {
        display_options,
        selected_display_index,
        screen_frame,
    }
}

pub(crate) fn native_panel_runtime_input_context_from_display_options_with_screen_frame(
    display_options: Vec<PanelDisplayOptionState>,
    settings: &AppSettings,
    fallback_index: Option<usize>,
    screen_frame_for_selected_index: impl FnOnce(usize) -> Option<PanelRect>,
) -> NativePanelRuntimeInputContext {
    let mut context = native_panel_runtime_input_context_from_display_options(
        display_options,
        settings,
        fallback_index,
        None,
    );
    context.screen_frame = screen_frame_for_selected_index(context.selected_display_index);
    context
}
