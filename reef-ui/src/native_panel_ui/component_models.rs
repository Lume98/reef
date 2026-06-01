use crate::native_panel_core::{PanelPoint, PanelRect};

use super::visual_primitives::NativePanelVisualColor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativePanelPanelColors {
    pub shell_fill: NativePanelVisualColor,
    pub shell_border: NativePanelVisualColor,
    pub separator: NativePanelVisualColor,
    pub text_primary: NativePanelVisualColor,
    pub text_secondary: NativePanelVisualColor,
}

impl Default for NativePanelPanelColors {
    fn default() -> Self {
        Self {
            shell_fill: NativePanelVisualColor::rgb(12, 12, 15),
            shell_border: NativePanelVisualColor::rgb(44, 44, 50),
            separator: NativePanelVisualColor::rgb(62, 62, 70),
            text_primary: NativePanelVisualColor::rgb(245, 247, 252),
            text_secondary: NativePanelVisualColor::rgb(230, 235, 245),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelContainerComponent {
    pub frame: PanelRect,
    pub radius: f64,
    pub fill: NativePanelVisualColor,
    pub border: Option<NativePanelVisualColor>,
    pub separator: Option<PanelRect>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelCompactBarComponent {
    pub frame: PanelRect,
    pub headline_origin: PanelPoint,
    pub headline_width: f64,
    pub active_origin: PanelPoint,
    pub total_origin: PanelPoint,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelStackComponent {
    pub frame: PanelRect,
    pub content_height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelSettingRowComponent {
    pub frame: PanelRect,
    pub title_frame: PanelRect,
    pub value_frame: PanelRect,
    pub active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelSessionCardComponent {
    pub frame: PanelRect,
    pub title_frame: PanelRect,
    pub body_frame: PanelRect,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePanelMastheadComponent {
    pub anchor: PanelPoint,
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NativePanelComponent {
    Container(NativePanelContainerComponent),
    CompactBar(NativePanelCompactBarComponent),
    Stack(NativePanelStackComponent),
    SettingRow(NativePanelSettingRowComponent),
    SessionCard(NativePanelSessionCardComponent),
    Masthead(NativePanelMastheadComponent),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NativePanelComponentTree {
    pub components: Vec<NativePanelComponent>,
}

impl NativePanelComponentTree {
    pub fn push(&mut self, component: NativePanelComponent) {
        self.components.push(component);
    }
}
