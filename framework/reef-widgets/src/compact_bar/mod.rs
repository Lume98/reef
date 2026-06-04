use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_theme::compact_bar as theme;
use reef_view::widget_host::{PaintContext, Widget};

mod components;

use crate::mascot::MascotWidget;
pub use components::{
    CompactBarActions, CompactBarBackground, CompactBarCounts, CompactBarHeadline, CompactShoulder,
    CompletionGlow, ShoulderPath, ShoulderSide,
};

/// Chrome visibility spec for transitions between compact and expanded modes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChromeVisibility {
    pub separator_visibility: f64,
    pub shoulder_progress: f64,
    pub collapsed_alpha: f64,
    pub action_button_visibility: f64,
}

impl ChromeVisibility {
    pub fn compact() -> Self {
        Self {
            separator_visibility: 0.0,
            shoulder_progress: 0.0,
            collapsed_alpha: 1.0,
            action_button_visibility: 0.0,
        }
    }

    pub fn expanded() -> Self {
        Self {
            separator_visibility: 1.0,
            shoulder_progress: 1.0,
            collapsed_alpha: 0.0,
            action_button_visibility: 1.0,
        }
    }

    pub fn interpolate(from: &Self, to: &Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            separator_visibility: from.separator_visibility
                + (to.separator_visibility - from.separator_visibility) * t,
            shoulder_progress: from.shoulder_progress
                + (to.shoulder_progress - from.shoulder_progress) * t,
            collapsed_alpha: from.collapsed_alpha + (to.collapsed_alpha - from.collapsed_alpha) * t,
            action_button_visibility: from.action_button_visibility
                + (to.action_button_visibility - from.action_button_visibility) * t,
        }
    }
}

/// Collapsed mini-pill bar with shoulders, marquee, action buttons, and glow.
#[derive(Clone)]
pub struct CompactBar {
    pub headline: String,
    pub headline_emphasized: bool,
    pub active_count: String,
    pub active_count_next: Option<String>,
    pub active_count_scroll: f64,
    pub total_count: String,
    pub completion_count: usize,
    pub mascot: Option<MascotWidget>,
    pub show_actions: bool,
    pub debug_mode: bool,
    pub shoulder_left: Option<CompactShoulder>,
    pub shoulder_right: Option<CompactShoulder>,
    pub glow: Option<CompletionGlow>,
    pub chrome: ChromeVisibility,
    pub action_button_scale: f64,
    pub action_button_opacity: f64,
    pub action_button_offset_y: f64,
    pub fill_color: Color,
    pub border_color: Color,
    pub text_color: Color,
    pub dim_text_color: Color,
    pub radius: f64,
    pub height: f64,
}

impl CompactBar {
    pub fn new() -> Self {
        Self {
            headline: String::new(),
            headline_emphasized: false,
            active_count: String::new(),
            active_count_next: None,
            active_count_scroll: 0.0,
            total_count: String::new(),
            completion_count: 0,
            mascot: None,
            show_actions: false,
            debug_mode: false,
            shoulder_left: None,
            shoulder_right: None,
            glow: None,
            chrome: ChromeVisibility::compact(),
            action_button_scale: 0.82,
            action_button_opacity: 0.0,
            action_button_offset_y: -4.0,
            fill_color: Color::from(theme::FILL),
            border_color: Color::from(theme::BORDER),
            text_color: Color::from(theme::TEXT),
            dim_text_color: Color::from(theme::DIM_TEXT),
            radius: theme::RADIUS,
            height: theme::HEIGHT,
        }
    }

    pub fn headline(mut self, headline: impl Into<String>) -> Self {
        self.headline = headline.into();
        self
    }

    pub fn headline_emphasized(mut self, emphasized: bool) -> Self {
        self.headline_emphasized = emphasized;
        self
    }

    pub fn counts(
        mut self,
        active_count: impl Into<String>,
        total_count: impl Into<String>,
    ) -> Self {
        self.active_count = active_count.into();
        self.total_count = total_count.into();
        self
    }

