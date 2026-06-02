use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

use crate::{
    card::Card,
    compact_bar::{ChromeVisibility, CompactBar},
    compact_shoulder::CompactShoulder,
    completion_glow::CompletionGlow,
    expanded_card_stack::ExpandedCardStack,
    expanded_shell::ExpandedShell,
    mascot::MascotWidget,
};

/// Top-level display mode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DisplayMode {
    Hidden,
    Compact,
    Expanded,
}

/// Top-level widget that composes the entire Dynamic Island UI.
#[derive(Clone)]
pub struct IslandWidget {
    pub mode: DisplayMode,
    pub compact_bar: CompactBar,
    pub expanded_shell: ExpandedShell,
    pub cards: Vec<Card>,
    pub mascot: Option<MascotWidget>,
    pub glow: Option<CompletionGlow>,
    pub shoulder_left: Option<CompactShoulder>,
    pub shoulder_right: Option<CompactShoulder>,
    pub chrome: ChromeVisibility,
    /// Card stack reveal animation progress (0..1)
    pub reveal_progress: f64,
    /// Whether cards are entering (true) or exiting (false)
    pub entering: bool,
    pub width: f64,
    pub compact_height: f64,
    pub expanded_height: f64,
}

impl IslandWidget {
    pub fn new() -> Self {
        Self {
            mode: DisplayMode::Hidden,
            compact_bar: CompactBar::new(),
            expanded_shell: ExpandedShell::new(),
            cards: Vec::new(),
            mascot: None,
            glow: None,
            shoulder_left: None,
            shoulder_right: None,
            chrome: ChromeVisibility::compact(),
            reveal_progress: 1.0,
            entering: true,
            width: 400.0,
            compact_height: 48.0,
            expanded_height: 300.0,
        }
    }

    pub fn width(mut self, w: f64) -> Self {
        self.width = w;
        self
    }

    pub fn compact_height(mut self, h: f64) -> Self {
        self.compact_height = h;
        self
    }

    pub fn expanded_height(mut self, h: f64) -> Self {
        self.expanded_height = h;
        self
    }
}

impl Widget for IslandWidget {
    fn measure(&self, constraints: Constraints) -> Size {
        let height = match self.mode {
            DisplayMode::Hidden => 0.0,
            DisplayMode::Compact => self.compact_height,
            DisplayMode::Expanded => self.expanded_height,
        };
        constraints.constrain(Size {
            width: self.width,
            height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        if self.mode == DisplayMode::Hidden {
            return;
        }

        if let Some(glow) = &self.glow {
            glow.paint(rect, ctx);
        }

        if self.mode == DisplayMode::Expanded {
            let shell_alpha = 1.0 - self.chrome.collapsed_alpha;
            let mut shell = self.expanded_shell.clone();
            shell.alpha = shell_alpha;

            let sep_vis = self.chrome.separator_visibility.clamp(0.0, 1.0);
            if sep_vis > 0.0 {
                let bar_y = rect.height - self.compact_height;
                shell.separator_y = Some(bar_y);
                shell.separator_color = Color::rgba(40, 44, 54, (0.5 * sep_vis * 255.0) as u8);
            }
            shell.paint(rect, ctx);

            ExpandedCardStack::new(
                self.cards.clone(),
                self.compact_height,
                self.reveal_progress,
                self.entering,
            )
            .paint(rect, ctx);
        }

        let bar_rect = if self.mode == DisplayMode::Compact {
            rect
        } else {
            Rect {
                x: rect.x,
                y: rect.y + rect.height - self.compact_height,
                width: rect.width,
                height: self.compact_height,
            }
        };

        if let Some(shoulder) = &self.shoulder_left {
            shoulder.paint(bar_rect, ctx);
        }
        if let Some(shoulder) = &self.shoulder_right {
            shoulder.paint(bar_rect, ctx);
        }

        ctx.primitives
            .push(VisualPrimitive::ClipStart { frame: bar_rect });
        let mut bar = self.compact_bar.clone();
        if self.mode == DisplayMode::Expanded {
            bar.chrome = self.chrome;
        }
        bar.paint(bar_rect, ctx);
        ctx.primitives.push(VisualPrimitive::ClipEnd);

        if let Some(mascot) = &self.mascot {
            mascot.paint(rect, ctx);
        }
    }
}

impl Default for IslandWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{BodyLine, CardStyle};

    #[test]
    fn island_hidden_paints_nothing() {
        let island = IslandWidget::new();
        let rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 400.0,
            height: 48.0,
        };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext {
            primitives: &mut primitives,
        };
        island.paint(rect, &mut ctx);
        assert!(primitives.is_empty());
    }

    #[test]
    fn island_compact_paints_bar() {
        let mut island = IslandWidget::new();
        island.mode = DisplayMode::Compact;
        let rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 400.0,
            height: 48.0,
        };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext {
            primitives: &mut primitives,
        };
        island.paint(rect, &mut ctx);
        // ClipStart + bar (2 round rects) + ClipEnd = at least 4
        assert!(primitives.len() >= 4);
    }

    #[test]
    fn island_expanded_with_cards() {
        let mut island = IslandWidget::new();
        island.mode = DisplayMode::Expanded;
        island.chrome = ChromeVisibility::expanded();
        island.cards.push(
            Card::new(CardStyle::PendingApproval)
                .title("Allow?")
                .body_line(BodyLine::plain(Some("$"), "rm -rf"))
                .height(100.0),
        );
        let rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 400.0,
            height: 300.0,
        };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext {
            primitives: &mut primitives,
        };
        island.paint(rect, &mut ctx);
        assert!(primitives.len() > 8);
    }
}
