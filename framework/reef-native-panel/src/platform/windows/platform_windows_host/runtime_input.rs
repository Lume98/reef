use crate::platform::windows::screen_geometry::{
    fallback_standalone_display_geometry, windows_standalone_screen_frame_with_scale,
};
use crate::presentation::descriptor::NativePanelRuntimeInputDescriptor;
use crate::{
    app_settings::{current_app_settings, AppSettings},
    display_settings::display_option_from_panel_geometry,
    panel_scene_input::native_panel_runtime_input_descriptor_from_display_options_with_screen_frame,
    scene::PanelInteractionProfile,
    state::{PanelDisplayGeometry, PanelRect},
};

use super::dpi::resolve_windows_system_dpi_scale;

pub(super) fn windows_platform_loop_runtime_input_descriptor(
    preferred_display_index: usize,
    screen_frame: Option<PanelRect>,
) -> NativePanelRuntimeInputDescriptor {
    let settings = current_app_settings();
    NativePanelRuntimeInputDescriptor {
        scene_input: crate::scene::PanelSceneBuildInput {
            display_options: vec![crate::scene::fallback_panel_display_option()],
            settings: panel_settings_state_from_app_settings(preferred_display_index, &settings),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            update_status: crate::updater_service::current_update_status(),
            interaction_profile: PanelInteractionProfile::Standalone,
        },
        screen_frame,
    }
}

fn panel_settings_state_from_app_settings(
    selected_display_index: usize,
    settings: &AppSettings,
) -> crate::state::PanelSettingsState {
    crate::state::PanelSettingsState {
        selected_display_index,
        island_width_preset: settings.island_width_preset,
        completion_sound_enabled: settings.completion_sound_enabled,
        mascot_enabled: settings.mascot_enabled,
        debug_mode_enabled: settings.debug_mode_enabled,
        language: settings.language,
    }
}

pub(super) fn windows_runtime_input_descriptor_without_app() -> NativePanelRuntimeInputDescriptor {
    let settings = current_app_settings();
    let display_geometry = windows_standalone_display_geometry();
    let screen_frame = windows_standalone_screen_frame(display_geometry);
    let display =
        display_option_from_panel_geometry(0, display_geometry, Some("Display 1".to_string()));
    native_panel_runtime_input_descriptor_from_display_options_with_screen_frame(
        &[display],
        &settings,
        Some(0),
        |_| Some(screen_frame),
    )
}

#[cfg(all(windows, not(test)))]
fn windows_standalone_display_geometry() -> PanelDisplayGeometry {
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
    if width <= 0 || height <= 0 {
        return fallback_standalone_display_geometry();
    }
    PanelDisplayGeometry {
        x: 0,
        y: 0,
        width: width as i64,
        height: height as i64,
    }
}

#[cfg(any(not(windows), test))]
fn windows_standalone_display_geometry() -> PanelDisplayGeometry {
    fallback_standalone_display_geometry()
}

fn windows_standalone_screen_frame(display_geometry: PanelDisplayGeometry) -> PanelRect {
    windows_standalone_screen_frame_with_scale(display_geometry, resolve_windows_system_dpi_scale())
}

#[cfg(test)]
mod tests {
    use super::windows_standalone_screen_frame_with_scale;
    use crate::presentation::descriptor::NativePanelRuntimeInputContext;
    use crate::{
        app_settings::AppSettings,
        panel_scene_input::native_panel_runtime_input_descriptor_from_context,
        platform_windows_host::dpi::WindowsDpiScale,
        state::{PanelDisplayGeometry, PanelRect},
    };

    #[test]
    fn standalone_screen_frame_uses_logical_size_from_dpi_scale() {
        let frame = windows_standalone_screen_frame_with_scale(
            PanelDisplayGeometry {
                x: 0,
                y: 0,
                width: 2400,
                height: 1350,
            },
            WindowsDpiScale::from_scale(1.5),
        );

        assert_eq!(
            frame,
            PanelRect {
                x: 0.0,
                y: 0.0,
                width: 1600.0,
                height: 900.0,
            }
        );
    }

    #[test]
    fn runtime_input_descriptor_projects_settings_and_display_context() {
        let descriptor = native_panel_runtime_input_descriptor_from_context(
            &AppSettings {
                completion_sound_enabled: false,
                mascot_enabled: false,
                debug_mode_enabled: false,
                island_width_preset: crate::state::PanelIslandWidthPreset::Standard,
                language: crate::state::PanelLanguage::En,
                preferred_display_index: 8,
                preferred_display_key: Some("display-key".to_string()),
            },
            NativePanelRuntimeInputContext {
                display_options: vec![
                    crate::scene::panel_display_option_state(
                        0,
                        "display-1",
                        "Built-in",
                        3024,
                        1964,
                    ),
                    crate::scene::panel_display_option_state(
                        1,
                        "display-2",
                        "Studio Display",
                        2560,
                        1440,
                    ),
                    crate::scene::panel_display_option_state(
                        2,
                        "display-3",
                        "Projector",
                        1920,
                        1080,
                    ),
                ],
                selected_display_index: 1,
                screen_frame: Some(PanelRect {
                    x: 20.0,
                    y: 30.0,
                    width: 1280.0,
                    height: 720.0,
                }),
            },
        );

        assert_eq!(descriptor.selected_display_index(), 1);
        assert_eq!(descriptor.scene_input.display_options.len(), 3);
        assert!(!descriptor.scene_input.settings.completion_sound_enabled);
        assert!(!descriptor.scene_input.settings.mascot_enabled);
        assert_eq!(
            descriptor.screen_frame,
            Some(PanelRect {
                x: 20.0,
                y: 30.0,
                width: 1280.0,
                height: 720.0,
            })
        );
    }

    #[test]
    fn runtime_input_descriptor_clamps_empty_display_list() {
        let descriptor = native_panel_runtime_input_descriptor_from_context(
            &AppSettings::default(),
            NativePanelRuntimeInputContext::default(),
        );

        assert_eq!(descriptor.scene_input.display_options.len(), 1);
        assert_eq!(descriptor.selected_display_index(), 0);
        assert_eq!(descriptor.screen_frame, None);
    }
}
