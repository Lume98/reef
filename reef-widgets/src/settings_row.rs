use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

/// Settings row with title, value badge, and optional active state.
pub struct SettingsRow {
    pub title: String,
    pub value: String,
    pub active: bool,
    pub title_color: Color,
    pub value_color: Color,
    pub active_bg: Color,
    pub inactive_bg: Color,
    pub font_size: i32,
    pub height: f64,
}

impl SettingsRow {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            active: false,
            title_color: Color::rgb(200, 210, 225),
            value_color: Color::WHITE,
            active_bg: Color::rgb(40, 45, 55),
            inactive_bg: Color::rgb(25, 28, 35),
            font_size: 13,
            height: 36.0,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl Widget for SettingsRow {
    fn measure(&self, constraints: Constraints) -> Size {
        let width = constraints.max_width;
        constraints.constrain(Size { width, height: self.height })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        let bg = if self.active { self.active_bg } else { self.inactive_bg };
        ctx.primitives.push(VisualPrimitive::Rect {
            frame: rect,
            color: bg,
            alpha: 1.0,
        });
        // Title (left-aligned)
        ctx.primitives.push(VisualPrimitive::Text {
            origin: reef_core::geometry::Point { x: rect.x + 12.0, y: rect.y + 9.0 },
            max_width: rect.width * 0.6,
            text: self.title.clone(),
            color: self.title_color,
            size: self.font_size,
            weight: reef_render::primitive::FontWeight::Normal,
            alignment: reef_render::primitive::TextAlignment::Left,
            alpha: 1.0,
        });
        // Value (right-aligned badge)
        let value_w = self.value.chars().count() as f64 * self.font_size as f64 * 0.6 + 12.0;
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: Rect {
                x: rect.x + rect.width - value_w - 12.0,
                y: rect.y + 6.0,
                width: value_w,
                height: 22.0,
            },
            radius: 6.0,
            color: Color::rgb(50, 55, 70),
            alpha: 1.0,
        });
        ctx.primitives.push(VisualPrimitive::Text {
            origin: reef_core::geometry::Point {
                x: rect.x + rect.width - value_w - 6.0,
                y: rect.y + 9.0,
            },
            max_width: value_w,
            text: self.value.clone(),
            color: self.value_color,
            size: self.font_size,
            weight: reef_render::primitive::FontWeight::Normal,
            alignment: reef_render::primitive::TextAlignment::Center,
            alpha: 1.0,
        });
    }
}
