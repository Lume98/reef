use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

/// MDL2 Assets glyph button (settings gear, quit X, etc.).
pub struct ActionButton {
    pub glyph: String,
    pub color: Color,
    pub size: i32,
    pub debug_mode: bool,
}

pub enum ActionKind {
    Settings,
    Quit,
}

impl ActionKind {
    pub fn glyph(&self) -> &'static str {
        match self {
            ActionKind::Settings => "\u{E713}",
            ActionKind::Quit => "\u{E7E8}",
        }
    }
}

impl ActionButton {
    pub fn new(kind: ActionKind) -> Self {
        Self {
            glyph: kind.glyph().to_string(),
            color: Color::rgb(160, 170, 190),
            size: 16,
            debug_mode: false,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Widget for ActionButton {
    fn measure(&self, constraints: Constraints) -> Size {
        let s = self.size as f64 + 8.0;
        constraints.constrain(Size { width: s, height: s })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        let cx = rect.x + rect.width / 2.0 - self.size as f64 / 2.0;
        let cy = rect.y + rect.height / 2.0 - self.size as f64 / 2.0;
        ctx.primitives.push(VisualPrimitive::Text {
            origin: reef_core::geometry::Point { x: cx, y: cy },
            max_width: rect.width,
            text: self.glyph.clone(),
            color: self.color,
            size: self.size,
            weight: reef_render::primitive::FontWeight::Normal,
            alignment: reef_render::primitive::TextAlignment::Left,
            alpha: 1.0,
        });
    }
}