    pub fn show_actions(mut self, show_actions: bool) -> Self {
        self.show_actions = show_actions;
        self
    }

    pub fn debug_mode(mut self, debug_mode: bool) -> Self {
        self.debug_mode = debug_mode;
        self
    }

    pub fn chrome(mut self, chrome: ChromeVisibility) -> Self {
        self.chrome = chrome;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }
}

impl Widget for CompactBar {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: self.height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        if let Some(glow) = &self.glow {
            glow.paint(rect, ctx);
        }

        CompactBarBackground {
            fill_color: self.fill_color,
            border_color: self.border_color,
            radius: self.radius,
        }
        .paint(rect, ctx);

        if let Some(shoulder) = &self.shoulder_left {
            shoulder.paint(rect, ctx);
        }
        if let Some(shoulder) = &self.shoulder_right {
            shoulder.paint(rect, ctx);
        }

        let action_reserves_space = self.show_actions && self.chrome.action_button_visibility > 0.0;
        let reserve_width = if action_reserves_space {
            theme::ACTION_BUTTON_RESERVE_WIDTH
        } else {
            0.0
        };

        CompactBarHeadline {
            text: self.headline.clone(),
            emphasized: self.headline_emphasized,
            text_color: self.text_color,
            leading_reserve: reserve_width,
            trailing_reserve: reserve_width,
        }
        .paint(rect, ctx);

        CompactBarCounts {
            active_count: self.active_count.clone(),
            active_count_next: self.active_count_next.clone(),
            active_count_scroll: self.active_count_scroll,
            total_count: self.total_count.clone(),
            text_color: self.text_color,
            dim_text_color: self.dim_text_color,
        }
        .paint(rect, ctx);

        CompactBarActions {
            show_actions: self.show_actions,
            debug_mode: self.debug_mode,
            visibility: self.chrome.action_button_visibility,
            base_scale: self.action_button_scale,
            base_opacity: self.action_button_opacity,
            base_offset_y: self.action_button_offset_y,
        }
        .paint(rect, ctx);

        if let Some(mascot) = &self.mascot {
            mascot.paint(rect, ctx);
        }
    }
}

impl Default for CompactBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::geometry::Rect;

    #[test]
    fn compact_bar_paints_background_and_text() {
        let bar = CompactBar::new();
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
        bar.paint(rect, &mut ctx);
        assert!(primitives.len() >= 2); // fill + border
    }

    #[test]
    fn compact_bar_with_headline_and_counts() {
        let mut bar = CompactBar::new();
        bar.headline = "Reef".to_string();
        bar.headline_emphasized = true;
        bar.active_count = "3".to_string();
        bar.total_count = "5".to_string();
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
        bar.paint(rect, &mut ctx);
        assert!(primitives.len() > 4);
    }

    #[test]
    fn compact_bar_marquee_renders_current_and_next() {
        let mut bar = CompactBar::new();
        bar.active_count = "3".to_string();
        bar.active_count_next = Some("4".to_string());
        bar.active_count_scroll = 0.5;
        bar.total_count = "5".to_string();
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
        bar.paint(rect, &mut ctx);
        assert!(primitives.len() > 6);
    }

    #[test]
    fn compact_bar_with_action_buttons() {
        let mut bar = CompactBar::new();
        bar.show_actions = true;
        bar.chrome = ChromeVisibility::expanded();
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
        bar.paint(rect, &mut ctx);
        // background (2) + settings icon + quit icon = 4
        assert!(primitives.len() >= 4);
    }

    #[test]
    fn chrome_visibility_interpolation() {
        let compact = ChromeVisibility::compact();
        let expanded = ChromeVisibility::expanded();
        let mid = ChromeVisibility::interpolate(&compact, &expanded, 0.5);
        assert!(mid.separator_visibility > 0.4 && mid.separator_visibility < 0.6);
        assert!(mid.shoulder_progress > 0.4 && mid.shoulder_progress < 0.6);
        assert!(mid.action_button_visibility > 0.4 && mid.action_button_visibility < 0.6);
    }
}
