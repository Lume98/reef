pub mod absolute;
pub mod column;
pub mod padding;
pub mod row;
pub mod stack;

use crate::core::geometry::{Rect, Size};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
    pub min_width: f64,
    pub max_width: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl Constraints {
    pub fn loose(size: Size) -> Self {
        Self {
            min_width: 0.0,
            max_width: size.width,
            min_height: 0.0,
            max_height: size.height,
        }
    }

    pub fn tight(size: Size) -> Self {
        Self {
            min_width: size.width,
            max_width: size.width,
            min_height: size.height,
            max_height: size.height,
        }
    }

    pub fn constrain(&self, size: Size) -> Size {
        Size {
            width: size.width.clamp(self.min_width, self.max_width),
            height: size.height.clamp(self.min_height, self.max_height),
        }
    }
}

pub trait Layout {
    fn measure(&self, constraints: Constraints) -> Size;
    fn arrange(&self, bounds: Rect, children: &mut [LayoutChild]);
}

pub struct LayoutChild {
    pub id: u64,
    pub size: Size,
    pub offset: Rect,
}
