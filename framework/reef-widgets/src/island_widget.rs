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
        constraints.constrain(Size { width: self.width, height })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        if self.mode == DisplayMode::Hidden {
            return;
        }

        // ── Glow (behind everything) ─────────────────────────────────────
        if let Some(glow) = &self.glow {
            glow.paint(rect, ctx);
        }

        if self.mode == DisplayMode::Expanded {
            // ── Expanded shell ───────────────────────────────────────────
            let shell_alpha = 1.0 - self.chrome.collapsed_alpha;
            let mut shell = self.expanded_shell.clone();
            shell.alpha = shell_alpha;

            // Show separator based on chrome transition
            let sep_vis = self.chrome.separator_visibility.clamp(0.0, 1.0);
            if sep_vis > 0.0 {
                let bar_y = rect.height - self.compact_height;
                shell.separator_y = Some(bar_y);
                shell.separator_color = Color::rgba(40, 44, 54, (0.5 * sep_vis * 255.0) as u8);
            }
            shell.paint(rect, ctx);

            // ── Cards stacked in the expanded area ───────────────────────
            let bar_y = rect.y + rect.height - self.compact_height;
            let card_area = Rect {
                x: rect.x,
                y: rect.y + 8.0,
                width: rect.width,
                height: bar_y - rect.y - 8.0,
            };
            if !self.cards.is_empty() {
                let total = self.cards.len();
                let card_gap = 6.0;
                // Compute total height needed for all cards
                let total_card_height: f64 = self.cards.iter().map(|c| c.height).sum();
                let total_gap = card_gap * (total.saturating_sub(1)) as f64;
                let total_height = total_card_height + total_gap;
                // Clip to card area if overflowing
                let visible_height = card_area.height.min(total_height);
                let mut y = card_area.y + visible_height;

                for (i, card) in self.cards.iter().rev().enumerate() {
                    let phase = crate::animation::staggered_card_phase(
                        self.reveal_progress,
                        i,
                        total,
                        self.entering,
                    );
                    let vis = crate::animation::card_content_visibility(phase, self.entering);

                    // Shell reveal: scale interpolation
                    let (_shell_w, shell_h) = crate::animation::shell_reveal_frame(
                        1.0,
                        card.height,
                        card.collapsed_height,
                        phase,
                    );

                    y -= shell_h;
                    let card_rect = Rect {
                        x: card_area.x + 8.0,
                        y,
                        width: card_area.width - 16.0,
                        height: shell_h,
                    };

                    // Only paint if visible
                    if vis > 0.01 && card_rect.y + card_rect.height > card_area.y && card_rect.y < card_area.y + card_area.height {
                        ctx.primitives.push(VisualPrimitive::ClipStart { frame: card_rect });
                        let mut staged_card = card.clone();
                        staged_card.reveal_phase = phase;
                        staged_card.content_visibility = vis;
                        staged_card.content_translate_y = crate::animation::lerp(-5.0, 0.0, phase);
                        staged_card.paint(card_rect, ctx);
                        ctx.primitives.push(VisualPrimitive::ClipEnd);
                    }

                    y -= card_gap;
                }
            }
        }

        // ── Shoulders ────────────────────────────────────────────────────
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

        // ── Compact bar (always visible in both modes) ───────────────────
        ctx.primitives.push(VisualPrimitive::ClipStart { frame: bar_rect });
        let mut bar = self.compact_bar.clone();
        // In expanded mode, fade the bar's background
        if self.mode == DisplayMode::Expanded {
            bar.chrome = self.chrome;
        }
        bar.paint(bar_rect, ctx);
        ctx.primitives.push(VisualPrimitive::ClipEnd);

        // ── Mascot (on top of compact bar) ───────────────────────────────
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

impl Clone for IslandWidget {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode,
            compact_bar: self.compact_bar.clone(),
            expanded_shell: self.expanded_shell.clone(),
            cards: self.cards.clone(),
            mascot: self.mascot.clone(),
            glow: self.glow.clone(),
            shoulder_left: self.shoulder_left.clone(),
            shoulder_right: self.shoulder_right.clone(),
            chrome: self.chrome,
            reveal_progress: self.reveal_progress,
            entering: self.entering,
            width: self.width,
            compact_height: self.compact_height,
            expanded_height: self.expanded_height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{BodyLine, CardStyle};

    #[test]
    fn island_hidden_paints_nothing() {
        let island = IslandWidget::new();
        let rect = Rect { x: 0.0, y: 0.0, width: 400.0, height: 48.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        island.paint(rect, &mut ctx);
        assert!(primitives.is_empty());
    }

    #[test]
    fn island_compact_paints_bar() {
        let mut island = IslandWidget::new();
        island.mode = DisplayMode::Compact;
        let rect = Rect { x: 0.0, y: 0.0, width: 400.0, height: 48.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
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
        let rect = Rect { x: 0.0, y: 0.0, width: 400.0, height: 300.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        island.paint(rect, &mut ctx);
        assert!(primitives.len() > 8);
    }
}

impl Clone for ExpandedShell {
    fn clone(&self) -> Self {
        Self {
            fill_color: self.fill_color,
            border_color: self.border_color,
            separator_color: self.separator_color,
            radius: self.radius,
            separator_y: self.separator_y,
            alpha: self.alpha,
        }
    }
}
