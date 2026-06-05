use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_draw::primitive::{DrawPrimitive, TextAlignment, TextWeight};
use reef_layout::Constraints;
use reef_theme::compact_bar as theme;
use reef_view::widget_host::{PaintContext, Widget};

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

        let sx = rect.x + theme::ACTION_LEFT_INSET;
        let sy = cy - 10.0 + offset_y;
        ctx.primitives.push(DrawPrimitive::Text {
            frame: Rect {
                x: sx,
                y: sy,
                width: 24.0,
                height: 24.0,
            },
            text: SETTINGS_ICON.to_string(),
            color: if self.debug_mode {
                Color::from(theme::ACTION_DEBUG)
            } else {
                Color::WHITE
            },
            size: (theme::ACTION_ICON_SIZE * scale) as i32,
            weight: TextWeight::Normal,
            alignment: TextAlignment::Left,
            alpha: opacity,
        });

        let qx = rect.x + rect.width - theme::ACTION_RIGHT_INSET;
        let qy = cy - 10.0 + offset_y;
        ctx.primitives.push(DrawPrimitive::Text {
            frame: Rect {
                x: qx,
                y: qy,
                width: 24.0,
                height: 24.0,
            },
            text: QUIT_ICON.to_string(),
            color: Color::from(theme::ACTION_QUIT),
            size: (theme::ACTION_ICON_SIZE * scale) as i32,
            weight: TextWeight::Bold,
            alignment: TextAlignment::Left,
            alpha: opacity,
        });
    }
}
