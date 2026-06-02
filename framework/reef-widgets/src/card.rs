use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

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

// ── Shell colors (matching reef-ui card_visual_spec) ──────────────────────

fn shell_border_color(style: CardStyle) -> Color {
    match style {
        CardStyle::Completion => Color::rgb(46, 79, 61),
        CardStyle::Pending | CardStyle::PendingApproval | CardStyle::PromptAssist => {
            Color::rgb(87, 61, 39)
        }
        CardStyle::PendingQuestion => Color::rgb(74, 62, 103),
        CardStyle::Settings | CardStyle::Default | CardStyle::Empty => Color::rgb(42, 42, 47),
    }
}

fn shell_fill_color(style: CardStyle) -> Color {
    match style {
        CardStyle::Completion => Color::rgb(37, 37, 41),
        CardStyle::Pending | CardStyle::PendingApproval => Color::rgb(54, 41, 34),
        CardStyle::PendingQuestion => Color::rgb(45, 42, 57),
        CardStyle::PromptAssist => Color::rgb(48, 41, 35),
        CardStyle::Settings | CardStyle::Default | CardStyle::Empty => Color::rgb(37, 37, 41),
    }
}

fn accent_color(style: CardStyle) -> Color {
    match style {
        CardStyle::Pending | CardStyle::PendingApproval | CardStyle::PromptAssist => {
            Color::rgb(255, 184, 77)
        }
        CardStyle::PendingQuestion => Color::rgb(201, 176, 255),
        CardStyle::Completion => Color::rgb(104, 213, 145),
        CardStyle::Settings => Color::rgb(142, 166, 255),
        CardStyle::Default | CardStyle::Empty => Color::rgb(142, 150, 166),
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
        Self { role: BodyRole::Plain, prefix: prefix.map(|s| s.to_string()), text: text.into(), max_lines: 2 }
    }
}

fn body_prefix_color(style: CardStyle, prefix: &str) -> Color {
    match (style, prefix) {
        (CardStyle::Default, "$") => Color::rgb(217, 120, 87),
        (CardStyle::Default, ">") | (CardStyle::Completion, _) => Color::rgb(104, 222, 145),
        (CardStyle::PendingQuestion, _) | (CardStyle::Pending, "?") => Color::rgb(201, 176, 255),
        _ => accent_color(style),
    }
}

fn body_text_color(style: CardStyle, role: BodyRole, prefix: Option<&str>) -> Color {
    match (style, role) {
        (CardStyle::Default, BodyRole::User) => Color::rgb(218, 222, 229),
        _ => match (style, prefix) {
            (CardStyle::Default, Some(">")) => Color::rgb(218, 222, 229),
            _ => Color::rgb(177, 183, 194),
        },
    }
}

fn title_text_color(style: CardStyle) -> Color {
    if style == CardStyle::Empty {
        Color::rgb(171, 179, 194)
    } else {
        Color::rgb(245, 247, 252)
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
        Self { role: BadgeRole::Status, text: text.into(), emphasized }
    }

    pub fn source(text: impl Into<String>) -> Self {
        Self { role: BadgeRole::Source, text: text.into(), emphasized: false }
    }
}

fn badge_background_color(style: CardStyle, badge: &Badge) -> Color {
    if badge.emphasized {
        return match (badge.role, style) {
            (BadgeRole::Status, CardStyle::Pending | CardStyle::PendingApproval | CardStyle::PromptAssist) => {
                Color::rgb(70, 53, 36)
            }
            (BadgeRole::Status, CardStyle::PendingQuestion) => Color::rgb(61, 52, 83),
            _ => Color::rgb(58, 84, 65),
        };
    }
    match badge.role {
        BadgeRole::Source => source_badge_bg(&badge.text),
        BadgeRole::Status => Color::rgb(54, 54, 58),
    }
}

fn badge_foreground_color(style: CardStyle, badge: &Badge) -> Color {
    if badge.emphasized {
        return match (badge.role, style) {
            (BadgeRole::Status, CardStyle::Pending | CardStyle::PendingApproval | CardStyle::PromptAssist) => {
                Color::rgb(255, 184, 77)
            }
            (BadgeRole::Status, CardStyle::PendingQuestion) => Color::rgb(201, 176, 255),
            _ => Color::rgb(102, 222, 145),
        };
    }
    match badge.role {
        BadgeRole::Source => source_badge_fg(&badge.text),
        BadgeRole::Status => Color::rgb(230, 235, 245),
    }
}

