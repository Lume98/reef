use echoisland_runtime::RuntimeSnapshot;
use std::sync::atomic::{AtomicU8, Ordering};

use crate::{
    app_settings::{
        current_app_settings, update_completion_sound_enabled, update_debug_mode_enabled,
        update_island_width_preset, update_language, update_mascot_enabled,
    },
    native_panel_core::{next_island_width_preset_for_display, next_panel_language},
};

use super::runtime_interaction::NativePanelSettingsSurfaceSnapshotUpdate;
use super::transition_controller::NativePanelTransitionRequest;

const DEBUG_MODE_TRIGGER_CLICK_THRESHOLD: u8 = 10;
static DEBUG_MODE_TRIGGER_CLICK_COUNT: AtomicU8 = AtomicU8::new(0);

pub(crate) fn execute_native_panel_toggle_completion_sound_command(
    refresh: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let next_enabled = !current_app_settings().completion_sound_enabled;
    update_completion_sound_enabled(next_enabled).map_err(|error| error.to_string())?;
    refresh()
}

pub(crate) fn execute_native_panel_cycle_island_width_command(
    supports_wide: bool,
    refresh: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let next_preset = next_island_width_preset_for_display(
        current_app_settings().island_width_preset,
        supports_wide,
    );
    update_island_width_preset(next_preset).map_err(|error| error.to_string())?;
    refresh()
}

pub(crate) fn execute_native_panel_cycle_language_command(
    refresh: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let next_language = next_panel_language(current_app_settings().language);
    update_language(next_language).map_err(|error| error.to_string())?;
    refresh()
}

pub(crate) fn execute_native_panel_toggle_mascot_command(
    refresh: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let next_enabled = !current_app_settings().mascot_enabled;
    update_mascot_enabled(next_enabled).map_err(|error| error.to_string())?;
    refresh()
}

pub(crate) fn execute_native_panel_debug_mode_trigger_command(
    refresh: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    if current_app_settings().debug_mode_enabled {
        let next_count = DEBUG_MODE_TRIGGER_CLICK_COUNT
            .fetch_add(1, Ordering::SeqCst)
            .saturating_add(1);
        if next_count < DEBUG_MODE_TRIGGER_CLICK_THRESHOLD {
            return Ok(());
        }

        DEBUG_MODE_TRIGGER_CLICK_COUNT.store(0, Ordering::SeqCst);
        update_debug_mode_enabled(false).map_err(|error| error.to_string())?;
        return refresh();
    }

    let next_count = DEBUG_MODE_TRIGGER_CLICK_COUNT
        .fetch_add(1, Ordering::SeqCst)
        .saturating_add(1);

    if next_count < DEBUG_MODE_TRIGGER_CLICK_THRESHOLD {
        return Ok(());
    }

    DEBUG_MODE_TRIGGER_CLICK_COUNT.store(0, Ordering::SeqCst);
    update_debug_mode_enabled(true).map_err(|error| error.to_string())?;
    refresh()
}

pub(crate) fn execute_native_panel_settings_surface_command(
    sync_update: impl FnOnce() -> Result<Option<NativePanelSettingsSurfaceSnapshotUpdate>, String>,
    dispatch_update: impl FnOnce(
        Option<NativePanelTransitionRequest>,
        Option<RuntimeSnapshot>,
    ) -> Result<(), String>,
) -> Result<bool, String> {
    let Some(update) = sync_update()? else {
        return Ok(false);
    };
    dispatch_update(update.transition_request, update.snapshot)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    use super::*;
    use crate::{
        app_settings::{
            current_app_settings, update_completion_sound_enabled, update_debug_mode_enabled,
            update_island_width_preset, update_language, update_mascot_enabled,
        },
        native_panel_core::{PanelIslandWidthPreset, PanelLanguage},
    };

    #[test]
    fn toggle_completion_sound_command_updates_setting_and_refreshes() {
        update_completion_sound_enabled(false).expect("seed completion sound setting");
        let refreshes = Arc::new(AtomicUsize::new(0));
        let refreshes_clone = refreshes.clone();

        execute_native_panel_toggle_completion_sound_command(move || {
            refreshes_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .expect("toggle completion sound");

        assert!(current_app_settings().completion_sound_enabled);
        assert_eq!(refreshes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn cycle_island_width_command_advances_setting_and_refreshes() {
        update_island_width_preset(PanelIslandWidthPreset::Compact).expect("seed width setting");
        let refreshes = Arc::new(AtomicUsize::new(0));
        let refreshes_clone = refreshes.clone();

        execute_native_panel_cycle_island_width_command(true, move || {
            refreshes_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .expect("cycle island width");

        assert_eq!(
            current_app_settings().island_width_preset,
            PanelIslandWidthPreset::Standard
        );
        assert_eq!(refreshes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn cycle_language_command_advances_setting_and_refreshes() {
        update_language(PanelLanguage::Zh).expect("seed language setting");
        let refreshes = Arc::new(AtomicUsize::new(0));
        let refreshes_clone = refreshes.clone();

        execute_native_panel_cycle_language_command(move || {
            refreshes_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .expect("cycle language");

        assert_eq!(current_app_settings().language, PanelLanguage::Ja);
        assert_eq!(refreshes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn toggle_mascot_command_updates_setting_and_refreshes() {
        update_mascot_enabled(false).expect("seed mascot setting");
        let refreshes = Arc::new(AtomicUsize::new(0));
        let refreshes_clone = refreshes.clone();

        execute_native_panel_toggle_mascot_command(move || {
            refreshes_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .expect("toggle mascot");

        assert!(current_app_settings().mascot_enabled);
        assert_eq!(refreshes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn debug_mode_trigger_requires_threshold_before_refresh() {
        update_debug_mode_enabled(false).expect("seed debug mode setting");
        let refreshes = Arc::new(AtomicUsize::new(0));
        let refreshes_clone = refreshes.clone();

        for _ in 0..(DEBUG_MODE_TRIGGER_CLICK_THRESHOLD - 1) {
            execute_native_panel_debug_mode_trigger_command(|| Ok(()))
                .expect("increment debug click count");
        }

        execute_native_panel_debug_mode_trigger_command(move || {
            refreshes_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .expect("trigger debug mode");

        assert!(current_app_settings().debug_mode_enabled);
        assert_eq!(refreshes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn settings_surface_command_dispatches_transition_and_snapshot() {
        let snapshot = RuntimeSnapshot::idle();
        let mut dispatched = None;

        let changed = execute_native_panel_settings_surface_command(
            || {
                Ok(Some(NativePanelSettingsSurfaceSnapshotUpdate {
                    transition_request: None,
                    snapshot: Some(snapshot.clone()),
                }))
            },
            |transition, next_snapshot| {
                dispatched = Some((transition, next_snapshot));
                Ok(())
            },
        )
        .expect("dispatch settings surface update");

        assert!(changed);
        assert_eq!(dispatched, Some((None, Some(snapshot))));
    }
}
