use std::sync::OnceLock;

use crate::{native_panel_core::PanelPoint, native_panel_scene::SceneMascotPose};
use reef_draw::primitive::DrawPrimitive;

use super::super::{
    env_flags::native_panel_enabled_from_env_value,
    mascot_sprite_spec::{
        parse_mascot_sprite_manifest, resolve_mascot_sprite_frame, MascotSpriteFrameInput,
        MascotSpriteFrameSpec, MascotSpriteManifest,
    },
    mascot_visual_spec::{
        MascotCompletionBadgeVisualSpec, MascotMessageBubbleVisualSpec, MascotRoundRectVisualSpec,
        MascotTextVisualSpec, MascotVisualSpec,
    },
    visual_primitives::{
        draw_rect, native_panel_visual_plain_text_frame, NativePanelVisualTextAlignment,
    },
};

const DEFAULT_MASCOT_SPRITE_MANIFEST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/mascot/default/pet.json"
));
const DEFAULT_MASCOT_SPRITE_PATH: &str = "mascot/default/spritesheet.png";
static DEFAULT_MASCOT_SPRITE_MANIFEST_CACHE: OnceLock<Option<MascotSpriteManifest>> =
    OnceLock::new();

pub(super) fn apply_mascot_chrome_alpha(primitives: &mut [DrawPrimitive], alpha: f64) {
    let alpha = alpha.clamp(0.0, 1.0);
    for primitive in primitives {
        match primitive {
            DrawPrimitive::RoundRect {
                alpha: primitive_alpha,
                ..
            }
            | DrawPrimitive::Ellipse {
                alpha: primitive_alpha,
                ..
            }
            | DrawPrimitive::StrokedRoundRect {
                alpha: primitive_alpha,
                ..
            }
            | DrawPrimitive::Text {
                alpha: primitive_alpha,
                ..
            } => {
                *primitive_alpha *= alpha;
            }
            DrawPrimitive::SpriteImage { opacity, .. } => {
                *opacity *= alpha;
            }
            _ => {}
        }
    }
}

pub(super) fn push_mascot_primitives(primitives: &mut Vec<DrawPrimitive>, spec: &MascotVisualSpec) {
    if spec.pose == SceneMascotPose::Hidden {
        return;
    }

    if mascot_sprite_enabled() {
        if let Some(sprite) = resolve_default_mascot_sprite(spec) {
            primitives.push(DrawPrimitive::SpriteImage {
                key: DEFAULT_MASCOT_SPRITE_PATH.to_string(),
                source_rect: draw_rect(sprite.source_rect),
                frame: draw_rect(sprite.frame),
                opacity: sprite.opacity,
            });
            if let Some(badge) = &spec.completion_badge {
                push_mascot_completion_badge(primitives, badge);
            }
            return;
        }
    }

    primitives.push(DrawPrimitive::StrokedRoundRect {
        frame: draw_rect(spec.body.frame),
        radius: spec.body.corner_radius.max(spec.body.radius),
        fill: spec.body.fill_color.into(),
        stroke: spec.body.stroke_color.into(),
        stroke_width: spec.body.stroke_width,
        alpha: spec.motion.shell_alpha,
    });
    if let Some(message_bubble) = &spec.message_bubble {
        push_mascot_message_bubble(primitives, message_bubble);
    }
    if let Some(sleep_label) = &spec.sleep_label {
        push_mascot_text(primitives, sleep_label);
    }
    for eye in &spec.eyes {
        primitives.push(DrawPrimitive::Ellipse {
            frame: draw_rect(eye.frame),
            color: eye.color.into(),
            alpha: 1.0,
        });
    }
    push_mascot_round_rect(primitives, spec.mouth, 1.0);
    if let Some(badge) = &spec.completion_badge {
        push_mascot_completion_badge(primitives, badge);
    }
}

fn mascot_sprite_enabled() -> bool {
    native_panel_enabled_from_env_value(!cfg!(test), std::env::var("ECHOISLAND_MASCOT_SPRITE").ok())
}

fn resolve_default_mascot_sprite(spec: &MascotVisualSpec) -> Option<MascotSpriteFrameSpec> {
    let manifest = default_mascot_sprite_manifest()?;
    let center = sprite_locked_mascot_center(spec);
    resolve_mascot_sprite_frame(MascotSpriteFrameInput {
        manifest,
        pose: spec.pose,
        center,
        elapsed_ms: spec.elapsed_ms,
        opacity: spec.motion.shell_alpha,
    })
}

fn sprite_locked_mascot_center(spec: &MascotVisualSpec) -> PanelPoint {
    PanelPoint {
        x: spec.body.center.x - spec.motion.offset_x,
        y: spec.body.center.y - spec.motion.offset_y,
    }
}

fn default_mascot_sprite_manifest() -> Option<&'static MascotSpriteManifest> {
    DEFAULT_MASCOT_SPRITE_MANIFEST_CACHE
        .get_or_init(|| parse_mascot_sprite_manifest(DEFAULT_MASCOT_SPRITE_MANIFEST).ok())
        .as_ref()
}

fn push_mascot_message_bubble(
    primitives: &mut Vec<DrawPrimitive>,
    bubble: &MascotMessageBubbleVisualSpec,
) {
    push_mascot_round_rect(primitives, bubble.bubble, bubble.alpha);
    for dot in &bubble.dots {
        primitives.push(DrawPrimitive::Ellipse {
            frame: draw_rect(dot.frame),
            color: dot.color.into(),
            alpha: bubble.alpha,
        });
    }
}

fn push_mascot_completion_badge(
    primitives: &mut Vec<DrawPrimitive>,
    badge: &MascotCompletionBadgeVisualSpec,
) {
    push_mascot_round_rect(primitives, badge.outline, 1.0);
    push_mascot_round_rect(primitives, badge.fill, 1.0);
    push_mascot_text(primitives, &badge.label);
}

fn push_mascot_round_rect(
    primitives: &mut Vec<DrawPrimitive>,
    spec: MascotRoundRectVisualSpec,
    alpha: f64,
) {
    primitives.push(DrawPrimitive::RoundRect {
        frame: draw_rect(spec.frame),
        radius: spec.radius,
        color: spec.color.into(),
        alpha,
    });
}

fn push_mascot_text(primitives: &mut Vec<DrawPrimitive>, spec: &MascotTextVisualSpec) {
    primitives.push(DrawPrimitive::Text {
        frame: native_panel_visual_plain_text_frame(
            spec.origin,
            spec.max_width,
            &spec.text,
            spec.size,
        ),
        text: spec.text.clone(),
        color: spec.color.into(),
        size: spec.size,
        weight: spec.weight.into(),
        alignment: NativePanelVisualTextAlignment::Center.into(),
        alpha: spec.alpha,
    });
}
