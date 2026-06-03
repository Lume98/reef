use crate::{display_settings::DisplayOption, native_panel_core::resolve_preferred_panel_display_index};

use super::AppSettings;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NativePanelDisplaySelectionUpdate {
    pub(crate) selected_display_index: usize,
    pub(crate) selected_display_key: String,
}

pub(crate) fn resolve_panel_selected_display_index(
    display_keys: &[String],
    settings: &AppSettings,
    fallback_index: Option<usize>,
) -> usize {
    resolve_preferred_panel_display_index(
        display_keys,
        settings.preferred_display_key.as_deref(),
        settings.preferred_display_index,
        fallback_index,
    )
    .unwrap_or(0)
}

pub(crate) fn resolve_selected_display_index_from_display_options(
    displays: &[DisplayOption],
    settings: &AppSettings,
    fallback_index: Option<usize>,
) -> usize {
    if displays.is_empty() {
        return fallback_index.unwrap_or(settings.preferred_display_index);
    }
    resolve_panel_selected_display_index(
        &displays
            .iter()
            .map(|display| display.key.clone())
            .collect::<Vec<_>>(),
        settings,
        fallback_index,
    )
}

pub(crate) fn resolve_next_display_selection_update_from_display_options(
    displays: &[DisplayOption],
    settings: &AppSettings,
) -> Option<NativePanelDisplaySelectionUpdate> {
    if displays.is_empty() {
        return None;
    }
    let current = resolve_selected_display_index_from_display_options(displays, settings, Some(0));
    let selected = displays.get((current + 1) % displays.len())?;
    Some(NativePanelDisplaySelectionUpdate {
        selected_display_index: selected.index,
        selected_display_key: selected.key.clone(),
    })
}

