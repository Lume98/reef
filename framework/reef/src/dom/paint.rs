use crate::core::color::Color;
use crate::core::geometry::{Rect, Size};
use crate::dom::opaque::get_opaque_plan;
use crate::dom::scene::SceneNode;
use crate::draw::{DrawPlan, DrawPrimitive, TextAlignment, TextWeight};

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
        "image" => paint_image(node, out),
        "icon" => paint_icon(node, out),
        "button" => paint_container(node, out),
        "divider" => paint_divider(node, out),
        "codeblock" => paint_codeblock(node, out),
        "badge" => paint_badge(node, out),
        "spacer" => {} // No visual output
        "#text" => {}
        "$component" | "$context_provider" | "$root" => {
            for child in &node.children {
                paint_node(child, out);
            }
        }
        "$opaque_draw_plan" => {
            paint_opaque_draw_plan(node, out);
        }
        _ => {
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

    // Support explicit frame overrides via props (for hybrid opaque+VNode rendering)
    let frame = if let (Some(x), Some(y), Some(w), Some(h)) = (
        node.prop_f64("frame_x"),
        node.prop_f64("frame_y"),
        node.prop_f64("frame_w"),
        node.prop_f64("frame_h"),
    ) {
        Rect {
            x,
            y,
            width: w,
            height: h,
        }
    } else {
        node.frame
    };
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

fn paint_image(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    let key = match node.prop_str("key") {
        Some(k) => k,
        None => return,
    };
    let frame = node.frame;
    let opacity = node.prop_f64("opacity").unwrap_or(1.0);
    let is_sprite = node.prop_bool("sprite").unwrap_or(false);

    if is_sprite {
        out.push(DrawPrimitive::SpriteImage {
            key,
            source_rect: Rect {
                x: 0.0,
                y: 0.0,
                width: frame.width,
                height: frame.height,
            },
            frame,
            opacity,
        });
    } else {
        out.push(DrawPrimitive::Image {
            key,
            source_rect: Rect {
                x: 0.0,
                y: 0.0,
                width: frame.width,
                height: frame.height,
            },
            frame,
            opacity,
        });
    }

    paint_clipped_children(node, out);
}

fn paint_icon(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    let icon_name = match node.prop_str("icon") {
        Some(n) => n,
        None => return,
    };
    let frame = node.frame;
    let opacity = node.prop_f64("opacity").unwrap_or(1.0);

    out.push(DrawPrimitive::Image {
        key: format!("icon/{}", icon_name),
        source_rect: Rect {
            x: 0.0,
            y: 0.0,
            width: frame.width,
            height: frame.height,
        },
        frame,
        opacity,
    });

    for child in &node.children {
        paint_node(child, out);
    }
}

fn paint_divider(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    let frame = node.frame;
    let color = node
        .prop_color("color")
        .unwrap_or(Color::rgba(128, 128, 128, 80));
    let thickness = node.prop_f64("thickness").unwrap_or(1.0);

    // Center the line vertically in the frame
    let line_y = frame.y + frame.height / 2.0 - thickness / 2.0;
    out.push(DrawPrimitive::RoundRect {
        frame: Rect {
            x: frame.x,
            y: line_y,
            width: frame.width,
            height: thickness,
        },
        radius: thickness / 2.0,
        color,
        alpha: 1.0,
    });
}

fn paint_codeblock(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    let frame = node.frame;
    let bg = node
        .prop_color("background")
        .unwrap_or(Color::rgba(40, 40, 50, 200));
    let radius = node.prop_f64("radius").unwrap_or(4.0);

    // Background pill
    out.push(DrawPrimitive::RoundRect {
        frame,
        radius,
        color: bg,
        alpha: 1.0,
    });

    // Text label inside
    if let Some(text) = node.prop_str("text") {
        let color = node
            .prop_color("color")
            .unwrap_or(Color::rgb(200, 210, 230));
        out.push(DrawPrimitive::Text {
            frame: Rect {
                x: frame.x + 6.0,
                y: frame.y + 2.0,
                width: frame.width - 12.0,
                height: frame.height - 4.0,
            },
            text,
            color,
            size: 11,
            weight: TextWeight::Normal,
            alignment: TextAlignment::Left,
            alpha: 1.0,
        });
    }
}

fn paint_badge(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    let frame = node.frame;
    let color = node.prop_color("color").unwrap_or(Color::WHITE);
    let bg = node
        .prop_color("background")
        .unwrap_or(Color::rgba(255, 255, 255, 30));
    let radius = node.prop_f64("radius").unwrap_or(8.0);

    // Background pill
    out.push(DrawPrimitive::RoundRect {
        frame,
        radius,
        color: bg,
        alpha: 1.0,
    });

    // Text label inside
    if let Some(text) = node.prop_str("text") {
        out.push(DrawPrimitive::Text {
            frame: Rect {
                x: frame.x + 8.0,
                y: frame.y + 2.0,
                width: frame.width - 16.0,
                height: frame.height - 4.0,
            },
            text,
            color,
            size: 10,
            weight: TextWeight::Semibold,
            alignment: TextAlignment::Center,
            alpha: 1.0,
        });
    }
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

fn paint_opaque_draw_plan(node: &SceneNode, out: &mut Vec<DrawPrimitive>) {
    if let Some(id) = node.prop_i32("__opaque_id") {
        if let Some(prims) = get_opaque_plan(id) {
            out.extend(prims);
        }
    }
    // Also paint children (for hybrid opaque+VNode elements)
    for child in &node.children {
        paint_node(child, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vnode::PropsMap;

    #[test]
    fn paint_empty_container() {
        let node = SceneNode::new(1, "container", PropsMap::new());
        let plan = paint_scene_to_plan(
            &node,
            Size {
                width: 100.0,
                height: 100.0,
            },
        );
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

        let plan = paint_scene_to_plan(
            &node,
            Size {
                width: 200.0,
                height: 200.0,
            },
        );
        assert_eq!(plan.primitives.len(), 1);
        match plan.primitives[0] {
            DrawPrimitive::RoundRect {
                frame,
                radius,
                color,
                alpha,
            } => {
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

        let plan = paint_scene_to_plan(
            &node,
            Size {
                width: 200.0,
                height: 200.0,
            },
        );
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

        let plan = paint_scene_to_plan(
            &container,
            Size {
                width: 100.0,
                height: 100.0,
            },
        );
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

        let plan = paint_scene_to_plan(
            &node,
            Size {
                width: 200.0,
                height: 200.0,
            },
        );
        assert_eq!(plan.primitives.len(), 2);
        assert!(matches!(
            plan.primitives[0],
            DrawPrimitive::RoundRect { .. }
        ));
        assert!(matches!(
            plan.primitives[1],
            DrawPrimitive::StrokedRoundRect { .. }
        ));
    }
}
