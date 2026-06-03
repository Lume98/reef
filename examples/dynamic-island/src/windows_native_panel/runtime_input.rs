#[cfg(feature = "tauri-host")]
use tauri::AppHandle;

#[cfg(feature = "tauri-host")]
use crate::display_settings::{display_options_from_monitors, panel_rect_from_monitor};
use crate::{
    app_settings::current_app_settings,
    display_settings::display_option_from_panel_geometry,
    native_panel_core::{PanelDisplayGeometry, PanelRect},
    native_panel_scene_input::native_panel_runtime_input_descriptor_from_display_options_with_screen_frame,
};
use reef_native_panel_windows::screen_geometry::{
    fallback_standalone_display_geometry, windows_standalone_screen_frame_with_scale,
};
use reef_ui::native_panel_ui::descriptor::NativePanelRuntimeInputDescriptor;

use super::dpi::resolve_windows_system_dpi_scale;

#[cfg(feature = "tauri-host")]
pub(super) fn windows_runtime_input_descriptor<R: tauri::Runtime>(
    app: &AppHandle<R>,
) -> NativePanelRuntimeInputDescriptor {
    let settings = current_app_settings();
    let monitors = app.available_monitors().unwrap_or_default();
    let displays = display_options_from_monitors(&monitors);
    native_panel_runtime_input_descriptor_from_display_options_with_screen_frame(
        &displays,
        &settings,
        Some(0),
        |selected_display_index| {
            monitors
                .get(selected_display_index)
                .or_else(|| monitors.first())
                .map(panel_rect_from_monitor)
        },
    )
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
    use crate::{
        app_settings::AppSettings,
        native_panel_core::{PanelDisplayGeometry, PanelRect},
        native_panel_scene_input::native_panel_runtime_input_descriptor_from_context,
        windows_native_panel::dpi::WindowsDpiScale,
    };
    use reef_ui::native_panel_ui::descriptor::NativePanelRuntimeInputContext;

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
                island_width_preset: crate::native_panel_core::PanelIslandWidthPreset::Standard,
                language: crate::native_panel_core::PanelLanguage::En,
                preferred_display_index: 8,
                preferred_display_key: Some("display-key".to_string()),
            },
            NativePanelRuntimeInputContext {
                display_options: vec![
                    crate::native_panel_scene::panel_display_option_state(
                        0,
                        "display-1",
                        "Built-in",
                        3024,
                        1964,
                    ),
                    crate::native_panel_scene::panel_display_option_state(
                        1,
                        "display-2",
                        "Studio Display",
                        2560,
                        1440,
                    ),
                    crate::native_panel_scene::panel_display_option_state(
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
