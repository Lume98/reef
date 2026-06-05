use reef_ui::panel::core::{PanelDisplayGeometry, PanelRect};

use crate::dpi::WindowsDpiScale;

pub fn fallback_standalone_display_geometry() -> PanelDisplayGeometry {
    PanelDisplayGeometry {
        x: 0,
        y: 0,
        width: 1440,
        height: 900,
    }
}

pub fn windows_standalone_screen_frame_with_scale(
    display_geometry: PanelDisplayGeometry,
    dpi_scale: WindowsDpiScale,
) -> PanelRect {
    PanelRect {
        x: display_geometry.x as f64 / dpi_scale.scale,
        y: display_geometry.y as f64 / dpi_scale.scale,
        width: display_geometry.width as f64 / dpi_scale.scale,
        height: display_geometry.height as f64 / dpi_scale.scale,
    }
}
