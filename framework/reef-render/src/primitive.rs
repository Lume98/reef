use reef_core::{color::Color, geometry::{Point, Rect}};

#[derive(Clone, Debug, PartialEq)]
pub enum VisualPrimitive {
    ClipStart {
        frame: Rect,
    },
    ClipEnd,
    RoundRect {
        frame: Rect,
        radius: f64,
        color: Color,
        alpha: f64,
    },
    Rect {
        frame: Rect,
        color: Color,
        alpha: f64,
    },
    Ellipse {
        frame: Rect,
        color: Color,
        alpha: f64,
    },
    StrokeLine {
        from: Point,
        to: Point,
        color: Color,
        width: f64,
        alpha: f64,
    },
    Text {
        origin: Point,
        max_width: f64,
        text: String,
        color: Color,
        size: i32,
        weight: FontWeight,
        alignment: TextAlignment,
        alpha: f64,
    },
    Image {
        key: String,
        source_rect: Rect,
        frame: Rect,
        opacity: f64,
    },
    StrokedRoundRect {
        frame: Rect,
        radius: f64,
        fill: Color,
        stroke: Color,
        stroke_width: f64,
        alpha: f64,
    },
    NineSliceImage {
        key: String,
        frame: Rect,
        slice_left: f64,
        slice_right: f64,
        slice_top: f64,
        slice_bottom: f64,
        opacity: f64,
    },
    BezierPath {
        segments: Vec<PathSegment>,
        fill: Color,
        alpha: f64,
    },
    SpriteImage {
        key: String,
        source_rect: Rect,
        frame: Rect,
        opacity: f64,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum PathSegment {
    LineTo(Point),
    CubicBezier {
        control1: Point,
        control2: Point,
        end: Point,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Semibold,
    Bold,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VisualPlan {
    pub hidden: bool,
    pub primitives: Vec<VisualPrimitive>,
}

impl VisualPlan {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visual_plan_carries_platform_neutral_primitives() {
        let plan = VisualPlan {
            hidden: false,
            primitives: vec![
                VisualPrimitive::RoundRect {
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 120.0,
                        height: 48.0,
                    },
                    radius: 24.0,
                    color: Color::rgb(18, 18, 22),
                    alpha: 1.0,
                },
                VisualPrimitive::Text {
                    origin: Point { x: 12.0, y: 14.0 },
                    max_width: 120.0,
                    text: "Reef UI".to_string(),
                    color: Color::rgb(230, 235, 245),
                    size: 13,
                    weight: FontWeight::Semibold,
                    alignment: TextAlignment::Center,
                    alpha: 1.0,
                },
                VisualPrimitive::ClipStart {
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 100.0,
                        height: 40.0,
                    },
                },
                VisualPrimitive::ClipEnd,
            ],
        };

        assert!(!plan.hidden);
        assert_eq!(plan.primitives.len(), 4);
    }
}
