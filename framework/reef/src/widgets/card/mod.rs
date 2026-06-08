use crate::core::{
    color::Color,
    geometry::{Rect, Size},
};
use crate::layout::Constraints;
use crate::theme::card as theme;
use crate::view::widget_host::{PaintContext, Widget};

// ── Card style ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardStyle {
    Default,
    Pending,
    PendingApproval,
    PendingQuestion,
    PromptAssist,
    Completion,
    Settings,
    Empty,
}

// ── Shell colors (matching reef card_visual_spec) ──────────────────────

fn shell_border_color(style: CardStyle) -> Color {
    match style {
        CardStyle::Completion => Color::from(theme::SHELL_BORDER_COMPLETION),
        CardStyle::Pending | CardStyle::PendingApproval | CardStyle::PromptAssist => {
            Color::from(theme::SHELL_BORDER_PENDING)
        }
        CardStyle::PendingQuestion => Color::from(theme::SHELL_BORDER_PENDING_QUESTION),
        CardStyle::Settings | CardStyle::Default | CardStyle::Empty => {
            Color::from(theme::SHELL_BORDER_DEFAULT)
        }
    }
}

fn shell_fill_color(style: CardStyle) -> Color {
    match style {
        CardStyle::Completion => Color::from(theme::SHELL_FILL_COMPLETION),
        CardStyle::Pending | CardStyle::PendingApproval => Color::from(theme::SHELL_FILL_PENDING),
        CardStyle::PendingQuestion => Color::from(theme::SHELL_FILL_PENDING_QUESTION),
        CardStyle::PromptAssist => Color::from(theme::SHELL_FILL_PROMPT),
        CardStyle::Settings | CardStyle::Default | CardStyle::Empty => {
            Color::from(theme::SHELL_FILL_DEFAULT)
        }
    }
}

fn accent_color(style: CardStyle) -> Color {
    match style {
        CardStyle::Pending | CardStyle::PendingApproval | CardStyle::PromptAssist => {
            Color::from(theme::ACCENT_PENDING)
        }
        CardStyle::PendingQuestion => Color::from(theme::ACCENT_PENDING_QUESTION),
        CardStyle::Completion => Color::from(theme::ACCENT_COMPLETION),
        CardStyle::Settings => Color::from(theme::ACCENT_SETTINGS),
        CardStyle::Default | CardStyle::Empty => Color::from(theme::ACCENT_DEFAULT),
    }
}

// ── Body role ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyRole {
    Assistant,
    User,
    Tool,
    Plain,
    ActionHint,
}

// ── Body line ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct BodyLine {
    pub role: BodyRole,
    pub prefix: Option<String>,
    pub text: String,
    pub max_lines: usize,
}

impl BodyLine {
    pub fn plain(prefix: Option<&str>, text: impl Into<String>) -> Self {
        Self {
            role: BodyRole::Plain,
            prefix: prefix.map(|s| s.to_string()),
            text: text.into(),
            max_lines: 2,
        }
    }
}

fn body_prefix_color(style: CardStyle, prefix: &str) -> Color {
    match (style, prefix) {
        (CardStyle::Default, "$") => Color::from(theme::PREFIX_DEFAULT_PROMPT),
        (CardStyle::Default, ">") | (CardStyle::Completion, _) => {
            Color::from(theme::PREFIX_DEFAULT_REPLY)
        }
        (CardStyle::PendingQuestion, _) | (CardStyle::Pending, "?") => {
            Color::from(theme::PREFIX_PENDING_QUESTION)
        }
        _ => accent_color(style),
    }
}

fn body_text_color(style: CardStyle, role: BodyRole, prefix: Option<&str>) -> Color {
    match (style, role) {
        (CardStyle::Default, BodyRole::User) => Color::from(theme::TEXT_BODY_USER),
        _ => match (style, prefix) {
            (CardStyle::Default, Some(">")) => Color::from(theme::TEXT_BODY_USER),
            _ => Color::from(theme::TEXT_BODY),
        },
    }
}

fn title_text_color(style: CardStyle) -> Color {
    if style == CardStyle::Empty {
        Color::from(theme::TEXT_TITLE_EMPTY)
    } else {
        Color::from(theme::TEXT_TITLE)
    }
}

// ── Badge ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeRole {
    Status,
    Source,
}

#[derive(Clone, Debug)]
pub struct Badge {
    pub role: BadgeRole,
    pub text: String,
    pub emphasized: bool,
}

impl Badge {
    pub fn status(text: impl Into<String>, emphasized: bool) -> Self {
        Self {
            role: BadgeRole::Status,
            text: text.into(),
            emphasized,
        }
    }

    pub fn source(text: impl Into<String>) -> Self {
        Self {
            role: BadgeRole::Source,
            text: text.into(),
            emphasized: false,
        }
    }
}

