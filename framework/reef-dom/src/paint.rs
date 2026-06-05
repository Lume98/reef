use crate::scene::SceneNode;
use reef_core::color::Color;
use reef_core::geometry::{Rect, Size};
use reef_draw::{DrawPlan, DrawPrimitive, TextAlignment, TextWeight};

/// Paint a scene tree into a `DrawPlan` by walking the tree and emitting
/// `DrawPrimitive` commands for each node.
pub fn paint_scene_to_plan(node: &SceneNode, viewport: Size) -> DrawPlan {
    let mut primitives = Vec::new();
    paint_node(node, &mut primitives);
    DrawPlan {
        hidden: false,
        viewport,
        primitives,
    }
}

fn paint_node(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    match node.ty.as_str() {
        "container" => paint_container(node, out),
        "label" => paint_label(node, out),
        "row" | "column" | "stack" => paint_layout_container(node, out),
        "#text" => {} // Text-only nodes are handled by parent
        "$component" | "$context_provider" | "$root" => {
            // Transparent — paint children directly
            for child in &node.children {
                paint_node(child, out);
            }
        }
        _ => {
            // Unknown type — paint children as fallback
            if node.clip_children {
                paint_clipped_children(node, out);
            } else {
                for child in &node.children {
                    paint_node(child, out);
                }
            }
        }
    }
}

fn paint_container(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    let frame = node.frame;
    let color = node.prop_color("color").unwrap_or(Color::TRANSPARENT);
    let radius = node.prop_f64("radius").unwrap_or(0.0);
    let alpha = node.prop_f64("alpha").unwrap_or(1.0);
    let border_color = node.prop_color("border_color");
    let border_width = node.prop_f64("border_width").unwrap_or(0.0);

    // Background fill
    if color.a > 0 && alpha > 0.0 {
        if radius > 0.0 {
            out.push(DrawPrimitive::RoundRect {
                frame,
                radius,
                color,
                alpha,
            });
        } else {
            out.push(DrawPrimitive::Rect {
                frame,
                color,
                alpha,
            });
        }
    }

    // Border (if specified)
    if let Some(border_col) = border_color {
        if border_width > 0.0 {
            out.push(DrawPrimitive::StrokedRoundRect {
                frame,
                radius,
                fill: Color::TRANSPARENT,
                stroke: border_col,
                stroke_width: border_width,
                alpha: 1.0,
            });
        }
    }

    // Clip and paint children
    if node.clip_children && !node.children.is_empty() {
        out.push(DrawPrimitive::ClipStart { frame });
        for child in &node.children {
            paint_node(child, out);
        }
        out.push(DrawPrimitive::ClipEnd);
    } else {
        for child in &node.children {
            paint_node(child, out);
        }
    }
}

fn paint_label(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    let text = match node.prop_str("text") {
        Some(t) => t,
        None => return,
    };

    let frame = node.frame;
    let color = node.prop_color("color").unwrap_or(Color::WHITE);
    let font_size = node.prop_i32("font_size").unwrap_or(14);
    let alpha = node.prop_f64("alpha").unwrap_or(1.0);

    let weight = match node.prop_str("weight").as_deref() {
        Some("bold") => TextWeight::Bold,
        Some("semibold") => TextWeight::Semibold,
        _ => TextWeight::Normal,
    };

    let alignment = match node.prop_str("alignment").as_deref() {
        Some("center") => TextAlignment::Center,
        Some("right") => TextAlignment::Right,
        _ => TextAlignment::Left,
    };

    out.push(DrawPrimitive::Text {
        frame,
        text,
        color,
        size: font_size,
        weight,
        alignment,
        alpha,
    });
}

fn paint_layout_container(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    // Layout containers (row/column/stack) have no visual of their own
    // — they just position children
    if node.clip_children {
        out.push(DrawPrimitive::ClipStart { frame: node.frame });
    }

    for child in &node.children {
        paint_node(child, out);
    }

    if node.clip_children {
        out.push(DrawPrimitive::ClipEnd);
    }
}

