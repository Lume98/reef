use crate::state::PanelSettingsState;
use crate::panel::{
    scene::PanelInteractionProfile,
    ui::descriptor::{NativePanelRuntimeInputContext, NativePanelRuntimeInputDescriptor},
};

pub fn panel_scene_build_input_from_parts(
    display_options: Vec<crate::panel::scene::PanelDisplayOptionState>,
    settings: PanelSettingsState,
    app_version: String,
    update_status: crate::updater_service::AppUpdateStatus,
    interaction_profile: PanelInteractionProfile,
) -> crate::panel::scene::PanelSceneBuildInput {
    crate::panel::scene::PanelSceneBuildInput {
        display_options: sanitize_panel_display_options(display_options),
        settings,
        app_version,
        update_status,
        interaction_profile,
    }
}

pub fn native_panel_runtime_input_descriptor_from_parts(
    display_options: Vec<crate::panel::scene::PanelDisplayOptionState>,
    settings: PanelSettingsState,
    screen_frame: Option<crate::state::PanelRect>,
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

pub fn native_panel_runtime_input_descriptor_from_context(
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
    display_options: Vec<crate::panel::scene::PanelDisplayOptionState>,
) -> Vec<crate::panel::scene::PanelDisplayOptionState> {
    if display_options.is_empty() {
        vec![crate::panel::scene::fallback_panel_display_option()]
    } else {
        display_options
    }
}

#[cfg(test)]
mod tests {
    use super::{
        native_panel_runtime_input_descriptor_from_context,
        native_panel_runtime_input_descriptor_from_parts, panel_scene_build_input_from_parts,
    };
    use crate::{
        state::{PanelIslandWidthPreset, PanelLanguage, PanelRect, PanelSettingsState},
        panel::scene::{panel_display_option_state, PanelInteractionProfile},
        panel::ui::descriptor::NativePanelRuntimeInputContext,
        updater_service::AppUpdateStatus,
    };

    #[test]
    fn panel_scene_build_input_preserves_display_options_and_settings() {
        let input = panel_scene_build_input_from_parts(
            vec![panel_display_option_state(
                0,
                "display-1",
                "Built-in",
                3024,
                1964,
            )],
            PanelSettingsState {
                selected_display_index: 0,
                island_width_preset: PanelIslandWidthPreset::Wide,
                completion_sound_enabled: false,
                mascot_enabled: true,
                debug_mode_enabled: true,
                language: PanelLanguage::Zh,
            },
            "1.2.3".to_string(),
            AppUpdateStatus::idle(),
            PanelInteractionProfile::FullHost,
        );

        assert_eq!(input.display_options.len(), 1);
        assert_eq!(input.settings.selected_display_index, 0);
        assert_eq!(input.app_version, "1.2.3");
        assert!(!input.settings.completion_sound_enabled);
        assert!(input.settings.debug_mode_enabled);
    }

    #[test]
    fn panel_scene_build_input_falls_back_to_default_display_when_empty() {
        let input = panel_scene_build_input_from_parts(
            Vec::new(),
            PanelSettingsState::default(),
            "1.2.3".to_string(),
            AppUpdateStatus::idle(),
            PanelInteractionProfile::FullHost,
        );

        assert_eq!(input.display_options.len(), 1);
        assert_eq!(
            input.display_options[0],
            crate::panel::scene::fallback_panel_display_option()
        );
    }

    #[test]
    fn runtime_input_descriptor_reuses_context_screen_frame() {
        let descriptor = native_panel_runtime_input_descriptor_from_context(
            NativePanelRuntimeInputContext {
                display_options: vec![panel_display_option_state(
                    0,
                    "display-1",
                    "Built-in",
                    3024,
                    1964,
                )],
                selected_display_index: 0,
                screen_frame: Some(PanelRect {
                    x: 20.0,
                    y: 30.0,
                    width: 1280.0,
                    height: 720.0,
                }),
            },
            PanelSettingsState::default(),
            "1.2.3".to_string(),
            AppUpdateStatus::idle(),
            PanelInteractionProfile::FullHost,
        );

        assert_eq!(
            descriptor.screen_frame,
            Some(PanelRect {
                x: 20.0,
                y: 30.0,
                width: 1280.0,
                height: 720.0,
            })
        );
        assert_eq!(descriptor.scene_input.display_options.len(), 1);
    }

    #[test]
    fn runtime_input_descriptor_from_parts_keeps_scene_input_in_sync() {
        let descriptor = native_panel_runtime_input_descriptor_from_parts(
            vec![panel_display_option_state(
                0,
                "display-1",
                "Built-in",
                3024,
                1964,
            )],
            PanelSettingsState::default(),
            Some(PanelRect {
                x: 1.0,
                y: 2.0,
                width: 3.0,
                height: 4.0,
            }),
            "1.2.3".to_string(),
            AppUpdateStatus::idle(),
            PanelInteractionProfile::FullHost,
        );

        assert_eq!(descriptor.selected_display_index(), 0);
        assert_eq!(descriptor.scene_input.display_options.len(), 1);
    }
}
