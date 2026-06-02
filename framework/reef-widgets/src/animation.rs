use std::time::Instant;

/// Easing functions.
pub fn ease_out_cubic(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

pub fn ease_in_cubic(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    t.powi(3)
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// A keyframe-driven animation target with start and end values.
#[derive(Clone, Copy, Debug)]
pub struct AnimatedValue {
    pub from: f64,
    pub to: f64,
    pub duration_ms: u64,
    pub started_at: Option<Instant>,
}

impl AnimatedValue {
    pub fn new(from: f64, to: f64, duration_ms: u64) -> Self {
        Self { from, to, duration_ms, started_at: None }
    }

    pub fn start(&mut self) {
        self.started_at = Some(Instant::now());
    }

    pub fn value(&self) -> f64 {
        match self.started_at {
            None => self.from,
            Some(start) => {
                let elapsed_ms = start.elapsed().as_millis() as u64;
                let t = (elapsed_ms as f64 / self.duration_ms as f64).min(1.0);
                lerp(self.from, self.to, t)
            }
        }
    }

    pub fn ease_out_value(&self) -> f64 {
        match self.started_at {
            None => self.from,
            Some(start) => {
                let elapsed_ms = start.elapsed().as_millis() as u64;
                let t = (elapsed_ms as f64 / self.duration_ms as f64).min(1.0);
                lerp(self.from, self.to, ease_out_cubic(t))
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        match self.started_at {
            None => false,
            Some(start) => start.elapsed().as_millis() as u64 >= self.duration_ms,
        }
    }
}

/// Animation target for the island widget's chrome transition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationTarget {
    Open,
    Close,
    SurfaceSwitch,
}

/// Card stack reveal constants (matching reef-ui).
pub const PANEL_CARD_REVEAL_MS: u64 = 280;
pub const PANEL_CARD_REVEAL_STAGGER_MS: u64 = 60;
pub const PANEL_CARD_EXIT_MS: u64 = 200;
pub const PANEL_CARD_EXIT_STAGGER_MS: u64 = 40;
pub const PANEL_CARD_CONTENT_REVEAL_DELAY_PROGRESS: f64 = 0.5;
pub const PANEL_CARD_CONTENT_EARLY_EXIT_PROGRESS: f64 = 0.35;

/// Compute the staggered phase for a card in a stack.
pub fn staggered_card_phase(progress: f64, index: usize, total: usize, entering: bool) -> f64 {
    let progress = progress.clamp(0.0, 1.0);
    let duration_ms = if entering { PANEL_CARD_REVEAL_MS } else { PANEL_CARD_EXIT_MS };
    let stagger_ms = if entering { PANEL_CARD_REVEAL_STAGGER_MS } else { PANEL_CARD_EXIT_STAGGER_MS };
    let total_ms = duration_ms + stagger_ms * (total.saturating_sub(1)) as u64;
    let order_index = if entering { index } else { total.saturating_sub(index + 1) };
    let elapsed_ms = progress * total_ms as f64;
    let delay_ms = order_index as f64 * stagger_ms as f64;
    ((elapsed_ms - delay_ms) / duration_ms as f64).clamp(0.0, 1.0)
}

/// Compute content visibility phase for a card reveal.
pub fn card_content_visibility(phase: f64, entering: bool) -> f64 {
    let phase = phase.clamp(0.0, 1.0);
    if entering {
        let delay = PANEL_CARD_CONTENT_REVEAL_DELAY_PROGRESS;
        ease_out_cubic(((phase - delay) / (1.0 - delay)).clamp(0.0, 1.0))
    } else if phase <= PANEL_CARD_CONTENT_EARLY_EXIT_PROGRESS {
        let exit = PANEL_CARD_CONTENT_EARLY_EXIT_PROGRESS;
        1.0 - (0.06 * (phase / exit).clamp(0.0, 1.0))
    } else {
        let exit = PANEL_CARD_CONTENT_EARLY_EXIT_PROGRESS;
        0.94 * (1.0 - ease_in_cubic(((phase - exit) / (1.0 - exit)).clamp(0.0, 1.0)))
    }
}

/// Compute compact shoulder progress from island width.
pub fn shoulder_progress_from_width(
    width: f64,
    compact_width: f64,
    expanded_width: f64,
) -> f64 {
    ((width - compact_width) / (expanded_width - compact_width)).clamp(0.0, 1.0)
}

/// Compute the collapsed alpha (fade of compact-mode-only elements) during transition.
pub fn collapsed_alpha(chrome_visibility: f64) -> f64 {
    (1.0 - chrome_visibility).clamp(0.0, 1.0)
}

/// Compute shell reveal frame (scale interpolation during expand).
pub fn shell_reveal_frame(
    expanded_frame_width: f64,
    expanded_frame_height: f64,
    collapsed_height: f64,
    phase: f64,
) -> (f64, f64) {
    let progress = ease_out_cubic(phase.clamp(0.0, 1.0));
    let width = lerp(expanded_frame_width * 0.96, expanded_frame_width, progress);
    let height = lerp(collapsed_height, expanded_frame_height, progress);
    (width, height)
}