fn paint_clipped_children(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    if !node.children.is_empty() {
        out.push(DrawPrimitive::ClipStart { frame: node.frame });
        for child in &node.children {
            paint_node(child, out);
        }
        out.push(DrawPrimitive::ClipEnd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_vnode::PropsMap;

    #[test]
    fn paint_empty_container() {
        let node = SceneNode::new(1, "container", PropsMap::new());
        let plan = paint_scene_to_plan(&node, Size { width: 100.0, height: 100.0 });
        assert!(!plan.hidden);
        assert_eq!(plan.viewport.width, 100.0);
        // No primitives: container has no color, radius, or children
        assert_eq!(plan.primitives.len(), 0);
    }

    #[test]
    fn paint_colored_container() {
        let mut props = PropsMap::new();
        props.insert("color", Color::rgb(18, 18, 22));
        props.insert("radius", 12.0_f64);

        let mut node = SceneNode::new(1, "container", props);
        node.frame = Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 50.0,
        };

        let plan = paint_scene_to_plan(&node, Size { width: 200.0, height: 200.0 });
        assert_eq!(plan.primitives.len(), 1);
        match plan.primitives[0] {
            DrawPrimitive::RoundRect { frame, radius, color, alpha } => {
                assert_eq!(frame.width, 100.0);
                assert_eq!(radius, 12.0);
                assert_eq!(color, Color::rgb(18, 18, 22));
                assert_eq!(alpha, 1.0);
            }
            _ => panic!("expected RoundRect"),
        }
    }

    #[test]
    fn paint_label_primitive() {
        let mut props = PropsMap::new();
        props.insert("text", "Hello");
        props.insert("color", Color::WHITE);

        let mut node = SceneNode::new(2, "label", props);
        node.frame = Rect {
            x: 10.0,
            y: 10.0,
            width: 50.0,
            height: 20.0,
        };

        let plan = paint_scene_to_plan(&node, Size { width: 200.0, height: 200.0 });
        assert_eq!(plan.primitives.len(), 1);
        match &plan.primitives[0] {
            DrawPrimitive::Text { text, frame, .. } => {
                assert_eq!(text, "Hello");
                assert_eq!(frame.x, 10.0);
            }
            other => panic!("expected Text, got {:?}", other),
        }
    }

    #[test]
    fn paint_container_with_children() {
        let mut container = SceneNode::new(1, "container", {
            let mut p = PropsMap::new();
            p.insert("color", Color::rgb(18, 18, 22));
            p
        });
        container.frame = Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };

        let mut label = SceneNode::new(2, "label", {
            let mut p = PropsMap::new();
            p.insert("text", "Hi");
            p
        });
        label.frame = Rect {
            x: 10.0,
            y: 10.0,
            width: 30.0,
            height: 20.0,
        };

        container.children.push(label);

        let plan = paint_scene_to_plan(&container, Size { width: 100.0, height: 100.0 });
        // Should have: Rect + ClipStart + Text + ClipEnd
        assert_eq!(plan.primitives.len(), 4);
    }

    #[test]
    fn paint_container_with_border() {
        let mut props = PropsMap::new();
        props.insert("color", Color::rgb(18, 18, 22));
        props.insert("radius", 8.0_f64);
        props.insert("border_color", Color::WHITE);
        props.insert("border_width", 1.0_f64);

        let mut node = SceneNode::new(1, "container", props);
        node.frame = Rect {
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 50.0,
        };

        let plan = paint_scene_to_plan(&node, Size { width: 200.0, height: 200.0 });
        assert_eq!(plan.primitives.len(), 2);
        assert!(matches!(plan.primitives[0], DrawPrimitive::RoundRect { .. }));
        assert!(matches!(plan.primitives[1], DrawPrimitive::StrokedRoundRect { .. }));
    }
}
