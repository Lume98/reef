use crate::core::geometry::{Rect, Size};

/// Arrange a list of pre-measured sizes horizontally within the given rect.
/// Returns child rects in the same order.
pub fn arrange_row(rect: Rect, sizes: &[Size], gap: f64) -> Vec<Rect> {
    let mut rects = Vec::with_capacity(sizes.len());
    let mut x = rect.x;
    for size in sizes {
        rects.push(Rect {
            x,
            y: rect.y,
            width: size.width,
            height: size.height,
        });
        x += size.width + gap;
    }
    rects
}

/// Compute the total size of a horizontal row from child sizes.
pub fn row_total_size(sizes: &[Size], gap: f64) -> Size {
    let mut total_width: f64 = 0.0;
    let mut max_height: f64 = 0.0;
    for size in sizes {
        total_width += size.width;
        max_height = max_height.max(size.height);
    }
    if sizes.len() > 1 {
        total_width += gap * (sizes.len() - 1) as f64;
    }
    Size {
        width: total_width,
        height: max_height,
    }
}
