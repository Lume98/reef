use crate::core::geometry::{Rect, Size};

pub fn absolute_rect(parent: Rect, child: Rect) -> Rect {
    Rect {
        x: parent.x + child.x,
        y: parent.y + child.y,
        width: child.width,
        height: child.height,
    }
}

pub fn centered_top_frame(screen: Rect, size: Size) -> Rect {
    let snapped_width = size.width.max(1.0).round();
    let snapped_height = size.height.max(1.0).round();
    let top_edge = screen.y + screen.height;

    Rect {
        x: (screen.x + ((screen.width - snapped_width) / 2.0).max(0.0)).round(),
        y: (top_edge - snapped_height).round(),
        width: snapped_width,
        height: snapped_height,
    }
}

pub fn centered_frame(container: Rect, size: Size) -> Rect {
    Rect {
        x: container.x + ((container.width - size.width) / 2.0).max(0.0),
        y: container.y + ((container.height - size.height) / 2.0).max(0.0),
        width: size.width,
        height: size.height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_rect_offsets_from_parent() {
        let parent = Rect {
            x: 100.0,
            y: 50.0,
            width: 400.0,
            height: 300.0,
        };
        let child = Rect {
            x: 10.0,
            y: 20.0,
            width: 50.0,
            height: 60.0,
        };
        let result = absolute_rect(parent, child);
        assert_eq!(result.x, 110.0);
        assert_eq!(result.y, 70.0);
        assert_eq!(result.width, 50.0);
        assert_eq!(result.height, 60.0);
    }

    #[test]
    fn centered_top_frame_positions_at_top_center() {
        let screen = Rect {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        };
        let size = Size {
            width: 200.0,
            height: 40.0,
        };
        let result = centered_top_frame(screen, size);
        assert_eq!(result.x, 300.0);
        assert_eq!(result.y, 560.0);
        assert_eq!(result.width, 200.0);
        assert_eq!(result.height, 40.0);
    }
}
