use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

/// Card style determines the color scheme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardStyle {
    Default,
    PendingApproval,
    PendingQuestion,
    PromptAssist,
    Completion,
    Settings,
    Empty,
}

impl CardStyle {
    pub fn colors(&self) -> (Color, Color, Color) {
        match self {
            CardStyle::Default => (Color::rgb(25, 28, 35), Color::rgb(44, 48, 58), Color::rgb(200, 210, 225)),
            CardStyle::PendingApproval => (Color::rgb(35, 30, 20), Color::rgb(60, 50, 30), Color::rgb(230, 200, 140)),
            CardStyle::PendingQuestion => (Color::rgb(30, 25, 40), Color::rgb(55, 45, 70), Color::rgb(190, 170, 230)),
            CardStyle::PromptAssist => (Color::rgb(30, 28, 25), Color::rgb(55, 50, 42), Color::rgb(220, 200, 170)),
            CardStyle::Completion => (Color::rgb(20, 35, 25), Color::rgb(35, 60, 40), Color::rgb(140, 230, 160)),
            CardStyle::Settings => (Color::rgb(25, 28, 35), Color::rgb(44, 48, 58), Color::rgb(200, 210, 225)),
            CardStyle::Empty => (Color::rgb(20, 22, 28), Color::rgb(35, 38, 45), Color::rgb(100, 105, 120)),
        }
    }
}

/// A body line with prefix and text.
#[derive(Clone, Debug)]
pub struct BodyLine {
    pub prefix: Option<String>,
    pub text: String,
}

/// A card widget showing title, badges, body lines, tool pill, and action hint.
pub struct Card {
    pub style: CardStyle,
    pub title: String,
    pub subtitle: Option<String>,
    pub status_badge: Option<String>,
    pub source_badge: Option<String>,
    pub body_lines: Vec<BodyLine>,
    pub tool_name: Option<String>,
    pub tool_description: Option<String>,
    pub action_hint: Option<String>,
    pub settings_rows: Vec<(String, String, bool)>,
    pub reveal_phase: f64,
    pub height: f64,
    pub radius: f64,
}

