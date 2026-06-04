use crate::prelude::SettingsRow;

pub(super) fn default_settings_rows() -> Vec<SettingsRow> {
    vec![
        SettingsRow {
            title: "Display".into(),
            value: "1".into(),
            active: true,
        },
        SettingsRow {
            title: "Width".into(),
            value: "Auto".into(),
            active: false,
        },
        SettingsRow {
            title: "Language".into(),
            value: "En".into(),
            active: false,
        },
        SettingsRow {
            title: "Sound".into(),
            value: "On".into(),
            active: true,
        },
        SettingsRow {
            title: "Mascot".into(),
            value: "On".into(),
            active: true,
        },
        SettingsRow {
            title: "Updates".into(),
            value: "Check".into(),
            active: false,
        },
    ]
}
