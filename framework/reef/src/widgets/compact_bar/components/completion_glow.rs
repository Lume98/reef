use crate::core::geometry::{Rect, Size};
use crate::draw::primitive::DrawPrimitive;
use crate::layout::Constraints;
use crate::view::widget_host::{PaintContext, Widget};

/// Breathing 9-slice glow image for completion animation.
#[derive(Clone)]
pub struct CompletionGlow {
    pub frame: Rect,
    pub base_opacity: f64,
    pub elapsed_ms: u128,
}

impl CompletionGlow {
    pub fn new(frame: Rect) -> Self {
        Self {
            frame,
            base_opacity: 0.6,
            elapsed_ms: 0,
        }
    }

    fn opacity(&self) -> f64 {
        let t = (self.elapsed_ms as f64 / 1000.0) * std::f64::consts::PI;
        let breathe = t.sin() * 0.3 + 0.7;
        self.base_opacity * breathe
    }
}

impl Widget for CompletionGlow {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.frame.width,
            height: self.frame.height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        let opacity = self.opacity();
        ctx.primitives.push(DrawPrimitive::NineSliceImage {
            key: "island-completion-inner-glow-9slice".to_string(),
            frame: self.frame,
            slice_left: 20.0,
            slice_right: 20.0,
            slice_top: 20.0,
            slice_bottom: 20.0,
            opacity,
        });
    }
}