fn source_badge_bg(source: &str) -> Color {
    match source.trim().to_ascii_lowercase().as_str() {
        "claude" => Color::rgb(84, 63, 42),
        "codex" => Color::rgb(78, 91, 104),
        "gemini" => Color::rgb(42, 68, 52),
        "feishu" => Color::rgb(38, 55, 78),
        _ => Color::rgb(76, 45, 67),
    }
}

fn source_badge_fg(source: &str) -> Color {
    match source.trim().to_ascii_lowercase().as_str() {
        "claude" => Color::rgb(255, 199, 122),
        "codex" => Color::rgb(218, 234, 246),
        "gemini" => Color::rgb(118, 224, 142),
        "feishu" => Color::rgb(126, 178, 255),
        _ => Color::rgb(255, 139, 214),
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
        "bash" => Color::rgb(125, 242, 163),
        "edit" | "write" => Color::rgb(135, 171, 255),
        "read" => Color::rgb(240, 209, 125),
        "grep" | "glob" => Color::rgb(194, 161, 255),
        "agent" => Color::rgb(255, 156, 102),
        _ => Color::rgb(245, 247, 252),
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
    if active { Color::rgb(50, 84, 61) } else { Color::rgb(50, 50, 56) }
}

fn settings_row_fill_color(active: bool) -> Color {
    if active { Color::rgb(42, 50, 44) } else { Color::rgb(43, 43, 48) }
}

fn settings_value_badge_bg(active: bool) -> Color {
    if active { Color::rgb(46, 68, 54) } else { Color::rgb(54, 54, 58) }
}

fn settings_value_badge_fg(active: bool) -> Color {
    if active { Color::rgb(104, 222, 145) } else { Color::rgb(230, 235, 245) }
}

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
            height: 100.0,
            radius: 12.0,
            collapsed_height: 52.0,
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
        self.tool = Some(ToolPill { name: name.into(), description: desc });
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
        constraints.constrain(Size { width: constraints.max_width, height: self.height })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        let fill = shell_fill_color(self.style);
        let border = shell_border_color(self.style);
        let text_color = title_text_color(self.style);

        // Card shell
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: rect,
            radius: self.radius,
            color: fill,
            alpha: self.reveal_phase,
        });
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: rect,
            radius: self.radius,
            color: border,
            alpha: 0.4 * self.reveal_phase,
        });

        // ── Settings rows (Settings style only) ──────────────────────────
        if self.style == CardStyle::Settings && !self.settings_rows.is_empty() {
            self.paint_settings_rows(rect, ctx);
            return;
        }

        let pad_x = 14.0;
        let content_alpha = (self.reveal_phase * self.content_visibility).min(1.0);

        // ── Title ─────────────────────────────────────────────────────────
        if !self.title.is_empty() {
            let base_y = if self.compact {
                rect.y + (rect.height - 20.0) / 2.0
            } else {
                rect.y + rect.height - 24.0
            };
            let y = base_y + self.content_translate_y;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: rect.x + pad_x, y },
                max_width: rect.width - pad_x * 2.0,
                text: self.title.clone(),
                color: text_color,
                size: 12,
                weight: FontWeight::Semibold,
                alignment: TextAlignment::Left,
                alpha: content_alpha,
            });
        }

        // ── Subtitle ──────────────────────────────────────────────────────
        if let Some(sub) = &self.subtitle {
            let y = rect.y + rect.height - 40.0 + self.content_translate_y;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: rect.x + pad_x, y },
                max_width: rect.width - pad_x * 2.0,
                text: sub.clone(),
                color: Color::rgb(171, 179, 194),
                size: 9,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: content_alpha,
            });
        }

        // ── Badges ────────────────────────────────────────────────────────
        if !self.badges.is_empty() {
            let title_y = rect.y + rect.height - 24.0 + self.content_translate_y;
            let mut right = rect.x + rect.width - pad_x;
            for badge in self.badges.iter().rev() {
                let bg = badge_background_color(self.style, badge);
                let fg = badge_foreground_color(self.style, badge);
                let w = 64.0;
                let bx = right - w;
                let by = title_y - 3.0;
                ctx.primitives.push(VisualPrimitive::RoundRect {
                    frame: Rect { x: bx, y: by, width: w, height: 22.0 },
                    radius: 11.0,
                    color: bg,
                    alpha: content_alpha,
                });
                ctx.primitives.push(VisualPrimitive::Text {
                    origin: Point { x: bx + 7.0, y: by + 2.0 },
                    max_width: w - 14.0,
                    text: badge.text.clone(),
                    color: fg,
                    size: 10,
                    weight: FontWeight::Normal,
                    alignment: TextAlignment::Center,
                    alpha: content_alpha,
                });
                right = bx - 6.0;
            }
        }

        // ── Body lines ────────────────────────────────────────────────────
        let action_hint_present = self.action_hint.is_some();
        let body_bottom = if action_hint_present { 36.0 } else { 10.0 };
        let mut y = rect.y + body_bottom + self.content_translate_y;

        for line in self.body_lines.iter().rev() {
            let (prefix_color, text_color) = {
                let p = line.prefix.as_deref().unwrap_or_default();
                (body_prefix_color(self.style, p), body_text_color(self.style, line.role, line.prefix.as_deref()))
            };
            let mut x = rect.x + pad_x;
            if let Some(prefix) = &line.prefix {
                let pw = prefix.chars().count() as f64 * 6.0;
                ctx.primitives.push(VisualPrimitive::Text {
                    origin: Point { x, y },
                    max_width: pw + 4.0,
                    text: prefix.clone(),
                    color: prefix_color,
                    size: 10,
                    weight: FontWeight::Normal,
                    alignment: TextAlignment::Left,
                    alpha: content_alpha,
                });
                x += 24.0; // chat_prefix_width
            }
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x, y },
                max_width: rect.width - x + rect.x - pad_x,
                text: line.text.clone(),
                color: text_color,
                size: 10,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: content_alpha,
            });
            y += 16.0;
        }

        // ── Tool pill ─────────────────────────────────────────────────────
        if let Some(tool) = &self.tool {
            let name_w = estimated_text_width(&tool.name, 9.0);
            let desc_w = tool.description.as_ref()
                .filter(|d| !d.trim().is_empty())
                .map(|d| estimated_text_width(d, 9.0) + 6.0)
                .unwrap_or(0.0);
            let pill_w = (name_w + desc_w + 14.0).max(36.0);
            let pill_h = 22.0;
            let pill_radius = 5.0;
            let px = rect.x + pad_x;
            let py = y;

            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: Rect { x: px, y: py, width: pill_w, height: pill_h },
                radius: pill_radius,
                color: Color::rgb(47, 47, 52),
                alpha: content_alpha,
            });
            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: Rect { x: px, y: py, width: pill_w, height: pill_h },
                radius: pill_radius,
                color: Color::rgb(60, 60, 64),
                alpha: 0.4 * content_alpha,
            });
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: px + 7.0, y: py + 5.0 },
                max_width: name_w,
                text: tool.name.clone(),
                color: tool_tone_color(&tool.name),
                size: 9,
                weight: FontWeight::Semibold,
                alignment: TextAlignment::Left,
                alpha: content_alpha,
            });
            if let Some(desc) = &tool.description {
                if !desc.trim().is_empty() {
                    let desc_x = px + 7.0 + name_w + 6.0;
                    ctx.primitives.push(VisualPrimitive::Text {
                        origin: Point { x: desc_x, y: py + 5.0 },
                        max_width: desc_w,
                        text: desc.clone(),
                        color: Color::rgb(214, 218, 225),
                        size: 9,
                        weight: FontWeight::Normal,
                        alignment: TextAlignment::Left,
                        alpha: content_alpha,
                    });
                }
            }
        }

        // ── Action hint ──────────────────────────────────────────────────
        if let Some(hint) = &self.action_hint {
            let hint_text = hint.split_whitespace().collect::<Vec<_>>().join(" ");
            if !hint_text.is_empty() {
                let hint_w = (estimated_text_width(&hint_text, 10.0) + 18.0).max(32.0);
                let hint_h = 18.0; // pending_action_height from reef-ui metrics
                let hint_radius = hint_h / 2.0;
                let hx = rect.x + pad_x;
                let hy = rect.y + 10.0; // pending_action_y

                ctx.primitives.push(VisualPrimitive::RoundRect {
                    frame: Rect { x: hx, y: hy, width: hint_w, height: hint_h },
                    radius: hint_radius,
                    color: Color::rgb(49, 49, 53),
                    alpha: content_alpha,
                });
                ctx.primitives.push(VisualPrimitive::Text {
                    origin: Point { x: hx + 9.0, y: hy + 4.0 },
                    max_width: hint_w - 18.0,
                    text: hint_text,
                    color: Color::rgb(230, 235, 245),
                    size: 10,
                    weight: FontWeight::Normal,
                    alignment: TextAlignment::Left,
                    alpha: content_alpha,
                });
            }
        }
    }
}

