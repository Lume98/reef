use crate::Card;

use super::settings_rows::default_settings_rows;

const DEFAULT_SETTINGS_SUBTITLE: &str = "v0.1.0";

pub(super) fn build_settings_cards() -> Vec<Card> {
    vec![Card::new(crate::CardStyle::Settings)
        .title("Settings")
        .subtitle(DEFAULT_SETTINGS_SUBTITLE)
        .settings_rows(default_settings_rows())
        .height(230.0)]
}
