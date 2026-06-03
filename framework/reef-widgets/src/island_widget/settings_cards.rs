use crate::{Card, SettingsRow};

const DEFAULT_SETTINGS_SUBTITLE: &str = "v0.1.0";

pub(super) fn build_settings_cards() -> Vec<Card> {
    vec![Card::new(crate::CardStyle::Settings)
        .title("Settings")
        .subtitle(DEFAULT_SETTINGS_SUBTITLE)
        .settings_rows(default_settings_rows())
        .height(230.0)]
}

fn default_settings_rows() -> Vec<SettingsRow> {
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
