use crate::Constraints;
use reef_core::geometry::Size;

/// Deflate constraints by padding on all sides.
pub fn deflate_constraints(constraints: Constraints, padding: f64) -> Constraints {
    let doubled = padding * 2.0;
    Constraints {
        min_width: (constraints.min_width - doubled).max(0.0),
        max_width: (constraints.max_width - doubled).max(0.0),
        min_height: (constraints.min_height - doubled).max(0.0),
        max_height: (constraints.max_height - doubled).max(0.0),
    }
}

/// Inflate a child size by padding on all sides.
pub fn inflate_size(size: Size, padding: f64) -> Size {
    Size {
        width: size.width + padding * 2.0,
        height: size.height + padding * 2.0,
    }
}
