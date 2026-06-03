use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    geometry::{Rect, Size},
};
use reef_layout::Constraints;

use crate::{
    card::Card,
    compact_bar::{ChromeVisibility, CompactBar, CompactShoulder, CompletionGlow},
    island::ExpandedShell,
    mascot::MascotWidget,
};

use super::{display_mode::DisplayMode, spec::IslandWidgetSpec};
use super::render::paint_island_widget;

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
        Self::from_spec(IslandWidgetSpec::default())
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

    pub fn from_spec(spec: IslandWidgetSpec) -> Self {
        spec.into()
    }
}

impl From<IslandWidgetSpec> for IslandWidget {
    fn from(spec: IslandWidgetSpec) -> Self {
        let IslandWidgetSpec {
            mode,
            layout,
            mut compact_bar,
            expanded_shell,
            cards,
            mascot,
            glow,
            shoulder_left,
            shoulder_right,
            chrome,
            reveal,
        } = spec;

        compact_bar.chrome = chrome;
        compact_bar.height = layout.compact_height;

        Self {
            mode,
            compact_bar,
            expanded_shell,
            cards,
            mascot,
            glow,
            shoulder_left,
            shoulder_right,
            chrome,
            reveal_progress: reveal.progress,
            entering: reveal.entering,
            width: layout.width,
            compact_height: layout.compact_height,
            expanded_height: layout.expanded_height,
        }
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
        paint_island_widget(self, rect, ctx);
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