impl Card {
    pub fn new(style: CardStyle) -> Self {
        Self {
            style,
            title: String::new(),
            subtitle: None,
            status_badge: None,
            source_badge: None,
            body_lines: Vec::new(),
            tool_name: None,
            tool_description: None,
            action_hint: None,
            settings_rows: Vec::new(),
            reveal_phase: 1.0,
            height: 100.0,
            radius: 12.0,
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

    pub fn status_badge(mut self, text: impl Into<String>) -> Self {
        self.status_badge = Some(text.into());
        self
    }

    pub fn source_badge(mut self, text: impl Into<String>) -> Self {
        self.source_badge = Some(text.into());
        self
    }

    pub fn body_line(mut self, line: BodyLine) -> Self {
        self.body_lines.push(line);
        self
    }

    pub fn tool(mut self, name: impl Into<String>, desc: Option<String>) -> Self {
        self.tool_name = Some(name.into());
        self.tool_description = desc;
        self
    }

    pub fn action_hint(mut self, hint: impl Into<String>) -> Self {
        self.action_hint = Some(hint.into());
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }
}

impl Widget for Card {
    fn measure(&self, constraints: Constraints) -> Size {
        let width = constraints.max_width;
        constraints.constrain(Size { width, height: self.height })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        let (fill, border, text_color) = self.style.colors();

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

        let pad_x = 14.0;
        let mut y = rect.y + 12.0;

        // Title
        if !self.title.is_empty() {
            ctx.primitives.push(VisualPrimitive::Text {
                origin: reef_core::geometry::Point { x: rect.x + pad_x, y },
                max_width: rect.width - pad_x * 2.0,
                text: self.title.clone(),
                color: text_color,
                size: 14,
                weight: FontWeight::Semibold,
                alignment: TextAlignment::Left,
                alpha: self.reveal_phase,
            });
            y += 20.0;
        }

        // Subtitle
        if let Some(sub) = &self.subtitle {
            ctx.primitives.push(VisualPrimitive::Text {
                origin: reef_core::geometry::Point { x: rect.x + pad_x, y },
                max_width: rect.width - pad_x * 2.0,
                text: sub.clone(),
                color: Color::rgb(140, 150, 170),
                size: 12,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: self.reveal_phase,
            });
            y += 16.0;
        }

        // Badges
        if self.status_badge.is_some() || self.source_badge.is_some() {
            let mut badge_x = rect.x + pad_x;
            if let Some(badge_text) = &self.status_badge {
                let w = badge_text.chars().count() as f64 * 7.0 + 12.0;
                ctx.primitives.push(VisualPrimitive::RoundRect {
                    frame: Rect { x: badge_x, y, width: w, height: 18.0 },
                    radius: 6.0,
                    color: border,
                    alpha: self.reveal_phase,
                });
                ctx.primitives.push(VisualPrimitive::Text {
                    origin: reef_core::geometry::Point { x: badge_x + 6.0, y: y + 3.0 },
                    max_width: w,
                    text: badge_text.clone(),
                    color: text_color,
                    size: 11,
                    weight: FontWeight::Normal,
                    alignment: TextAlignment::Left,
                    alpha: self.reveal_phase,
                });
                badge_x += w + 6.0;
            }
            if let Some(badge_text) = &self.source_badge {
                let w = badge_text.chars().count() as f64 * 7.0 + 12.0;
                ctx.primitives.push(VisualPrimitive::RoundRect {
                    frame: Rect { x: badge_x, y, width: w, height: 18.0 },
                    radius: 6.0,
                    color: Color::rgb(35, 38, 48),
                    alpha: self.reveal_phase,
                });
                ctx.primitives.push(VisualPrimitive::Text {
                    origin: reef_core::geometry::Point { x: badge_x + 6.0, y: y + 3.0 },
                    max_width: w,
                    text: badge_text.clone(),
                    color: Color::rgb(140, 150, 170),
                    size: 11,
                    weight: FontWeight::Normal,
                    alignment: TextAlignment::Left,
                    alpha: self.reveal_phase,
                });
            }
            y += 24.0;
        }

        // Body lines
        for line in &self.body_lines {
            let mut x = rect.x + pad_x;
            if let Some(prefix) = &line.prefix {
                let pw = prefix.chars().count() as f64 * 7.0;
                ctx.primitives.push(VisualPrimitive::Text {
                    origin: reef_core::geometry::Point { x, y },
                    max_width: pw + 4.0,
                    text: prefix.clone(),
                    color: Color::rgb(100, 180, 140),
                    size: 12,
                    weight: FontWeight::Normal,
                    alignment: TextAlignment::Left,
                    alpha: self.reveal_phase,
                });
                x += pw + 4.0;
            }
            ctx.primitives.push(VisualPrimitive::Text {
                origin: reef_core::geometry::Point { x, y },
                max_width: rect.width - x + rect.x - pad_x,
                text: line.text.clone(),
                color: Color::rgb(160, 170, 190),
                size: 12,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: self.reveal_phase,
            });
            y += 16.0;
        }

        // Tool pill
        if let Some(name) = &self.tool_name {
            let pill_w = name.chars().count() as f64 * 7.0 + 16.0;
            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: Rect { x: rect.x + pad_x, y, width: pill_w, height: 22.0 },
                radius: 6.0,
                color: Color::rgb(45, 50, 65),
                alpha: self.reveal_phase,
            });
            ctx.primitives.push(VisualPrimitive::Text {
                origin: reef_core::geometry::Point { x: rect.x + pad_x + 8.0, y: y + 4.0 },
                max_width: pill_w,
                text: name.clone(),
                color: Color::rgb(180, 190, 210),
                size: 12,
                weight: FontWeight::Semibold,
                alignment: TextAlignment::Left,
                alpha: self.reveal_phase,
            });
            y += 26.0;
        }

        // Action hint
        if let Some(hint) = &self.action_hint {
            let hint_w = hint.chars().count() as f64 * 7.0 + 16.0;
            ctx.primitives.push(VisualPrimitive::RoundRect {
                frame: Rect { x: rect.x + pad_x, y, width: hint_w, height: 20.0 },
                radius: 6.0,
                color: Color::rgb(40, 43, 55),
                alpha: self.reveal_phase,
            });
            ctx.primitives.push(VisualPrimitive::Text {
                origin: reef_core::geometry::Point { x: rect.x + pad_x + 8.0, y: y + 3.0 },
                max_width: hint_w,
                text: hint.clone(),
                color: Color::rgb(130, 140, 160),
                size: 11,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: self.reveal_phase,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_paints_shell_and_title() {
        let card = Card::new(CardStyle::Default);
        let rect = Rect { x: 0.0, y: 0.0, width: 300.0, height: 100.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        card.paint(rect, &mut ctx);
        // At least shell (2 round rects)
        assert!(primitives.len() >= 2);
    }

    #[test]
    fn card_with_full_content() {
        let card = Card::new(CardStyle::PendingApproval)
            .title("Allow command?")
            .body_line(BodyLine { prefix: Some("$ ".into()), text: "rm -rf /".into() })
            .action_hint("Allow / Deny in terminal");
        let rect = Rect { x: 0.0, y: 0.0, width: 300.0, height: 120.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        card.paint(rect, &mut ctx);
        assert!(primitives.len() > 4);
    }
}
