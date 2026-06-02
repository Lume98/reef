#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

pub fn lerp(start: f64, end: f64, progress: f64) -> f64 {
    start + ((end - start) * progress.clamp(0.0, 1.0))
}

pub fn clamp_f64(value: f64, min: f64, max: f64) -> f64 {
    if max < min {
        return min;
    }
    value.clamp(min, max)
}

pub fn smoothstep_unit(progress: f64) -> f64 {
    let progress = progress.clamp(0.0, 1.0);
    progress * progress * (3.0 - (2.0 * progress))
}

pub fn smoothstep_range(edge0: f64, edge1: f64, value: f64) -> f64 {
    if (edge1 - edge0).abs() <= f64::EPSILON {
        return if value >= edge1 { 1.0 } else { 0.0 };
    }
    smoothstep_unit((value - edge0) / (edge1 - edge0))
}

pub fn point_in_rect(point: Point, rect: Rect) -> bool {
    point.x >= rect.x
        && point.x <= rect.x + rect.width
        && point.y >= rect.y
        && point.y <= rect.y + rect.height
}

pub fn compose_local_rect(parent: Rect, child: Rect) -> Rect {
    Rect {
        x: parent.x + child.x,
        y: parent.y + child.y,
        width: child.width,
        height: child.height,
    }
}

pub fn clamp_rect_to_bounds(rect: Rect, bounds: Rect) -> Rect {
    if bounds.width <= 0.0 || bounds.height <= 0.0 {
        return rect;
    }
    let width = rect.width.max(1.0).min(bounds.width);
    let height = rect.height.max(1.0).min(bounds.height);
    let max_x = bounds.x + (bounds.width - width).max(0.0);
    let max_y = bounds.y + (bounds.height - height).max(0.0);

    Rect {
        x: clamp_f64(rect.x, bounds.x, max_x),
        y: clamp_f64(rect.y, bounds.y, max_y),
        width,
        height,
    }
}

pub fn resolve_centered_top_frame(screen_frame: Rect, size: Size) -> Rect {
    let snapped_width = size.width.max(1.0).round();
    let snapped_height = size.height.max(1.0).round();
    let top_edge = screen_frame.y + screen_frame.height;

    Rect {
        x: (screen_frame.x + ((screen_frame.width - snapped_width) / 2.0).max(0.0)).round(),
        y: (top_edge - snapped_height).round(),
        width: snapped_width,
        height: snapped_height,
    }
}

pub fn rects_nearly_equal(a: Rect, b: Rect, tolerance: f64) -> bool {
    let tolerance = tolerance.max(0.0);
    (a.x - b.x).abs() < tolerance
        && (a.y - b.y).abs() < tolerance
        && (a.width - b.width).abs() < tolerance
        && (a.height - b.height).abs() < tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_clamp_moves_offscreen_rect_into_bounds() {
        assert_eq!(
            clamp_rect_to_bounds(
                Rect {
                    x: -200.0,
                    y: 900.0,
                    width: 320.0,
                    height: 80.0,
                },
                Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 800.0,
                    height: 600.0,
                },
            ),
            Rect {
                x: 0.0,
                y: 520.0,
                width: 320.0,
                height: 80.0,
            }
        );
    }

    #[test]
    fn rect_clamp_shrinks_rect_larger_than_bounds() {
        assert_eq!(
            clamp_rect_to_bounds(
                Rect {
                    x: 25.0,
                    y: 25.0,
                    width: 1000.0,
                    height: 900.0,
                },
                Rect {
                    x: -100.0,
                    y: -50.0,
                    width: 640.0,
                    height: 480.0,
                },
            ),
            Rect {
                x: -100.0,
                y: -50.0,
                width: 640.0,
                height: 480.0,
            }
        );
    }

    #[test]
    fn point_in_rect_checks_bounds() {
        let rect = Rect {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 50.0,
        };
        assert!(point_in_rect(Point { x: 50.0, y: 40.0 }, rect));
        assert!(!point_in_rect(Point { x: 5.0, y: 40.0 }, rect));
    }

    #[test]
    fn lerp_interpolates() {
        assert_eq!(lerp(0.0, 100.0, 0.5), 50.0);
        assert_eq!(lerp(0.0, 100.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 100.0, 1.0), 100.0);
    }

    #[test]
    fn smoothstep_produces_s_curve() {
        assert_eq!(smoothstep_unit(0.0), 0.0);
        assert_eq!(smoothstep_unit(1.0), 1.0);
        assert!(smoothstep_unit(0.5) > 0.0 && smoothstep_unit(0.5) < 1.0);
    }
}