fn badge_background_color(style: CardStyle, badge: &Badge) -> Color {
    if badge.emphasized {
        return match (badge.role, style) {
            (
                BadgeRole::Status,
                CardStyle::Pending | CardStyle::PendingApproval | CardStyle::PromptAssist,
            ) => Color::from(theme::BADGE_BG_PENDING),
            (BadgeRole::Status, CardStyle::PendingQuestion) => {
                Color::from(theme::BADGE_BG_PENDING_QUESTION)
            }
            _ => Color::from(theme::BADGE_BG_EMPHASIZED),
        };
    }
    match badge.role {
        BadgeRole::Source => source_badge_bg(&badge.text),
        BadgeRole::Status => Color::from(theme::BADGE_BG_DEFAULT),
    }
}

fn badge_foreground_color(style: CardStyle, badge: &Badge) -> Color {
    if badge.emphasized {
        return match (badge.role, style) {
            (
                BadgeRole::Status,
                CardStyle::Pending | CardStyle::PendingApproval | CardStyle::PromptAssist,
            ) => Color::from(theme::BADGE_FG_PENDING),
            (BadgeRole::Status, CardStyle::PendingQuestion) => {
                Color::from(theme::BADGE_FG_PENDING_QUESTION)
            }
            _ => Color::from(theme::BADGE_FG_EMPHASIZED),
        };
    }
    match badge.role {
        BadgeRole::Source => source_badge_fg(&badge.text),
        BadgeRole::Status => Color::from(theme::BADGE_FG_DEFAULT),
    }
}

fn source_badge_bg(source: &str) -> Color {
    match source.trim().to_ascii_lowercase().as_str() {
        "claude" => Color::from(theme::SOURCE_BG_CLAUDE),
        "codex" => Color::from(theme::SOURCE_BG_CODEX),
        "gemini" => Color::from(theme::SOURCE_BG_GEMINI),
        "feishu" => Color::from(theme::SOURCE_BG_FEISHU),
        _ => Color::from(theme::SOURCE_BG_DEFAULT),
    }
}

fn source_badge_fg(source: &str) -> Color {
    match source.trim().to_ascii_lowercase().as_str() {
        "claude" => Color::from(theme::SOURCE_FG_CLAUDE),
        "codex" => Color::from(theme::SOURCE_FG_CODEX),
        "gemini" => Color::from(theme::SOURCE_FG_GEMINI),
        "feishu" => Color::from(theme::SOURCE_FG_FEISHU),
        _ => Color::from(theme::SOURCE_FG_DEFAULT),
    }
}

// ── Tool pill ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ToolPill {
    pub name: String,
    pub description: Option<String>,
}

fn tool_tone_color(tool: &str) -> Color {
    match tool.to_ascii_lowercase().as_str() {
        "bash" => Color::from(theme::TOOL_TONE_BASH),
        "edit" | "write" => Color::from(theme::TOOL_TONE_EDIT),
        "read" => Color::from(theme::TOOL_TONE_READ),
        "grep" | "glob" => Color::from(theme::TOOL_TONE_GREP),
        "agent" => Color::from(theme::TOOL_TONE_AGENT),
        _ => Color::from(theme::TOOL_TONE_DEFAULT),
    }
}

// ── Settings row ──────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SettingsRow {
    pub title: String,
    pub value: String,
    pub active: bool,
}

fn settings_row_border_color(active: bool) -> Color {
    if active {
        Color::from(theme::SETTINGS_ROW_BORDER_ACTIVE)
    } else {
        Color::from(theme::SETTINGS_ROW_BORDER_INACTIVE)
    }
}

fn settings_row_fill_color(active: bool) -> Color {
    if active {
        Color::from(theme::SETTINGS_ROW_FILL_ACTIVE)
    } else {
        Color::from(theme::SETTINGS_ROW_FILL_INACTIVE)
    }
}

fn settings_value_badge_bg(active: bool) -> Color {
    if active {
        Color::from(theme::SETTINGS_VALUE_BG_ACTIVE)
    } else {
        Color::from(theme::SETTINGS_VALUE_BG_INACTIVE)
    }
}

fn settings_value_badge_fg(active: bool) -> Color {
    if active {
        Color::from(theme::SETTINGS_VALUE_FG_ACTIVE)
    } else {
        Color::from(theme::SETTINGS_VALUE_FG_INACTIVE)
    }
}

mod components;

pub use components::{CardBadges, CardBody, CardHeader, CardSettingsPanel, CardShell};

// ── Card widget ───────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Card {
    pub style: CardStyle,
    pub title: String,
    pub subtitle: Option<String>,
    pub badges: Vec<Badge>,
    pub body_lines: Vec<BodyLine>,
    pub tool: Option<ToolPill>,
    pub action_hint: Option<String>,
    pub settings_rows: Vec<SettingsRow>,
    pub reveal_phase: f64,
    pub content_visibility: f64,
    pub content_translate_y: f64,
    pub height: f64,
    pub radius: f64,
    pub collapsed_height: f64,
    pub compact: bool,
}

