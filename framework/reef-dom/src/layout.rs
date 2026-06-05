use crate::scene::SceneNode;
use reef_core::geometry::{Rect, Size};
use reef_layout::column::{arrange_column, column_child_constraints, column_total_size};
use reef_layout::padding::deflate_constraints;
use reef_layout::Constraints;

/// Run a layout pass on the scene tree.
///
/// Walks the tree top-down, applying Constraints-based measurement and
/// arranging children within the computed frames.
pub fn layout_scene(node: &mut SceneNode, constraints: Constraints) -> Size {
    match node.ty.as_str() {
        "container" => layout_container(node, constraints),
        "label" => layout_leaf(node, constraints),
        "row" => layout_row(node, constraints),
        "column" => layout_column(node, constraints),
        "stack" => layout_stack(node, constraints),
        "#text" => layout_leaf(node, constraints),
        "$component" | "$context_provider" => {
            // Pass through — delegate to first child
            if let Some(first) = node.children.first_mut() {
                layout_scene(first, constraints)
            } else {
                constraints.constrain(Size { width: 0.0, height: 0.0 })
            }
        }
        _ => layout_leaf(node, constraints),
    }
}

fn layout_container(node: &mut SceneNode, constraints: Constraints) -> Size {
    let padding = node.prop_f64("padding").unwrap_or(0.0);
    let min_w = node.prop_f64("min_width").unwrap_or(0.0);
    let min_h = node.prop_f64("min_height").unwrap_or(0.0);

    let inner_constraints = deflate_constraints(constraints, padding);

    let mut total_size = if let Some(first) = node.children.first_mut() {
        layout_scene(first, inner_constraints)
    } else {
        Size {
            width: 0.0,
            height: 0.0,
        }
    };

    // Apply min size
    total_size.width = total_size.width.max(min_w).max(padding * 2.0);
    total_size.height = total_size.height.max(min_h).max(padding * 2.0);

    let constrained = constraints.constrain(total_size);
    node.frame = Rect {
        x: 0.0,
        y: 0.0,
        width: constrained.width,
        height: constrained.height,
    };

    // Arrange children within padding
    if let Some(first) = node.children.first_mut() {
        first.frame.x = padding;
        first.frame.y = padding;
    }

    constrained
}

fn layout_leaf(node: &mut SceneNode, constraints: Constraints) -> Size {
    let intrinsic = match node.ty.as_str() {
        "label" => {
            let text_len = node.prop_str("text").map(|t| t.len() as f64).unwrap_or(0.0);
            let font_size = node.prop_f64("font_size").unwrap_or(14.0);
            // Rough estimate: ~0.6 * font_size per character width, font_size for height
            Size {
                width: text_len * font_size * 0.6,
                height: font_size * 1.4,
            }
        }
        "#text" => Size {
            width: 0.0,
            height: 0.0,
        },
        _ => Size {
            width: 0.0,
            height: 0.0,
        },
    };

    let constrained = constraints.constrain(intrinsic);
    node.frame = Rect {
        x: 0.0,
        y: 0.0,
        width: constrained.width,
        height: constrained.height,
    };
    constrained
}

fn layout_column(node: &mut SceneNode, constraints: Constraints) -> Size {
    let gap = node.prop_f64("gap").unwrap_or(0.0);
    let child_count = node.children.len();

    let child_sizes: Vec<Size> = node
        .children
        .iter_mut()
        .map(|child| {
            let child_cx = column_child_constraints(constraints, gap, child_count);
            layout_scene(child, child_cx)
        })
        .collect();

    let total = column_total_size(&child_sizes, gap);
    let constrained = constraints.constrain(total);

    node.frame = Rect {
        x: 0.0,
        y: 0.0,
        width: constrained.width,
        height: constrained.height,
    };

    // Arrange children vertically
    let rects = arrange_column(node.frame, &child_sizes, gap);
    for (i, child) in node.children.iter_mut().enumerate() {
        if i < rects.len() {
            child.frame = rects[i];
        }
    }

    constrained
}

