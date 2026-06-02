use reef_app::widget_host::{PaintContext, Widget};
use reef_core::geometry::{Rect, Size};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

use crate::base::{card_content_visibility, lerp, shell_reveal_frame, staggered_card_phase};
use crate::card::Card;

/// Expanded-mode card stack renderer.
#[derive(Clone)]
pub struct ExpandedCardStack {
    pub cards: Vec<Card>,
    pub compact_height: f64,
    pub reveal_progress: f64,
    pub entering: bool,
    pub top_padding: f64,
    pub horizontal_padding: f64,
    pub card_gap: f64,
}

impl ExpandedCardStack {
    pub fn new(
        cards: Vec<Card>,
        compact_height: f64,
        reveal_progress: f64,
        entering: bool,
    ) -> Self {
        Self {
            cards,
            compact_height,
            reveal_progress,
            entering,
            top_padding: 8.0,
            horizontal_padding: 8.0,
            card_gap: 6.0,
        }
    }
}

impl Widget for ExpandedCardStack {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: constraints.max_height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        if self.cards.is_empty() {
            return;
        }

        let bar_y = rect.y + rect.height - self.compact_height;
        let card_area = Rect {
            x: rect.x,
            y: rect.y + self.top_padding,
            width: rect.width,
            height: bar_y - rect.y - self.top_padding,
        };

        let total = self.cards.len();
        let total_card_height: f64 = self.cards.iter().map(|c| c.height).sum();
        let total_gap = self.card_gap * (total.saturating_sub(1)) as f64;
        let total_height = total_card_height + total_gap;
        let visible_height = card_area.height.min(total_height);
        let mut y = card_area.y + visible_height;

        for (i, card) in self.cards.iter().rev().enumerate() {
            let phase = staggered_card_phase(self.reveal_progress, i, total, self.entering);
            let vis = card_content_visibility(phase, self.entering);
            let (_shell_w, shell_h) =
                shell_reveal_frame(1.0, card.height, card.collapsed_height, phase);

            y -= shell_h;
            let card_rect = Rect {
                x: card_area.x + self.horizontal_padding,
                y,
                width: card_area.width - self.horizontal_padding * 2.0,
                height: shell_h,
            };

            if vis > 0.01
                && card_rect.y + card_rect.height > card_area.y
                && card_rect.y < card_area.y + card_area.height
            {
                ctx.primitives
                    .push(VisualPrimitive::ClipStart { frame: card_rect });
                let mut staged_card = card.clone();
                staged_card.reveal_phase = phase;
                staged_card.content_visibility = vis;
                staged_card.content_translate_y = lerp(-5.0, 0.0, phase);
                staged_card.paint(card_rect, ctx);
                ctx.primitives.push(VisualPrimitive::ClipEnd);
            }

            y -= self.card_gap;
        }
    }
}
