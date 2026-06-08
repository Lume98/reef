use crate::prelude::{
    Card, ChromeVisibility, CompactBar, CompactShoulder, CompletionGlow, MascotWidget,
};
use crate::widgets::island::ExpandedShell;

use super::DisplayMode;

/// Top-level island layout parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IslandWidgetLayout {
    pub width: f64,
    pub compact_height: f64,
    pub expanded_height: f64,
}

impl IslandWidgetLayout {
    pub fn new(width: f64, compact_height: f64, expanded_height: f64) -> Self {
        Self {
            width,
            compact_height,
            expanded_height,
        }
    }
}

impl Default for IslandWidgetLayout {
    fn default() -> Self {
        Self {
            width: 400.0,
            compact_height: 48.0,
            expanded_height: 300.0,
        }
    }
}

/// Island reveal/transition parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IslandRevealSpec {
    /// Card stack reveal animation progress, clamped to 0..1 at paint time.
    pub progress: f64,
    /// Whether cards are entering (true) or exiting (false).
    pub entering: bool,
}

impl IslandRevealSpec {
    pub fn new(progress: f64, entering: bool) -> Self {
        Self { progress, entering }
    }
}

impl Default for IslandRevealSpec {
    fn default() -> Self {
        Self {
            progress: 1.0,
            entering: true,
        }
    }
}

/// Spec-driven top-level island composition.
///
/// This is intentionally a data-only shape so bridges can map runtime state into a reusable
/// widget description without having to know rendering details.
#[derive(Clone)]
pub struct IslandWidgetSpec {
    pub mode: DisplayMode,
    pub layout: IslandWidgetLayout,
    pub compact_bar: CompactBar,
    pub expanded_shell: ExpandedShell,
    pub cards: Vec<Card>,
    pub mascot: Option<MascotWidget>,
    pub glow: Option<CompletionGlow>,
    pub shoulder_left: Option<CompactShoulder>,
    pub shoulder_right: Option<CompactShoulder>,
    pub chrome: ChromeVisibility,
    pub reveal: IslandRevealSpec,
}

impl IslandWidgetSpec {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Render-time overrides applied after content composition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IslandRenderOverrides {
    pub width: f64,
    pub compact_height: f64,
    pub expanded_height: f64,
    pub chrome: crate::widgets::compact_bar::ChromeVisibility,
    pub reveal_progress: f64,
    pub entering: bool,
}

impl IslandRenderOverrides {
    pub fn new(
        width: f64,
        compact_height: f64,
        expanded_height: f64,
        chrome: crate::widgets::compact_bar::ChromeVisibility,
        reveal_progress: f64,
        entering: bool,
    ) -> Self {
        Self {
            width,
            compact_height,
            expanded_height,
            chrome,
            reveal_progress,
            entering,
        }
    }
}

impl Default for IslandWidgetSpec {
    fn default() -> Self {
        Self {
            mode: DisplayMode::Hidden,
            layout: IslandWidgetLayout::default(),
            compact_bar: CompactBar::new(),
            expanded_shell: ExpandedShell::new(),
            cards: Vec::new(),
            mascot: None,
            glow: None,
            shoulder_left: None,
            shoulder_right: None,
            chrome: ChromeVisibility::compact(),
            reveal: IslandRevealSpec::default(),
        }
    }
}
