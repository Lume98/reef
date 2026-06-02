use reef_core::geometry::Rect;

pub struct StackLayoutInput {
    pub available_height: f64,
    pub available_width: f64,
    pub gap: f64,
    pub overhang: f64,
}

pub fn resolve_stacked_card_frame(
    cursor_y: &mut f64,
    needs_gap: bool,
    height: f64,
    width: f64,
    gap: f64,
    overhang: f64,
) -> Option<Rect> {
    if needs_gap {
        *cursor_y -= gap;
    }
    if *cursor_y < height {
        return None;
    }

    *cursor_y -= height;
    Some(Rect {
        x: -overhang,
        y: *cursor_y,
        width: width + (overhang * 2.0),
        height,
    })
}

pub fn resolve_stacked_total_height(heights: &[f64], gap: f64) -> f64 {
    if heights.is_empty() {
        return 0.0;
    }
    heights.iter().copied().sum::<f64>() + gap * (heights.len() as f64 - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stacked_card_frames_bottom_up() {
        let mut cursor = 200.0;
        let first = resolve_stacked_card_frame(&mut cursor, false, 60.0, 200.0, 8.0, 2.0);
        let second = resolve_stacked_card_frame(&mut cursor, true, 40.0, 200.0, 8.0, 2.0);

        assert!(first.is_some());
        assert!(second.is_some());
        assert_eq!(first.unwrap().y, 140.0);
        assert_eq!(second.unwrap().y, 92.0);
    }

    #[test]
    fn stacked_card_returns_none_when_no_space() {
        let mut cursor = 30.0;
        let result = resolve_stacked_card_frame(&mut cursor, false, 60.0, 200.0, 8.0, 2.0);
        assert!(result.is_none());
    }

    #[test]
    fn stacked_total_height_sums_with_gaps() {
        assert_eq!(
            resolve_stacked_total_height(&[60.0, 40.0, 30.0], 8.0),
            146.0
        );
        assert_eq!(resolve_stacked_total_height(&[], 8.0), 0.0);
    }
}
