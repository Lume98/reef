#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_rgb_sets_alpha_to_255() {
        let c = Color::rgb(128, 64, 32);
        assert_eq!(c.r, 128);
        assert_eq!(c.g, 64);
        assert_eq!(c.b, 32);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn color_rgba_sets_all_channels() {
        let c = Color::rgba(10, 20, 30, 128);
        assert_eq!(c.a, 128);
    }
}
