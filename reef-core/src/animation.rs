use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Keyframe {
    pub time: f64,
    pub value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Spring,
}

impl EasingFunction {
    pub fn sample(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t).powi(3),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            EasingFunction::Spring => {
                let c4 = (2.0 * std::f64::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationCurve {
    pub easing: EasingFunction,
    pub keyframes: Vec<Keyframe>,
}

impl AnimationCurve {
    pub fn sample(&self, progress: f64) -> f64 {
        if self.keyframes.is_empty() {
            return self.easing.sample(progress);
        }
        if self.keyframes.len() == 1 {
            return self.keyframes[0].value;
        }
        let t = self.easing.sample(progress);
        let idx = self
            .keyframes
            .iter()
            .position(|k| k.time >= t)
            .unwrap_or(self.keyframes.len() - 1);
        if idx == 0 {
            return self.keyframes[0].value;
        }
        let prev = &self.keyframes[idx - 1];
        let next = &self.keyframes[idx];
        let local_t = if (next.time - prev.time).abs() < f64::EPSILON {
            0.0
        } else {
            (t - prev.time) / (next.time - prev.time)
        };
        prev.value + (next.value - prev.value) * local_t
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationTimeline {
    pub duration: Duration,
    pub curves: Vec<AnimationCurve>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationFrame {
    pub progress: f64,
    pub values: Vec<f64>,
}

pub trait AnimationDriver {
    fn sample(&mut self, now: Instant) -> Option<AnimationFrame>;
    fn is_active(&self) -> bool;
    fn start_time(&self) -> Instant;
    fn duration(&self) -> Duration;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easing_linear() {
        assert_eq!(EasingFunction::Linear.sample(0.0), 0.0);
        assert_eq!(EasingFunction::Linear.sample(0.5), 0.5);
        assert_eq!(EasingFunction::Linear.sample(1.0), 1.0);
    }

    #[test]
    fn easing_spring_at_endpoints() {
        assert_eq!(EasingFunction::Spring.sample(0.0), 0.0);
        assert!((EasingFunction::Spring.sample(1.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn curve_with_keyframes() {
        let curve = AnimationCurve {
            easing: EasingFunction::Linear,
            keyframes: vec![
                Keyframe { time: 0.0, value: 0.0 },
                Keyframe { time: 1.0, value: 100.0 },
            ],
        };
        assert_eq!(curve.sample(0.0), 0.0);
        assert_eq!(curve.sample(1.0), 100.0);
        assert!((curve.sample(0.5) - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn curve_empty_keyframes_uses_easing() {
        let curve = AnimationCurve {
            easing: EasingFunction::Linear,
            keyframes: vec![],
        };
        assert_eq!(curve.sample(0.5), 0.5);
    }
}
