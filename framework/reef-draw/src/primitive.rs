use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};

#[derive(Clone, Debug, PartialEq)]
pub enum DrawPrimitive {
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
        frame: Rect,
        text: String,
        color: Color,
        size: i32,
        weight: TextWeight,
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
    Path {
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
pub enum TextWeight {
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

#[derive(Clone, Debug, PartialEq)]
pub struct DrawPlan {
    pub hidden: bool,
    pub viewport: Size,
    pub primitives: Vec<DrawPrimitive>,
}

impl Default for DrawPlan {
    fn default() -> Self {
        Self {
            hidden: false,
            viewport: Size {
                width: 0.0,
                height: 0.0,
            },
            primitives: Vec::new(),
        }
    }
}

impl DrawPlan {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_viewport(viewport: Size) -> Self {
        Self {
            hidden: false,
            viewport,
            primitives: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visual_plan_carries_platform_neutral_primitives() {
        let plan = DrawPlan {
            hidden: false,
            viewport: Size {
                width: 120.0,
                height: 48.0,
            },
            primitives: vec![
                DrawPrimitive::RoundRect {
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
                DrawPrimitive::Text {
                    frame: Rect {
                        x: 12.0,
                        y: 14.0,
                        width: 120.0,
                        height: 24.0,
                    },
                    text: "Reef UI".to_string(),
                    color: Color::rgb(230, 235, 245),
                    size: 13,
                    weight: TextWeight::Semibold,
                    alignment: TextAlignment::Center,
                    alpha: 1.0,
                },
                DrawPrimitive::ClipStart {
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 100.0,
                        height: 40.0,
                    },
                },
                DrawPrimitive::ClipEnd,
            ],
        };

        assert!(!plan.hidden);
        assert_eq!(
            plan.viewport,
            Size {
                width: 120.0,
                height: 48.0,
            }
        );
        assert_eq!(plan.primitives.len(), 4);
    }

    #[test]
    fn frame_submission_carries_multiple_plans() {
        let submission = crate::draw_backend::FrameSubmission {
            hidden: false,
            plans: vec![
                DrawPlan::with_viewport(Size {
                    width: 10.0,
                    height: 20.0,
                }),
                DrawPlan::with_viewport(Size {
                    width: 30.0,
                    height: 40.0,
                }),
            ],
        };

        assert_eq!(submission.plans.len(), 2);
        assert_eq!(submission.plans[1].viewport.height, 40.0);
    }

    #[test]
    fn path_segments_keep_explicit_points() {
        let segment = PathSegment::CubicBezier {
            control1: Point { x: 1.0, y: 2.0 },
            control2: Point { x: 3.0, y: 4.0 },
            end: Point { x: 5.0, y: 6.0 },
        };

        assert_eq!(
            segment,
            PathSegment::CubicBezier {
                control1: Point { x: 1.0, y: 2.0 },
                control2: Point { x: 3.0, y: 4.0 },
                end: Point { x: 5.0, y: 6.0 },
            }
        );
    }
}