fn layout_row(node: &mut SceneNode, _constraints: Constraints) -> Size {
    // Simplified row layout — arrange children horizontally
    let gap = node.prop_f64("gap").unwrap_or(0.0);
    let mut x = 0.0_f64;
    let mut max_h = 0.0_f64;

    for child in node.children.iter_mut() {
        // Measure child with loose constraints
        let child_size = layout_scene(child, Constraints::loose(Size { width: 0.0, height: 0.0 }));
        child.frame = Rect {
            x,
            y: 0.0,
            width: child_size.width,
            height: child_size.height,
        };
        x += child_size.width + gap;
        max_h = max_h.max(child_size.height);
    }

    let total_w = if x > 0.0 { x - gap } else { 0.0 };
    node.frame = Rect {
        x: 0.0,
        y: 0.0,
        width: total_w,
        height: max_h,
    };
    Size {
        width: total_w,
        height: max_h,
    }
}

fn layout_stack(node: &mut SceneNode, constraints: Constraints) -> Size {
    // Stack: children overlap, each gets full constraints
    let mut max_w = 0.0_f64;
    let mut max_h = 0.0_f64;

    for child in node.children.iter_mut() {
        let size = layout_scene(child, constraints);
        child.frame = Rect {
            x: 0.0,
            y: 0.0,
            width: size.width,
            height: size.height,
        };
        max_w = max_w.max(size.width);
        max_h = max_h.max(size.height);
    }

    let constrained = constraints.constrain(Size {
        width: max_w,
        height: max_h,
    });
    node.frame = Rect {
        x: 0.0,
        y: 0.0,
        width: constrained.width,
        height: constrained.height,
    };
    constrained
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_vnode::PropsMap;

    fn make_label(text: &str) -> SceneNode {
        let mut props = PropsMap::new();
        props.insert("text", text);
        SceneNode::new(1, "label", props)
    }

    #[test]
    fn layout_empty_container() {
        let mut node = SceneNode::new(1, "container", PropsMap::new());
        let size = layout_scene(&mut node, Constraints::loose(Size { width: 200.0, height: 200.0 }));
        assert_eq!(size.width, 0.0);
        assert_eq!(size.height, 0.0);
    }

    #[test]
    fn layout_label() {
        let mut node = make_label("Hello");
        let size = layout_scene(&mut node, Constraints::loose(Size { width: 200.0, height: 50.0 }));
        assert!(size.width > 0.0);
        assert!(size.height > 0.0);
    }

    #[test]
    fn layout_container_with_child() {
        let mut container = SceneNode::new(1, "container", PropsMap::new());
        let label = make_label("Hello");
        container.children.push(label);

        let size = layout_scene(
            &mut container,
            Constraints::loose(Size { width: 200.0, height: 200.0 }),
        );
        assert!(size.width > 0.0);
        assert!(size.height > 0.0);
        assert_eq!(container.children.len(), 1);
        // Child should have a non-zero frame
        assert!(container.children[0].frame.width > 0.0);
    }

    #[test]
    fn layout_column_with_children() {
        let mut column = SceneNode::new(1, "column", {
            let mut p = PropsMap::new();
            p.insert("gap", 8.0_f64);
            p
        });
        column.children.push(make_label("Row 1"));
        column.children.push(make_label("Row 2"));

        let constraints = Constraints::tight(Size { width: 150.0, height: 300.0 });
        let size = layout_scene(&mut column, constraints);

        assert_eq!(size.width, 150.0);
        assert!(size.height > 0.0);
        // Children should have different Y positions
        assert!(column.children[1].frame.y > column.children[0].frame.y);
    }

    #[test]
    fn layout_container_with_padding() {
        let mut container = SceneNode::new(1, "container", {
            let mut p = PropsMap::new();
            p.insert("padding", 12.0_f64);
            p
        });
        container.children.push(make_label("Hello"));

        layout_scene(
            &mut container,
            Constraints::loose(Size { width: 200.0, height: 200.0 }),
        );

        // Container frame height should account for padding
        // (12px top + label height + 12px bottom: height >= 24)
        assert!(container.frame.height >= 24.0);
        // Child should be offset by padding
        assert!(container.children[0].frame.x >= 12.0);
        assert!(container.children[0].frame.y >= 12.0);
    }
}