impl Card {
    fn paint_settings_rows(&self, rect: Rect, ctx: &mut PaintContext) {
        let content_alpha = (self.reveal_phase * self.content_visibility).min(1.0);

        // Title in settings position
        if !self.title.is_empty() {
            let y = rect.y + rect.height - 24.0 + self.content_translate_y;
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: rect.x + 14.0, y },
                max_width: rect.width - 28.0,
                text: self.title.clone(),
                color: Color::rgb(245, 247, 252),
                size: 12,
                weight: FontWeight::Semibold,
                alignment: TextAlignment::Left,
                alpha: content_alpha,
            });
        }

        // Version badge (subtitle as badge)
        if let Some(version) = &self.subtitle {
            let title_y = rect.y + rect.height - 24.0 + self.content_translate_y;
            let w = 64.0;
            let bx = rect.x + rect.width - 14.0 - w;
            let by = title_y - 3.0;
            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: Rect { x: bx, y: by, width: w, height: 22.0 },
                radius: 11.0,
                color: Color::rgb(54, 54, 58),
                alpha: content_alpha,
            });
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: bx + 7.0, y: by + 2.0 },
                max_width: w - 14.0,
                text: version.clone(),
                color: Color::rgb(230, 235, 245),
                size: 10,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Center,
                alpha: content_alpha,
            });
        }

        // Settings rows
        let row_h = 32.0;
        let row_gap = 2.0;
        let pad_x = 14.0;
        for (i, row) in self.settings_rows.iter().enumerate() {
            let ry = rect.y + 8.0 + (self.settings_rows.len() - 1 - i) as f64 * (row_h + row_gap);
            let row_frame = Rect { x: rect.x + pad_x, y: ry, width: rect.width - pad_x * 2.0, height: row_h };

            // Row background
            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: row_frame,
                radius: 8.0,
                color: settings_row_border_color(row.active),
                alpha: content_alpha,
            });
            let inner = Rect { x: row_frame.x + 1.0, y: row_frame.y + 1.0, width: row_frame.width - 2.0, height: row_frame.height - 2.0 };
            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: inner,
                radius: 7.0,
                color: settings_row_fill_color(row.active),
                alpha: content_alpha,
            });

            // Row title
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: inner.x + 11.0, y: inner.y + (inner.height - 16.0) / 2.0 },
                max_width: inner.width - 70.0,
                text: row.title.clone(),
                color: Color::rgb(245, 247, 252),
                size: 11,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: content_alpha,
            });

            // Value badge
            let badge_w = 44.0;
            let badge_h = 18.0;
            let badge_x = inner.x + inner.width - badge_w - 9.0;
            let badge_y = inner.y + (inner.height - badge_h) / 2.0;
            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: Rect { x: badge_x, y: badge_y, width: badge_w, height: badge_h },
                radius: 9.0,
                color: settings_value_badge_bg(row.active),
                alpha: content_alpha,
            });
            ctx.primitives.push(VisualPrimitive::Text {
                origin: Point { x: badge_x + 9.0, y: badge_y + 2.0 },
                max_width: badge_w - 18.0,
                text: row.value.clone(),
                color: settings_value_badge_fg(row.active),
                size: 10,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Center,
                alpha: content_alpha,
            });
        }
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
        let rect = Rect { x: 0.0, y: 0.0, width: 300.0, height: 100.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
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
        let rect = Rect { x: 0.0, y: 0.0, width: 300.0, height: 120.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        card.paint(rect, &mut ctx);
        assert!(primitives.len() > 6);
    }

    #[test]
    fn card_settings_rows() {
        let card = Card::new(CardStyle::Settings)
            .title("Settings")
            .subtitle("v1.0")
            .settings_rows(vec![
                SettingsRow { title: "Display".into(), value: "1".into(), active: true },
                SettingsRow { title: "Width".into(), value: "Auto".into(), active: false },
            ]);
        let rect = Rect { x: 0.0, y: 0.0, width: 300.0, height: 120.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
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
        assert_eq!(tool_tone_color("bash"), Color::rgb(125, 242, 163));
        assert_eq!(tool_tone_color("edit"), Color::rgb(135, 171, 255));
        assert_eq!(tool_tone_color("grep"), Color::rgb(194, 161, 255));
        assert_eq!(tool_tone_color("agent"), Color::rgb(255, 156, 102));
    }
}
