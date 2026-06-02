use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{PathSegment, VisualPrimitive};

/// Animated shoulder nubbin (bezier path).
pub struct CompactShoulder {
    pub frame: Rect,
    pub side: ShoulderSide,
    pub progress: f64,
    pub fill_color: Color,
    pub border_color: Color,
}

pub enum ShoulderSide {
    Left,
    Right,
}

impl CompactShoulder {
    fn build_segments(&self) -> Vec<PathSegment> {
        let r = self.frame;
        match self.side {
            ShoulderSide::Left => vec![
                PathSegment::LineTo(Point { x: r.x, y: r.y + r.height * self.progress }),
                PathSegment::CubicBezier {
                    control1: Point { x: r.x, y: r.y + r.height * 0.3 },
                    control2: Point { x: r.x + r.width * 0.6, y: r.y },
                    end: Point { x: r.x + r.width, y: r.y },
                },
            ],
            ShoulderSide::Right => vec![
                PathSegment::LineTo(Point { x: r.x + r.width, y: r.y + r.height * self.progress }),
                PathSegment::CubicBezier {
                    control1: Point { x: r.x + r.width, y: r.y + r.height * 0.3 },
                    control2: Point { x: r.x + r.width * 0.4, y: r.y },
                    end: Point { x: r.x, y: r.y },
                },
            ],
        }
    }
}

impl Widget for CompactShoulder {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size { width: self.frame.width, height: self.frame.height })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        ctx.primitives.push(VisualPrimitive::BezierPath {
            segments: self.build_segments(),
            fill: self.fill_color,
            alpha: 1.0,
        });
    }
}
