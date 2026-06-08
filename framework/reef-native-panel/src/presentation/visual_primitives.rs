use crate::state::{PanelPoint, PanelRect};
use reef::core::{
    color::Color,
    geometry::{Point, Rect},
};
use reef::draw::primitive::{TextAlignment, TextWeight};
use reef::theme::Rgb;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativePanelVisualColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl NativePanelVisualColor {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<Rgb> for NativePanelVisualColor {
    fn from(value: Rgb) -> Self {
        Self::rgb(value.r, value.g, value.b)
    }
}

impl From<NativePanelVisualColor> for Color {
    fn from(value: NativePanelVisualColor) -> Self {
        Self::rgb(value.r, value.g, value.b)
    }
}

pub fn draw_rect(rect: PanelRect) -> Rect {
    Rect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}

pub fn draw_point(point: PanelPoint) -> Point {
    Point {
        x: point.x,
        y: point.y,
    }
}

pub fn native_panel_visual_text_box_height(text: &str, size: i32) -> f64 {
    let line_count = text.lines().count().max(1) as f64;
    let line_height = if size >= 13 { 24.0 } else { size as f64 + 8.0 };
    line_count * line_height
}

pub fn native_panel_visual_text_box_height_for_role(
    role: NativePanelVisualTextRole,
    text: &str,
    size: i32,
) -> f64 {
    let line_count = text.lines().count().max(1) as f64;
    let line_height = match role {
        NativePanelVisualTextRole::CardBodyText | NativePanelVisualTextRole::CardBodyPrefix => {
            crate::state::CARD_CHAT_LINE_HEIGHT
        }
        NativePanelVisualTextRole::CardStatusBadge
        | NativePanelVisualTextRole::CardSourceBadge
        | NativePanelVisualTextRole::CardToolName
        | NativePanelVisualTextRole::CardToolDescription
        | NativePanelVisualTextRole::CardActionHint
        | NativePanelVisualTextRole::CardSettingsValue => size as f64 + 3.0,
        _ => return native_panel_visual_text_box_height(text, size),
    };
    line_count * line_height.max(1.0)
}

pub fn native_panel_visual_text_frame(
    role: NativePanelVisualTextRole,
    origin: PanelPoint,
    max_width: f64,
    text: &str,
    size: i32,
) -> Rect {
    Rect {
        x: origin.x,
        y: origin.y,
        width: max_width,
        height: native_panel_visual_text_box_height_for_role(role, text, size),
    }
}

pub fn native_panel_visual_plain_text_frame(
    origin: PanelPoint,
    max_width: f64,
    text: &str,
    size: i32,
) -> Rect {
    Rect {
        x: origin.x,
        y: origin.y,
        width: max_width,
        height: native_panel_visual_text_box_height(text, size),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativePanelVisualTextWeight {
    Normal,
    Semibold,
    Bold,
}

impl From<NativePanelVisualTextWeight> for TextWeight {
    fn from(value: NativePanelVisualTextWeight) -> Self {
        match value {
            NativePanelVisualTextWeight::Normal => Self::Normal,
            NativePanelVisualTextWeight::Semibold => Self::Semibold,
            NativePanelVisualTextWeight::Bold => Self::Bold,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativePanelVisualTextAlignment {
    Left,
    Center,
    Right,
}

impl From<NativePanelVisualTextAlignment> for TextAlignment {
    fn from(value: NativePanelVisualTextAlignment) -> Self {
        match value {
            NativePanelVisualTextAlignment::Left => Self::Left,
            NativePanelVisualTextAlignment::Center => Self::Center,
            NativePanelVisualTextAlignment::Right => Self::Right,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativePanelVisualTextRole {
    Unspecified,
    CompactHeadline,
    CompactActiveCount,
    CompactActiveCountNext,
    CompactSlash,
    CompactTotalCount,
    ActionButtonSettings,
    ActionButtonQuit,
    CardTitle,
    CardSubtitle,
    CardStatusBadge,
    CardSourceBadge,
    CardBodyPrefix,
    CardBodyText,
    CardToolName,
    CardToolDescription,
    CardActionHint,
    CardSettingsTitle,
    CardSettingsValue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativePanelVisualShoulderSide {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::{
        native_panel_visual_text_box_height, native_panel_visual_text_box_height_for_role,
        native_panel_visual_text_frame, NativePanelVisualTextRole,
    };
    use crate::state::PanelPoint;

    #[test]
    fn visual_text_box_height_is_shared_for_platform_text_rendering() {
        assert_eq!(native_panel_visual_text_box_height("Reef UI", 13), 24.0);
        assert_eq!(native_panel_visual_text_box_height("2", 8), 16.0);
        assert_eq!(
            native_panel_visual_text_box_height("line one\nline two", 10),
            36.0
        );
    }

    #[test]
    fn visual_text_box_height_uses_tight_boxes_for_card_pill_text_roles() {
        assert_eq!(
            native_panel_visual_text_box_height_for_role(
                NativePanelVisualTextRole::CardStatusBadge,
                "Idle",
                10
            ),
            13.0
        );
        assert_eq!(
            native_panel_visual_text_box_height_for_role(
                NativePanelVisualTextRole::CardToolName,
                "Bash",
                9
            ),
            12.0
        );
    }

    #[test]
    fn visual_text_frame_carries_computed_height_before_backend() {
        let frame = native_panel_visual_text_frame(
            NativePanelVisualTextRole::CardBodyText,
            PanelPoint { x: 10.0, y: 20.0 },
            120.0,
            "line one\nline two",
            10,
        );

        assert_eq!(frame.x, 10.0);
        assert_eq!(frame.width, 120.0);
        assert_eq!(frame.height, crate::state::CARD_CHAT_LINE_HEIGHT * 2.0);
    }
}
