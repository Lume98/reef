use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;

use super::shoulder_path::{ShoulderPath, ShoulderSide};

/// Animated shoulder nubbin (bezier path).
#[derive(Clone)]
pub struct CompactShoulder {
    pub frame: Rect,
    pub side: ShoulderSide,
    pub progress: f64,
    pub fill_color: Color,
    pub border_color: Color,
}

impl Widget for CompactShoulder {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.frame.width,
            height: self.frame.height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        ShoulderPath {
            frame: self.frame,
            side: self.side,
            progress: self.progress,
            fill_color: self.fill_color,
        }
        .paint(self.frame, ctx);
    }
}
