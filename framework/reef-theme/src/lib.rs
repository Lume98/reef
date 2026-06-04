//! Shared visual tokens for Reef widgets and UI specs.
//!
//! This crate keeps the palette and a small set of layout constants in one place so
//! `reef-widgets` and `reef-ui` can stay aligned without duplicating literal values.

use reef_core::color::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub const fn rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb::new(r, g, b)
}

impl From<Rgb> for Color {
    fn from(value: Rgb) -> Self {
        Color::rgb(value.r, value.g, value.b)
    }
}

pub mod card {
    use super::Rgb;

    pub const SHELL_BORDER_DEFAULT: Rgb = Rgb::new(42, 42, 47);
    pub const SHELL_BORDER_PENDING: Rgb = Rgb::new(87, 61, 39);
    pub const SHELL_BORDER_PENDING_QUESTION: Rgb = Rgb::new(74, 62, 103);
    pub const SHELL_BORDER_COMPLETION: Rgb = Rgb::new(46, 79, 61);

    pub const SHELL_FILL_DEFAULT: Rgb = Rgb::new(37, 37, 41);
    pub const SHELL_FILL_PENDING: Rgb = Rgb::new(54, 41, 34);
    pub const SHELL_FILL_PENDING_QUESTION: Rgb = Rgb::new(45, 42, 57);
    pub const SHELL_FILL_PROMPT: Rgb = Rgb::new(48, 41, 35);
    pub const SHELL_FILL_COMPLETION: Rgb = Rgb::new(37, 37, 41);

    pub const ACCENT_DEFAULT: Rgb = Rgb::new(142, 150, 166);
    pub const ACCENT_SETTINGS: Rgb = Rgb::new(142, 166, 255);
    pub const ACCENT_PENDING: Rgb = Rgb::new(255, 184, 77);
    pub const ACCENT_PENDING_QUESTION: Rgb = Rgb::new(201, 176, 255);
    pub const ACCENT_COMPLETION: Rgb = Rgb::new(104, 213, 145);

    pub const TEXT_TITLE: Rgb = Rgb::new(245, 247, 252);
    pub const TEXT_TITLE_EMPTY: Rgb = Rgb::new(171, 179, 194);
    pub const TEXT_SUBTITLE: Rgb = Rgb::new(171, 179, 194);
    pub const TEXT_BODY: Rgb = Rgb::new(177, 183, 194);
    pub const TEXT_BODY_USER: Rgb = Rgb::new(218, 222, 229);
    pub const TEXT_BODY_EMPHASIZED: Rgb = Rgb::new(104, 222, 145);
    pub const TEXT_DETAIL: Rgb = Rgb::new(214, 218, 225);

    pub const PREFIX_DEFAULT_PROMPT: Rgb = Rgb::new(217, 120, 87);
    pub const PREFIX_DEFAULT_REPLY: Rgb = Rgb::new(104, 222, 145);
    pub const PREFIX_PENDING_QUESTION: Rgb = Rgb::new(201, 176, 255);

    pub const BADGE_BG_DEFAULT: Rgb = Rgb::new(54, 54, 58);
    pub const BADGE_FG_DEFAULT: Rgb = Rgb::new(230, 235, 245);
    pub const BADGE_BG_EMPHASIZED: Rgb = Rgb::new(58, 84, 65);
    pub const BADGE_FG_EMPHASIZED: Rgb = Rgb::new(102, 222, 145);
    pub const BADGE_BG_PENDING: Rgb = Rgb::new(70, 53, 36);
    pub const BADGE_BG_PENDING_QUESTION: Rgb = Rgb::new(61, 52, 83);
    pub const BADGE_FG_PENDING: Rgb = Rgb::new(255, 184, 77);
    pub const BADGE_FG_PENDING_QUESTION: Rgb = Rgb::new(201, 176, 255);

    pub const SOURCE_BG_DEFAULT: Rgb = Rgb::new(76, 45, 67);
    pub const SOURCE_BG_CLAUDE: Rgb = Rgb::new(84, 63, 42);
    pub const SOURCE_BG_CODEX: Rgb = Rgb::new(78, 91, 104);
    pub const SOURCE_BG_GEMINI: Rgb = Rgb::new(42, 68, 52);
    pub const SOURCE_BG_FEISHU: Rgb = Rgb::new(38, 55, 78);

    pub const SOURCE_FG_DEFAULT: Rgb = Rgb::new(255, 139, 214);
    pub const SOURCE_FG_CLAUDE: Rgb = Rgb::new(255, 199, 122);
    pub const SOURCE_FG_CODEX: Rgb = Rgb::new(218, 234, 246);
    pub const SOURCE_FG_GEMINI: Rgb = Rgb::new(118, 224, 142);
    pub const SOURCE_FG_FEISHU: Rgb = Rgb::new(126, 178, 255);

    pub const TOOL_TONE_DEFAULT: Rgb = Rgb::new(245, 247, 252);
    pub const TOOL_TONE_BASH: Rgb = Rgb::new(125, 242, 163);
    pub const TOOL_TONE_EDIT: Rgb = Rgb::new(135, 171, 255);
    pub const TOOL_TONE_READ: Rgb = Rgb::new(240, 209, 125);
    pub const TOOL_TONE_GREP: Rgb = Rgb::new(194, 161, 255);
    pub const TOOL_TONE_AGENT: Rgb = Rgb::new(255, 156, 102);

