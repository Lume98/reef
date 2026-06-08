use crate::core::color::Color;

pub trait Theme {
    fn color(&self, token: ColorToken) -> Color;
    fn font_size(&self, role: TextRole) -> i32;
    fn font_weight(&self, role: TextRole) -> TextWeight;
    fn spacing(&self, kind: SpacingKind) -> f64;
    fn radius(&self, kind: RadiusKind) -> f64;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorToken {
    Background,
    Surface,
    SurfaceBorder,
    TextPrimary,
    TextSecondary,
    Accent,
    Separator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextRole {
    Headline,
    Body,
    Caption,
    Badge,
    Label,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextWeight {
    Normal,
    Semibold,
    Bold,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpacingKind {
    Gap,
    Inset,
    Padding,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadiusKind {
    Small,
    Medium,
    Large,
}
