use reef_render::{primitive::VisualPrimitive, render_backend::RenderBackend};

use reef_core::geometry::Rect;

#[derive(Default)]
pub struct Direct2DPainter;

impl Direct2DPainter {
    pub fn new() -> Self {
        Self
    }

    pub fn paint_primitives(&mut self, primitives: &[VisualPrimitive]) -> PaintResult {
        if primitives.is_empty() {
            return PaintResult::default();
        }
        PaintResult {
            primitive_count: primitives.len(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PaintResult {
    pub primitive_count: usize,
}

impl RenderBackend for Direct2DPainter {
    type Error = String;

    fn submit_frame(
        &mut self,
        submission: &reef_render::render_backend::FrameSubmission,
    ) -> Result<(), Self::Error> {
        if submission.hidden {
            return Ok(());
        }
        for plan in &submission.commands {
            self.paint_primitives(&plan.primitives);
        }
        Ok(())
    }
}

pub fn resolve_paint_operations(plan: &reef_render::primitive::VisualPlan) -> Vec<PaintOperation> {
    if plan.hidden {
        return Vec::new();
    }
    plan.primitives
        .iter()
        .map(|p| match p {
            VisualPrimitive::ClipStart { frame } => PaintOperation::PushClip { frame: *frame },
            VisualPrimitive::ClipEnd => PaintOperation::PopClip,
            VisualPrimitive::RoundRect {
                frame,
                radius,
                color,
                alpha,
            } => PaintOperation::FillRoundRect {
                frame: *frame,
                radius: *radius,
                color: *color,
                alpha: *alpha,
            },
            VisualPrimitive::Rect {
                frame,
                color,
                alpha,
            } => PaintOperation::FillRect {
                frame: *frame,
                color: *color,
                alpha: *alpha,
            },
            VisualPrimitive::Ellipse {
                frame,
                color,
                alpha,
            } => PaintOperation::FillEllipse {
                frame: *frame,
                color: *color,
                alpha: *alpha,
            },
            VisualPrimitive::StrokeLine {
                from,
                to,
                color,
                width,
                alpha,
            } => PaintOperation::StrokeLine {
                from: *from,
                to: *to,
                color: *color,
                width: *width,
                alpha: *alpha,
            },
            VisualPrimitive::Text {
                origin,
                max_width,
                text,
                color,
                size,
                weight,
                alignment,
                alpha,
            } => PaintOperation::DrawText {
                origin: *origin,
                max_width: *max_width,
                text: text.clone(),
                color: *color,
                size: *size,
                weight: *weight,
                alignment: *alignment,
                alpha: *alpha,
            },
            VisualPrimitive::Image {
                key,
                source_rect,
                frame,
                opacity,
            } => PaintOperation::DrawImage {
                key: key.clone(),
                source_rect: *source_rect,
                frame: *frame,
                opacity: *opacity,
            },
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
pub enum PaintOperation {
    PushClip { frame: Rect },
    PopClip,
    FillRoundRect {
        frame: Rect,
        radius: f64,
        color: reef_core::color::Color,
        alpha: f64,
    },
    FillRect {
        frame: Rect,
        color: reef_core::color::Color,
        alpha: f64,
    },
    FillEllipse {
        frame: Rect,
        color: reef_core::color::Color,
        alpha: f64,
    },
    StrokeLine {
        from: reef_core::geometry::Point,
        to: reef_core::geometry::Point,
        color: reef_core::color::Color,
        width: f64,
        alpha: f64,
    },
    DrawText {
        origin: reef_core::geometry::Point,
        max_width: f64,
        text: String,
        color: reef_core::color::Color,
        size: i32,
        weight: reef_render::primitive::FontWeight,
        alignment: reef_render::primitive::TextAlignment,
        alpha: f64,
    },
    DrawImage {
        key: String,
        source_rect: Rect,
        frame: Rect,
        opacity: f64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::{color::Color, geometry::{Point, Rect}};
    use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

    #[test]
    fn painter_handles_empty_plan() {
        let mut painter = Direct2DPainter::new();
        let result = painter.paint_primitives(&[]);
        assert_eq!(result.primitive_count, 0);
    }

    #[test]
    fn painter_counts_primitives() {
        let mut painter = Direct2DPainter::new();
        let primitives = vec![
            VisualPrimitive::RoundRect {
                frame: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 100.0,
                    height: 40.0,
                },
                radius: 20.0,
                color: Color::rgb(18, 18, 22),
                alpha: 1.0,
            },
            VisualPrimitive::Text {
                origin: Point { x: 10.0, y: 10.0 },
                max_width: 80.0,
                text: "Hello".to_string(),
                color: Color::WHITE,
                size: 14,
                weight: FontWeight::Normal,
                alignment: TextAlignment::Left,
                alpha: 1.0,
            },
        ];
        let result = painter.paint_primitives(&primitives);
        assert_eq!(result.primitive_count, 2);
    }

    #[test]
    fn resolve_paint_operations_maps_all_variants() {
        let plan = reef_render::primitive::VisualPlan {
            hidden: false,
            primitives: vec![
                VisualPrimitive::ClipStart {
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 100.0,
                        height: 50.0,
                    },
                },
                VisualPrimitive::ClipEnd,
                VisualPrimitive::RoundRect {
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 100.0,
                        height: 40.0,
                    },
                    radius: 20.0,
                    color: Color::rgb(18, 18, 22),
                    alpha: 1.0,
                },
                VisualPrimitive::Rect {
                    frame: Rect {
                        x: 5.0,
                        y: 5.0,
                        width: 90.0,
                        height: 30.0,
                    },
                    color: Color::BLACK,
                    alpha: 0.5,
                },
                VisualPrimitive::Ellipse {
                    frame: Rect {
                        x: 10.0,
                        y: 10.0,
                        width: 20.0,
                        height: 20.0,
                    },
                    color: Color::WHITE,
                    alpha: 1.0,
                },
                VisualPrimitive::StrokeLine {
                    from: Point { x: 0.0, y: 0.0 },
                    to: Point {
                        x: 100.0,
                        y: 100.0,
                    },
                    color: Color::WHITE,
                    width: 1.0,
                    alpha: 1.0,
                },
                VisualPrimitive::Image {
                    key: "test.png".to_string(),
                    source_rect: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 50.0,
                        height: 50.0,
                    },
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 50.0,
                        height: 50.0,
                    },
                    opacity: 1.0,
                },
            ],
        };
        let ops = resolve_paint_operations(&plan);
        assert_eq!(ops.len(), 7);
    }

    #[test]
    fn resolve_paint_operations_returns_empty_for_hidden() {
        let plan = reef_render::primitive::VisualPlan {
            hidden: true,
            primitives: vec![VisualPrimitive::ClipEnd],
        };
        assert!(resolve_paint_operations(&plan).is_empty());
    }
}
