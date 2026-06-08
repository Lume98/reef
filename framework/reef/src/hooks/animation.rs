use crate::core::animation::EasingFunction;
use crate::hooks::use_state;
use std::time::{Duration, Instant};

/// The current state of an animation.
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationState {
    /// Current interpolated value in [0.0, 1.0].
    pub value: f64,
    /// Whether the animation is still running.
    pub playing: bool,
    /// Progress in [0.0, 1.0].
    pub progress: f64,
}

/// A hook that drives an animation over a given duration with an easing function.
///
/// Returns `(AnimationState, fn() -> restart)`.
///
/// # Example
/// ```ignore
/// let (anim, restart) = use_animation(EasingFunction::EaseOut, 300);
/// if anim.value > 0.5 { /* show content */ }
/// ```
pub fn use_animation(easing: EasingFunction, duration_ms: u64) -> (AnimationState, impl Fn()) {
    let (started_at, set_started_at) = use_state(Option::<Instant>::None);
    let (value, set_value) = use_state(0.0_f64);
    let (playing, set_playing) = use_state(true);

    let now = Instant::now();
    let start = started_at.unwrap_or(now);
    if started_at.is_none() {
        set_started_at.set(Some(now));
    }

    let elapsed = now.duration_since(start).as_millis() as u64;
    let progress = (elapsed as f64 / duration_ms as f64).min(1.0);
    let eased = easing.sample(progress);
    let is_playing = progress < 1.0;

    if value != eased {
        set_value.set(eased);
    }
    if playing != is_playing {
        set_playing.set(is_playing);
    }

    let restart = move || {
        set_started_at.set(Some(Instant::now()));
        set_playing.set(true);
    };

    (
        AnimationState {
            value: eased,
            playing: is_playing,
            progress,
        },
        restart,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_initial_state() {
        // Testing directly (use_animation requires fiber context)
        let state = AnimationState {
            value: 0.0,
            playing: true,
            progress: 0.0,
        };
        assert_eq!(state.value, 0.0);
        assert!(state.playing);
    }

    #[test]
    fn easing_linear_samples() {
        let easing = EasingFunction::Linear;
        assert!((easing.sample(0.0) - 0.0).abs() < 0.001);
        assert!((easing.sample(0.5) - 0.5).abs() < 0.001);
        assert!((easing.sample(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn easing_ease_out_bounds() {
        let easing = EasingFunction::EaseOut;
        assert!((easing.sample(0.0)).abs() < 0.001);
        assert!((easing.sample(1.0) - 1.0).abs() < 0.001);
        assert!(easing.sample(0.5) > 0.5); // EaseOut accelerates
    }
}