    pub const SETTINGS_ROW_BORDER_ACTIVE: Rgb = Rgb::new(50, 84, 61);
    pub const SETTINGS_ROW_BORDER_INACTIVE: Rgb = Rgb::new(50, 50, 56);
    pub const SETTINGS_ROW_FILL_ACTIVE: Rgb = Rgb::new(42, 50, 44);
    pub const SETTINGS_ROW_FILL_INACTIVE: Rgb = Rgb::new(43, 43, 48);
    pub const SETTINGS_VALUE_BG_ACTIVE: Rgb = Rgb::new(46, 68, 54);
    pub const SETTINGS_VALUE_BG_INACTIVE: Rgb = Rgb::new(54, 54, 58);
    pub const SETTINGS_VALUE_FG_ACTIVE: Rgb = Rgb::new(104, 222, 145);
    pub const SETTINGS_VALUE_FG_INACTIVE: Rgb = Rgb::new(230, 235, 245);

    pub const TOOL_PILL_BG: Rgb = Rgb::new(47, 47, 52);
    pub const TOOL_PILL_SHADOW: Rgb = Rgb::new(60, 60, 64);
    pub const ACTION_HINT_BG: Rgb = Rgb::new(49, 49, 53);
    pub const ACTION_HINT_FG: Rgb = Rgb::new(230, 235, 245);

    pub const HEADER_PAD_X: f64 = 14.0;
    pub const BADGE_WIDTH: f64 = 64.0;
    pub const BADGE_HEIGHT: f64 = 22.0;
    pub const BADGE_RADIUS: f64 = 11.0;
    pub const SETTINGS_ROW_RADIUS: f64 = 8.0;
    pub const SETTINGS_VALUE_BADGE_WIDTH: f64 = 44.0;
    pub const SETTINGS_VALUE_BADGE_HEIGHT: f64 = 18.0;
    pub const SETTINGS_VALUE_BADGE_RADIUS: f64 = 9.0;
    pub const CARD_RADIUS: f64 = 12.0;
    pub const CARD_HEIGHT_DEFAULT: f64 = 100.0;
    pub const CARD_COLLAPSED_HEIGHT_DEFAULT: f64 = 52.0;
    pub const CARD_COLLAPSED_HEIGHT_SETTINGS: f64 = 64.0;
}

pub mod compact_bar {
    use super::Rgb;

    pub const FILL: Rgb = Rgb::new(18, 20, 26);
    pub const BORDER: Rgb = Rgb::new(44, 48, 58);
    pub const TEXT: Rgb = Rgb::new(200, 210, 225);
    pub const DIM_TEXT: Rgb = Rgb::new(100, 108, 125);
    pub const ACTION_DEBUG: Rgb = Rgb::new(102, 222, 145);
    pub const ACTION_QUIT: Rgb = Rgb::new(255, 82, 82);

    pub const HEIGHT: f64 = 48.0;
    pub const RADIUS: f64 = 24.0;
    pub const ACTION_ICON_SIZE: f64 = 16.0;
    pub const ACTION_LEFT_INSET: f64 = 12.0;
    pub const ACTION_RIGHT_INSET: f64 = 28.0;
    pub const ACTION_BUTTON_RESERVE_WIDTH: f64 = 44.0;
    pub const HEADLINE_LEFT_INSET: f64 = 16.0;
    pub const HEADLINE_SIDE_RESERVE: f64 = 32.0;
    pub const COUNTS_RIGHT_INSET: f64 = 16.0;
}

pub mod panel {
    use super::Rgb;

    pub const SHELL_FILL: Rgb = Rgb::new(12, 12, 15);
    pub const SHELL_BORDER: Rgb = Rgb::new(44, 44, 50);
    pub const SHELL_SEPARATOR: Rgb = Rgb::new(62, 62, 70);
    pub const TEXT_PRIMARY: Rgb = Rgb::new(245, 247, 252);
    pub const TEXT_SECONDARY: Rgb = Rgb::new(230, 235, 245);
    pub const ACTION_SETTINGS_DEBUG: Rgb = Rgb::new(102, 222, 145);
    pub const ACTION_SETTINGS: Rgb = Rgb::new(245, 247, 252);
    pub const ACTION_QUIT: Rgb = Rgb::new(255, 82, 82);
}

pub mod shell {
    use super::Rgb;

    pub const FILL: Rgb = Rgb::new(18, 20, 26);
    pub const BORDER: Rgb = Rgb::new(44, 48, 58);
    pub const SEPARATOR: Rgb = Rgb::new(40, 44, 54);
    pub const RADIUS: f64 = 20.0;
}

pub mod mascot {
    use super::Rgb;

    pub const FILL: Rgb = Rgb::new(60, 65, 80);
    pub const STROKE: Rgb = Rgb::new(220, 160, 60);
    pub const EYE: Rgb = Rgb::new(220, 225, 240);
    pub const SHADOW: Rgb = Rgb::new(30, 30, 35);
    pub const MOUTH: Rgb = Rgb::new(255, 130, 100);
    pub const EYE_LID: Rgb = Rgb::new(160, 170, 190);
    pub const BUBBLE_FILL: Rgb = Rgb::new(47, 47, 52);
    pub const BUBBLE_DOT: Rgb = Rgb::new(140, 150, 170);
    pub const BADGE_FILL: Rgb = Rgb::new(37, 37, 41);
    pub const BADGE_OUTLINE: Rgb = Rgb::new(46, 79, 61);
    pub const BADGE_LABEL: Rgb = Rgb::new(102, 222, 145);
}

pub mod progress_bar {
    use super::Rgb;

    pub const TRACK: Rgb = Rgb::new(43, 43, 48);
    pub const FILL: Rgb = Rgb::new(104, 213, 145);
}
