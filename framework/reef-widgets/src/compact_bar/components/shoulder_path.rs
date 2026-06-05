use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_draw::primitive::{DrawPrimitive, PathSegment};
use reef_layout::Constraints;
use reef_view::widget_host::{PaintContext, Widget};

#[derive(Clone, Copy)]
pub enum ShoulderSide {
    Left,
    Right,
}

/// Geometry-only shoulder path layer.
#[derive(Clone)]
pub struct ShoulderPath {
    pub frame: Rect,
    pub side: ShoulderSide,
    pub progress: f64,
    pub fill_color: Color,
}

impl ShoulderPath {
    fn build_segments(&self) -> Vec<PathSegment> {
        let r = self.frame;
        match self.side {
            ShoulderSide::Left => vec![
                PathSegment::LineTo(Point {
                    x: r.x,
                    y: r.y + r.height * self.progress,
                }),
                PathSegment::CubicBezier {
                    control1: Point {
                        x: r.x,
                        y: r.y + r.height * 0.3,
                    },
                    control2: Point {
                        x: r.x + r.width * 0.6,
                        y: r.y,
                    },
                    end: Point {
                        x: r.x + r.width,
                        y: r.y,
                    },
                },
            ],
            ShoulderSide::Right => vec![
                PathSegment::LineTo(Point {
                    x: r.x + r.width,
                    y: r.y + r.height * self.progress,
                }),
                PathSegment::CubicBezier {
                    control1: Point {
                        x: r.x + r.width,
                        y: r.y + r.height * 0.3,
                    },
                    control2: Point {
                        x: r.x + r.width * 0.4,
                        y: r.y,
                    },
                    end: Point { x: r.x, y: r.y },
                },
            ],
        }
    }
}

impl Widget for ShoulderPath {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.frame.width,
            height: self.frame.height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        ctx.primitives.push(DrawPrimitive::Path {
            segments: self.build_segments(),
            fill: self.fill_color,
            alpha: 1.0,
        });
    }
}
