use reef_view::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;

mod background;
mod dots;

pub use background::MessageBubbleBackground;
pub use dots::MessageBubbleDots;

/// Message bubble shown above the mascot (3 dots + rounded rect).
#[derive(Clone)]
pub struct MessageBubble {
    pub center_x: f64,
    pub bottom_y: f64,
    pub bubble_width: f64,
    pub bubble_height: f64,
    pub dot_radius: f64,
    pub fill_color: Color,
    pub alpha: f64,
}

impl MessageBubble {
    pub fn new(center_x: f64, bottom_y: f64) -> Self {
        Self {
            center_x,
            bottom_y,
            bubble_width: 48.0,
            bubble_height: 22.0,
            dot_radius: 3.0,
            fill_color: Color::rgb(47, 47, 52),
            alpha: 1.0,
        }
    }
}

impl Widget for MessageBubble {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: self.bubble_width,
            height: self.bubble_height,
        })
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        let bx = self.center_x - self.bubble_width / 2.0;
        let by = self.bottom_y - self.bubble_height;
        let frame = Rect {
            x: bx,
            y: by,
            width: self.bubble_width,
            height: self.bubble_height,
        };

        MessageBubbleBackground {
            frame,
            fill_color: self.fill_color,
            alpha: self.alpha,
        }
        .paint(frame, ctx);

        MessageBubbleDots {
            center_x: self.center_x,
            bottom_y: self.bottom_y,
            bubble_height: self.bubble_height,
            dot_radius: self.dot_radius,
            alpha: self.alpha,
        }
        .paint(frame, ctx);
    }
}