impl Card {
    pub fn new(style: CardStyle) -> Self {
        Self {
            style,
            title: String::new(),
            subtitle: None,
            badges: Vec::new(),
            body_lines: Vec::new(),
            tool: None,
            action_hint: None,
            settings_rows: Vec::new(),
            reveal_phase: 1.0,
            content_visibility: 1.0,
            content_translate_y: 0.0,
            height: theme::CARD_HEIGHT_DEFAULT,
            radius: theme::CARD_RADIUS,
            collapsed_height: theme::CARD_COLLAPSED_HEIGHT_DEFAULT,
            compact: false,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn badge(mut self, badge: Badge) -> Self {
        self.badges.push(badge);
        self
    }

    pub fn body_line(mut self, line: BodyLine) -> Self {
        self.body_lines.push(line);
        self
    }

    pub fn tool(mut self, name: impl Into<String>, desc: Option<String>) -> Self {
        self.tool = Some(ToolPill {
            name: name.into(),
            description: desc,
        });
        self
    }

    pub fn action_hint(mut self, hint: impl Into<String>) -> Self {
        self.action_hint = Some(hint.into());
        self
    }

    pub fn settings_rows(mut self, rows: Vec<SettingsRow>) -> Self {
        self.settings_rows = rows;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }
}

impl Widget for Card {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: self.height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        CardShell {
            fill_color: shell_fill_color(self.style),
            border_color: shell_border_color(self.style),
            radius: self.radius,
            alpha: self.reveal_phase,
        }
        .paint(rect, ctx);

        if self.style == CardStyle::Settings && !self.settings_rows.is_empty() {
            CardSettingsPanel {
                title: self.title.clone(),
                subtitle: self.subtitle.clone(),
                settings_rows: self.settings_rows.clone(),
                content_translate_y: self.content_translate_y,
                content_alpha: (self.reveal_phase * self.content_visibility).min(1.0),
            }
            .paint(rect, ctx);
            return;
        }

        let content_alpha = (self.reveal_phase * self.content_visibility).min(1.0);

        CardHeader {
            title: self.title.clone(),
            subtitle: self.subtitle.clone(),
            title_color: title_text_color(self.style),
            compact: self.compact,
            content_translate_y: self.content_translate_y,
            content_alpha,
            pad_x: theme::HEADER_PAD_X,
        }
        .paint(rect, ctx);

        CardBadges {
            style: self.style,
            badges: self.badges.clone(),
            content_translate_y: self.content_translate_y,
            content_alpha,
            pad_x: theme::HEADER_PAD_X,
        }
        .paint(rect, ctx);

        CardBody {
            style: self.style,
            body_lines: self.body_lines.clone(),
            tool: self.tool.clone(),
            action_hint: self.action_hint.clone(),
            content_translate_y: self.content_translate_y,
            content_alpha,
            pad_x: theme::HEADER_PAD_X,
        }
        .paint(rect, ctx);
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new(CardStyle::Default)
    }
}

fn estimated_text_width(text: &str, font_size: f64) -> f64 {
    text.chars().count() as f64 * font_size * 0.58
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_paints_shell_and_title() {
        let card = Card::new(CardStyle::Default).title("Test");
        let rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 100.0,
        };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext {
            primitives: &mut primitives,
        };
        card.paint(rect, &mut ctx);
        assert!(primitives.len() >= 3); // shell (2 round rects) + title
    }

    #[test]
    fn card_with_full_content() {
        let card = Card::new(CardStyle::PendingApproval)
            .title("Allow command?")
            .badge(Badge::status("Approval", true))
            .badge(Badge::source("Claude"))
            .body_line(BodyLine::plain(Some("$"), "rm -rf /"))
            .tool("bash", Some("run command".to_string()))
            .action_hint("Allow / Deny in terminal");
        let rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 120.0,
        };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext {
            primitives: &mut primitives,
        };
        card.paint(rect, &mut ctx);
        assert!(primitives.len() > 6);
    }

    #[test]
    fn card_settings_rows() {
        let card = Card::new(CardStyle::Settings)
            .title("Settings")
            .subtitle("v1.0")
            .settings_rows(vec![
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
            ]);
        let rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 120.0,
        };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext {
            primitives: &mut primitives,
        };
        card.paint(rect, &mut ctx);
        assert!(primitives.len() > 6);
    }

    #[test]
    fn all_card_styles_have_colors() {
        let styles = [
            CardStyle::Default,
            CardStyle::Pending,
            CardStyle::PendingApproval,
            CardStyle::PendingQuestion,
            CardStyle::PromptAssist,
            CardStyle::Completion,
            CardStyle::Settings,
            CardStyle::Empty,
        ];
        for style in styles {
            let _border = shell_border_color(style);
            let _fill = shell_fill_color(style);
            let _accent = accent_color(style);
        }
    }

    #[test]
    fn tool_tone_colors() {
        assert_eq!(tool_tone_color("bash"), Color::from(theme::TOOL_TONE_BASH));
        assert_eq!(tool_tone_color("edit"), Color::from(theme::TOOL_TONE_EDIT));
        assert_eq!(tool_tone_color("grep"), Color::from(theme::TOOL_TONE_GREP));
        assert_eq!(
            tool_tone_color("agent"),
            Color::from(theme::TOOL_TONE_AGENT)
        );
    }
}
