use serde::Serialize;

use crate::state::{panel_display_key, PanelDisplayGeometry, PanelRect};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayOption {
    pub index: usize,
    pub key: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub supports_wide_island: bool,
}

pub fn display_key_for_panel_geometry(geometry: PanelDisplayGeometry) -> String {
    panel_display_key(geometry)
}

#[cfg_attr(target_os = "windows", allow(dead_code))]
pub fn panel_rect_from_panel_geometry(geometry: PanelDisplayGeometry) -> PanelRect {
    PanelRect {
        x: geometry.x as f64,
        y: geometry.y as f64,
        width: geometry.width as f64,
        height: geometry.height as f64,
    }
}

pub fn display_option_from_panel_geometry(
    index: usize,
    geometry: PanelDisplayGeometry,
    name: Option<String>,
) -> DisplayOption {
    display_option_from_panel_geometry_with_width_support(index, geometry, name, true)
}

pub fn display_option_from_panel_geometry_with_width_support(
    index: usize,
    geometry: PanelDisplayGeometry,
    name: Option<String>,
    supports_wide_island: bool,
) -> DisplayOption {
    DisplayOption {
        index,
        key: display_key_for_panel_geometry(geometry),
        name: name.unwrap_or_else(|| format!("Display {}", index + 1)),
        width: geometry.width.max(0) as u32,
        height: geometry.height.max(0) as u32,
        supports_wide_island,
    }
}
