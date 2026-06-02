use reef_app::widget_host::{PaintContext, Widget};
use reef_core::geometry::{Rect, Size};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

use crate::{
    card::Card,
    compact_bar::CompactBar,
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
/// React-style: holds all child widgets as struct fields.
///
/// ```ignore
/// let mut island = IslandWidget::new();
/// island.update_from_input(...); // push new props
/// host.set_root(Box::new(island));
/// host.render(); // measure → paint
/// ```
pub struct IslandWidget {
    pub mode: DisplayMode,
    pub compact_bar: CompactBar,
    pub expanded_shell: ExpandedShell,
    pub cards: Vec<Card>,
    pub mascot: Option<MascotWidget>,
    pub glow: Option<CompletionGlow>,
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

        // Glow (behind everything)
        if let Some(glow) = &self.glow {
            glow.paint(rect, ctx);
        }

        if self.mode == DisplayMode::Expanded {
            // Expanded shell background
            self.expanded_shell.paint(rect, ctx);

            // Cards stacked in the expanded area
            let card_area = Rect {
                x: rect.x,
                y: rect.y + 8.0,
                width: rect.width,
                height: rect.height - 16.0,
            };
            if !self.cards.is_empty() {
                let card_height = 100.0;
                let gap = 6.0;
                let mut y = card_area.y + card_area.height;
                for card in self.cards.iter().rev() {
                    y -= card_height;
                    let card_rect = Rect {
                        x: card_area.x + 8.0,
                        y,
                        width: card_area.width - 16.0,
                        height: card_height,
                    };
                    ctx.primitives.push(VisualPrimitive::ClipStart { frame: card_rect });
                    card.paint(card_rect, ctx);
                    ctx.primitives.push(VisualPrimitive::ClipEnd);
                    y -= gap;
                }
            }
        }

        // Compact bar (always visible in both modes)
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
        ctx.primitives.push(VisualPrimitive::ClipStart { frame: bar_rect });
        self.compact_bar.paint(bar_rect, ctx);
        ctx.primitives.push(VisualPrimitive::ClipEnd);

        // Mascot (on top of compact bar)
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
        island.cards.push(
            Card::new(CardStyle::PendingApproval)
                .title("Allow?")
                .body_line(BodyLine { prefix: Some("$ ".into()), text: "rm -rf".into() })
                .height(100.0),
        );
        let rect = Rect { x: 0.0, y: 0.0, width: 400.0, height: 300.0 };
        let mut primitives = Vec::new();
        let mut ctx = PaintContext { primitives: &mut primitives };
        island.paint(rect, &mut ctx);
        assert!(primitives.len() > 8);
    }
}
