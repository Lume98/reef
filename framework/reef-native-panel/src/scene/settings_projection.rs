use crate::state::{
    effective_island_width_preset_for_display, PanelIslandWidthPreset, PanelSettingsState,
};

use super::PanelDisplayOptionState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SettingsSurfaceProjection {
    pub selected_display_label: String,
    pub selected_display_supports_wide: bool,
    pub effective_width_preset: PanelIslandWidthPreset,
    pub has_display_options: bool,
}

pub fn resolve_settings_surface_projection(
    display_options: &[PanelDisplayOptionState],
    settings: PanelSettingsState,
) -> SettingsSurfaceProjection {
    let selected_display_position = display_options
        .iter()
        .position(|display| display.index == settings.selected_display_index)
        .or_else(|| {
            (settings.selected_display_index < display_options.len())
                .then_some(settings.selected_display_index)
        })
        .unwrap_or(0);
    let selected_display_label = if display_options.is_empty() {
        "0/0".to_string()
    } else {
        format!(
            "{}/{}",
            selected_display_position + 1,
            display_options.len()
        )
    };
    let selected_display_supports_wide = display_options
        .iter()
        .find(|display| display.index == settings.selected_display_index)
        .or_else(|| display_options.get(settings.selected_display_index))
        .or_else(|| display_options.first())
        .is_none_or(|display| display.supports_wide_island);
    let effective_width_preset = effective_island_width_preset_for_display(
        settings.island_width_preset,
        selected_display_supports_wide,
    );

    SettingsSurfaceProjection {
        selected_display_label,
        selected_display_supports_wide,
        effective_width_preset,
        has_display_options: !display_options.is_empty(),
    }
}
