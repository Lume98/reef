use crate::core::geometry::{Rect, Size};
use crate::layout::Constraints;

/// Arrange a list of pre-measured sizes vertically within the given rect.
/// Returns child rects in the same order.
pub fn arrange_column(rect: Rect, sizes: &[Size], gap: f64) -> Vec<Rect> {
    let mut rects = Vec::with_capacity(sizes.len());
    let mut y = rect.y;
    for size in sizes {
        rects.push(Rect {
            x: rect.x,
            y,
            width: size.width,
            height: size.height,
        });
        y += size.height + gap;
    }
    rects
}

/// Compute the total size of a vertical column from child sizes.
pub fn column_total_size(sizes: &[Size], gap: f64) -> Size {
    let mut total_height: f64 = 0.0;
    let mut max_width: f64 = 0.0;
    for size in sizes {
        total_height += size.height;
        max_width = max_width.max(size.width);
    }
    if sizes.len() > 1 {
        total_height += gap * (sizes.len() - 1) as f64;
    }
    Size {
        width: max_width,
        height: total_height,
    }
}

/// Constrain column child heights to fit within available space.
pub fn column_child_constraints(
    constraints: Constraints,
    gap: f64,
    child_count: usize,
) -> Constraints {
    let total_gap = if child_count > 1 {
        gap * (child_count - 1) as f64
    } else {
        0.0
    };
    Constraints {
        min_width: constraints.min_width,
        max_width: constraints.max_width,
        min_height: 0.0,
        max_height: (constraints.max_height - total_gap).max(0.0),
    }
}
