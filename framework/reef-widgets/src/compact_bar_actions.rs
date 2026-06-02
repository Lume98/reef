use reef_app::widget_host::{PaintContext, Widget};
use reef_core::{
    color::Color,
    geometry::{Point, Rect, Size},
};
use reef_layout::Constraints;
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

const SETTINGS_ICON: &str = "\u{E713}";
const QUIT_ICON: &str = "\u{E7E8}";

/// Action buttons on the compact bar.
#[derive(Clone)]
pub struct CompactBarActions {
    pub show_actions: bool,
    pub debug_mode: bool,
    pub visibility: f64,
    pub base_scale: f64,
    pub base_opacity: f64,
    pub base_offset_y: f64,
}

impl Widget for CompactBarActions {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size {
            width: constraints.max_width,
            height: constraints.max_height,
        })
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        if !self.show_actions {
            return;
        }

        let cy = rect.y + rect.height / 2.0;
        let av = self.visibility.clamp(0.0, 1.0);
        let scale = self.base_scale + (1.0 - self.base_scale) * av;
        let opacity = self.base_opacity + (1.0 - self.base_opacity) * av;
        let offset_y = self.base_offset_y * (1.0 - av);

        let sx = rect.x + 12.0;
        let sy = cy - 10.0 + offset_y;
        ctx.primitives.push(VisualPrimitive::Text {
            origin: Point { x: sx, y: sy },
            max_width: 24.0,
            text: SETTINGS_ICON.to_string(),
            color: if self.debug_mode {
                Color::rgb(102, 222, 145)
            } else {
                Color::rgb(245, 247, 252)
            },
            size: (16.0 * scale) as i32,
            weight: FontWeight::Normal,
            alignment: TextAlignment::Left,
            alpha: opacity,
        });

        let qx = rect.x + rect.width - 28.0;
        let qy = cy - 10.0 + offset_y;
        ctx.primitives.push(VisualPrimitive::Text {
            origin: Point { x: qx, y: qy },
            max_width: 24.0,
            text: QUIT_ICON.to_string(),
            color: Color::rgb(255, 82, 82),
            size: (16.0 * scale) as i32,
            weight: FontWeight::Bold,
            alignment: TextAlignment::Left,
            alpha: opacity,
        });
    }
}
