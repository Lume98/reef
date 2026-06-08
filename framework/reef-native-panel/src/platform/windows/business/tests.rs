use super::{
    native_panel_runtime_input_context_from_display_options,
    native_panel_runtime_input_context_from_display_options_with_screen_frame,
    native_panel_runtime_input_descriptor_from_app_settings,
    native_panel_runtime_input_descriptor_from_context,
    native_panel_runtime_input_descriptor_from_display_options_with_screen_frame,
    panel_display_options_from_display_options, panel_scene_build_input_from_app_settings,
    panel_scene_build_input_from_display_options,
    resolve_next_display_selection_update_from_display_options,
    resolve_panel_selected_display_index, resolve_selected_display_index_from_display_options,
    NativePanelDisplaySelectionUpdate,
};
use crate::presentation::descriptor::NativePanelRuntimeInputContext;
use crate::{
    display_settings::DisplayOption,
    scene::panel_display_option_state,
    state::{PanelIslandWidthPreset, PanelLanguage, PanelRect},
};

use super::AppSettings;

fn settings() -> AppSettings {
    AppSettings::default()
}

#[test]
fn scene_build_input_clamps_display_count_and_projects_settings() {
    let input = panel_scene_build_input_from_app_settings(
        Vec::new(),
        2,
        &AppSettings {
            completion_sound_enabled: false,
            mascot_enabled: false,
            debug_mode_enabled: true,
            island_width_preset: PanelIslandWidthPreset::Standard,
            language: PanelLanguage::En,
            preferred_display_index: 7,
            preferred_display_key: Some("display-key".to_string()),
        },
    );

    assert_eq!(input.display_options.len(), 1);
    assert_eq!(input.display_options[0].label, "Display 1");
    assert_eq!(input.settings.selected_display_index, 2);
    assert_eq!(
        input.settings.island_width_preset,
        PanelIslandWidthPreset::Standard
    );
    assert!(!input.settings.completion_sound_enabled);
    assert!(!input.settings.mascot_enabled);
    assert!(input.settings.debug_mode_enabled);
    assert_eq!(input.app_version, env!("CARGO_PKG_VERSION"));
}

#[test]
fn runtime_input_descriptor_wraps_scene_input_and_screen_frame() {
    let descriptor = native_panel_runtime_input_descriptor_from_app_settings(
        vec![
            panel_display_option_state(0, "display-1", "Studio Display", 2560, 1440),
            panel_display_option_state(1, "display-2", "LG UltraFine", 1512, 982),
        ],
        1,
        &settings(),
        Some(PanelRect {
            x: 0.0,
            y: 0.0,
            width: 1440.0,
            height: 900.0,
        }),
    );

    assert_eq!(descriptor.selected_display_index(), 1);
    assert_eq!(descriptor.scene_input.display_options.len(), 2);
    assert_eq!(
        descriptor.screen_frame,
        Some(PanelRect {
            x: 0.0,
            y: 0.0,
            width: 1440.0,
            height: 900.0,
        })
    );
}

#[test]
fn runtime_input_descriptor_can_be_built_from_shared_context() {
    let descriptor = native_panel_runtime_input_descriptor_from_context(
        &settings(),
        NativePanelRuntimeInputContext {
            display_options: vec![
                panel_display_option_state(0, "display-1", "Built-in", 3024, 1964),
                panel_display_option_state(1, "display-2", "Studio Display", 2560, 1440),
                panel_display_option_state(2, "display-3", "LG", 1920, 1080),
            ],
            selected_display_index: 2,
            screen_frame: Some(PanelRect {
                x: 10.0,
                y: 20.0,
                width: 1280.0,
                height: 720.0,
            }),
        },
    );

    assert_eq!(descriptor.selected_display_index(), 2);
    assert_eq!(descriptor.scene_input.display_options.len(), 3);
    assert_eq!(
        descriptor.screen_frame,
        Some(PanelRect {
            x: 10.0,
            y: 20.0,
            width: 1280.0,
            height: 720.0,
        })
    );
}

#[test]
fn runtime_input_context_selects_display_from_shared_display_options() {
    let context = native_panel_runtime_input_context_from_display_options(
        vec![
            panel_display_option_state(0, "display-1", "Built-in", 3024, 1964),
            panel_display_option_state(1, "display-2", "Studio Display", 2560, 1440),
        ],
        &AppSettings {
            preferred_display_index: 9,
            preferred_display_key: Some("display-2".to_string()),
            ..settings()
        },
        Some(0),
        Some(PanelRect {
            x: 5.0,
            y: 10.0,
            width: 1280.0,
            height: 720.0,
        }),
    );

    assert_eq!(context.selected_display_index, 1);
    assert_eq!(context.display_options.len(), 2);
    assert_eq!(
        context.screen_frame,
        Some(PanelRect {
            x: 5.0,
            y: 10.0,
            width: 1280.0,
            height: 720.0,
        })
    );
}

