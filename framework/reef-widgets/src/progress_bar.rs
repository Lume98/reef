use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_draw::primitive::DrawPrimitive;
use reef_layout::Constraints;
use reef_theme::progress_bar as theme;
use reef_view::widget_host::{PaintContext, Widget};

#[derive(Clone, Debug)]
pub struct ProgressBar {
    pub value: f64,
    pub height: f64,
    pub radius: f64,
    pub track_color: Color,
    pub fill_color: Color,
}

impl ProgressBar {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            height: 8.0,
            radius: 4.0,
            track_color: Color::from(theme::TRACK),
            fill_color: Color::from(theme::FILL),
        }
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self.radius = height / 2.0;
        self
    }
}

impl Widget for ProgressBar {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: self.height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        ctx.primitives.push(DrawPrimitive::RoundRect {
            frame: rect,
            radius: self.radius,
            color: self.track_color,
            alpha: 1.0,
        });

        let filled = Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width * self.value.clamp(0.0, 1.0),
            height: rect.height,
        };
        ctx.primitives.push(DrawPrimitive::RoundRect {
            frame: filled,
            radius: self.radius,
            color: self.fill_color,
            alpha: 1.0,
        });
    }
}