#[test]
fn runtime_input_context_with_screen_frame_uses_selected_display_index() {
    let context = native_panel_runtime_input_context_from_display_options_with_screen_frame(
        vec![
            panel_display_option_state(0, "display-1", "Built-in", 3024, 1964),
            panel_display_option_state(1, "display-2", "Studio Display", 2560, 1440),
        ],
        &AppSettings {
            preferred_display_index: 9,
            preferred_display_key: Some("display-2".to_string()),
            ..settings()
        },
        Some(0),
        |selected_display_index| {
            Some(PanelRect {
                x: selected_display_index as f64,
                y: 10.0,
                width: 1280.0,
                height: 720.0,
            })
        },
    );

    assert_eq!(context.selected_display_index, 1);
    assert_eq!(
        context.screen_frame,
        Some(PanelRect {
            x: 1.0,
            y: 10.0,
            width: 1280.0,
            height: 720.0,
        })
    );
}

#[test]
fn scene_build_input_from_display_options_resolves_selected_display() {
    let displays = vec![
        DisplayOption {
            index: 0,
            key: "display-1".to_string(),
            name: "Built-in".to_string(),
            width: 3024,
            height: 1964,
            supports_wide_island: false,
        },
        DisplayOption {
            index: 1,
            key: "display-2".to_string(),
            name: "Studio Display".to_string(),
            width: 2560,
            height: 1440,
            supports_wide_island: true,
        },
    ];

    let input = panel_scene_build_input_from_display_options(
        &displays,
        &AppSettings {
            preferred_display_key: Some("display-2".to_string()),
            ..settings()
        },
        Some(0),
    );

    assert_eq!(input.settings.selected_display_index, 1);
    assert_eq!(input.display_options.len(), 2);
}

#[test]
fn runtime_input_descriptor_from_display_options_resolves_screen_frame() {
    let displays = vec![DisplayOption {
        index: 0,
        key: "display-1".to_string(),
        name: "Built-in".to_string(),
        width: 3024,
        height: 1964,
        supports_wide_island: false,
    }];

    let descriptor = native_panel_runtime_input_descriptor_from_display_options_with_screen_frame(
        &displays,
        &settings(),
        Some(0),
        |selected_display_index| {
            assert_eq!(selected_display_index, 0);
            Some(PanelRect {
                x: 1.0,
                y: 2.0,
                width: 300.0,
                height: 200.0,
            })
        },
    );

    assert_eq!(descriptor.selected_display_index(), 0);
    assert_eq!(
        descriptor.screen_frame,
        Some(PanelRect {
            x: 1.0,
            y: 2.0,
            width: 300.0,
            height: 200.0,
        })
    );
}

#[test]
fn display_options_preserve_name_and_resolution_format() {
    let options = panel_display_options_from_display_options(&[DisplayOption {
        index: 1,
        key: "display-2".to_string(),
        name: "Studio Display".to_string(),
        width: 2560,
        height: 1440,
        supports_wide_island: true,
    }]);

    assert_eq!(options[0].index, 1);
    assert_eq!(options[0].key, "display-2");
    assert_eq!(options[0].label, "Studio Display · 2560×1440");
}

#[test]
fn selected_display_index_prefers_key_then_index_then_fallback() {
    let settings = AppSettings {
        preferred_display_index: 7,
        preferred_display_key: Some("display-2".to_string()),
        ..settings()
    };
    let display_keys = vec!["display-1".to_string(), "display-2".to_string()];

    assert_eq!(
        resolve_panel_selected_display_index(&display_keys, &settings, Some(0)),
        1
    );
}

#[test]
fn selected_display_index_from_display_options_uses_key_and_fallback() {
    let settings = AppSettings {
        preferred_display_index: 7,
        preferred_display_key: Some("display-2".to_string()),
        ..settings()
    };
    let displays = vec![
        DisplayOption {
            index: 0,
            key: "display-1".to_string(),
            name: "Built-in".to_string(),
            width: 3024,
            height: 1964,
            supports_wide_island: false,
        },
        DisplayOption {
            index: 3,
            key: "display-2".to_string(),
            name: "Studio Display".to_string(),
            width: 2560,
            height: 1440,
            supports_wide_island: true,
        },
    ];

    assert_eq!(
        resolve_selected_display_index_from_display_options(&displays, &settings, Some(1)),
        1
    );
    assert_eq!(
        resolve_selected_display_index_from_display_options(&[], &settings, Some(4)),
        4
    );
}

#[test]
fn next_display_selection_update_cycles_and_skips_empty_display_lists() {
    let settings = AppSettings {
        preferred_display_index: 0,
        preferred_display_key: Some("display-1".to_string()),
        ..settings()
    };
    let displays = vec![
        DisplayOption {
            index: 0,
            key: "display-1".to_string(),
            name: "Built-in".to_string(),
            width: 3024,
            height: 1964,
            supports_wide_island: false,
        },
        DisplayOption {
            index: 1,
            key: "display-2".to_string(),
            name: "Studio Display".to_string(),
            width: 2560,
            height: 1440,
            supports_wide_island: true,
        },
    ];

    assert_eq!(
        resolve_next_display_selection_update_from_display_options(&displays, &settings),
        Some(NativePanelDisplaySelectionUpdate {
            selected_display_index: 1,
            selected_display_key: "display-2".to_string(),
        })
    );
    assert_eq!(
        resolve_next_display_selection_update_from_display_options(&[], &settings),
        None
    );
}
