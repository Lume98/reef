use super::{
    PanelAnimationDescriptor, PanelAnimationKind, PanelMascotBaseState, PanelTransitionFrame,
    ACTIVE_COUNT_SCROLL_HOLD_MS, ACTIVE_COUNT_SCROLL_MOVE_MS, ACTIVE_COUNT_SCROLL_STEP_MS,
    ACTIVE_COUNT_SCROLL_TRAVEL, ACTIVE_COUNT_SLOT_NUDGE_X, ACTIVE_COUNT_SLOT_WIDTH,
    MASCOT_WAKE_ANGRY_SECONDS,
};

const SHARED_CONTENT_REVEAL_PROGRESS: f64 = 0.94;
const SHARED_CONTENT_INTERACTIVE_PROGRESS: f64 = 0.985;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelRenderProgress {
    pub bar: f64,
    pub height: f64,
    pub shoulder: f64,
    pub drop: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharedExpandedContentInput {
    pub enabled: bool,
    pub shell_visible: bool,
    pub height_progress: f64,
    pub bar_progress: f64,
    pub cards_height: f64,
    pub status_surface_active: bool,
    pub content_visibility: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharedExpandedContentState {
    pub visible: bool,
    pub interactive: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharedExpandedRenderInput {
    pub enabled: bool,
    pub shell_visible: bool,
    pub height_progress: f64,
    pub bar_progress: f64,
    pub cards_height: f64,
    pub status_surface_active: bool,
    pub content_visibility: f64,
    pub transitioning: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharedExpandedRenderState {
    pub enabled: bool,
    pub visible: bool,
    pub interactive: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelRenderLayerStyleInput {
    pub shell_visible: bool,
    pub separator_visibility: f64,
    pub shared_visible: bool,
    pub bar_progress: f64,
    pub height_progress: f64,
    pub chrome_transition_progress: f64,
    pub shoulder_progress: f64,
    pub headline_emphasized: bool,
    pub edge_actions_visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelRenderLayerStyleState {
    pub shell_visible: bool,
    pub separator_visibility: f64,
    pub shared_visible: bool,
    pub bar_progress: f64,
    pub height_progress: f64,
    pub chrome_transition_progress: f64,
    pub shoulder_progress: f64,
    pub headline_emphasized: bool,
    pub edge_actions_visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelRenderStateInput {
    pub shared_expanded_enabled: bool,
    pub shell_visible: bool,
    pub separator_visibility: f64,
    pub bar_progress: f64,
    pub height_progress: f64,
    pub chrome_transition_progress: f64,
    pub shoulder_progress: f64,
    pub cards_height: f64,
    pub status_surface_active: bool,
    pub content_visibility: f64,
    pub transitioning: bool,
    pub headline_emphasized: bool,
    pub edge_actions_visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelRenderState {
    pub shared: SharedExpandedRenderState,
    pub layer_style: PanelRenderLayerStyleState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharedBodyHeightDecisionInput {
    pub current_height: Option<f64>,
    pub requested_height: f64,
    pub has_snapshot: bool,
    pub update_threshold: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharedBodyHeightDecision {
    pub next_height: f64,
    pub should_update: bool,
    pub should_rerender: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn clamp_panel_rect_to_bounds(rect: PanelRect, bounds: PanelRect) -> PanelRect {
    if bounds.width <= 0.0 || bounds.height <= 0.0 {
        return rect;
    }
    let width = rect.width.max(1.0).min(bounds.width);
    let height = rect.height.max(1.0).min(bounds.height);
    let max_x = bounds.x + (bounds.width - width).max(0.0);
    let max_y = bounds.y + (bounds.height - height).max(0.0);

    PanelRect {
        x: clamp_f64(rect.x, bounds.x, max_x),
        y: clamp_f64(rect.y, bounds.y, max_y),
        width,
        height,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelGeometryMetrics {
    pub compact_height: f64,
    pub compact_width: f64,
    pub expanded_width: f64,
    pub panel_width: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelScreenTopArea {
    pub screen_width: f64,
    pub auxiliary_left_width: f64,
    pub auxiliary_right_width: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelScreenWidthInput {
    pub top_area: PanelScreenTopArea,
    pub compact_height: f64,
    pub default_compact_width: f64,
    pub expanded_width_delta: f64,
    pub default_expanded_width: f64,
    pub default_canvas_width: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelLayoutInput {
    pub screen_frame: PanelRect,
    pub metrics: PanelGeometryMetrics,
    pub canvas_height: f64,
    pub visible_height: f64,
    pub bar_progress: f64,
    pub height_progress: f64,
    pub drop_progress: f64,
    pub content_visibility: f64,
    pub collapsed_height: f64,
    pub drop_distance: f64,
    pub content_top_gap: f64,
    pub content_bottom_inset: f64,
    pub cards_side_inset: f64,
    pub shoulder_size: f64,
    pub separator_side_inset: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelLayout {
    pub panel_frame: PanelRect,
    pub content_frame: PanelRect,
    pub pill_frame: PanelRect,
    pub left_shoulder_frame: PanelRect,
    pub right_shoulder_frame: PanelRect,
    pub expanded_frame: PanelRect,
    pub cards_frame: PanelRect,
    pub separator_frame: PanelRect,
    pub shared_content_frame: PanelRect,
    pub shell_visible: bool,
    pub separator_visibility: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CompactBarContentLayoutInput {
    pub bar_width: f64,
    pub bar_height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CompactBarContentLayout {
    pub mascot_center_x: f64,
    pub headline_x: f64,
    pub headline_width: f64,
    pub headline_center_x: f64,
    pub active_x: f64,
    pub slash_x: f64,
    pub total_x: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CompactActionButtonLayout {
    pub settings: PanelRect,
    pub quit: PanelRect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActiveCountMarqueeInput<'a> {
    pub text: &'a str,
    pub elapsed_ms: u128,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActiveCountMarqueeFrame {
    pub current: String,
    pub next: String,
    pub show_next: bool,
    pub scroll_offset: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MascotVisualFrameInput {
    pub state: PanelMascotBaseState,
    pub elapsed_ms: u128,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MascotVisualFrame {
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub shell_alpha: f64,
    pub shadow_opacity: f64,
    pub shadow_radius: f64,
    pub eye_open: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MascotVisualFrameTransitionInput {
    pub start: MascotVisualFrame,
    pub target: MascotVisualFrame,
    pub elapsed_ms: u128,
    pub duration_ms: u128,
}

pub fn resolve_compact_bar_content_layout(
    input: CompactBarContentLayoutInput,
) -> CompactBarContentLayout {
    let mascot_size = (input.bar_height - 9.0).clamp(24.0, 28.0);
    let left_inset = ((input.bar_height - mascot_size) / 2.0).clamp(8.0, 12.0);
    let headline_width = 156.0;
    let headline_x = (input.bar_width - headline_width) / 2.0;
    let total_width = 24.0;
    let slash_width = 12.0;
    let active_width = ACTIVE_COUNT_SLOT_WIDTH;
    let group_right = input.bar_width.max(208.0);
    let total_x = group_right - total_width;
    let slash_x = total_x - slash_width;
    let active_x = (slash_x - active_width + ACTIVE_COUNT_SLOT_NUDGE_X).max(168.0);

    CompactBarContentLayout {
        mascot_center_x: left_inset + mascot_size / 2.0,
        headline_x,
        headline_width,
        headline_center_x: headline_x + headline_width / 2.0,
        active_x,
        slash_x,
        total_x,
    }
}

pub fn resolve_compact_action_button_layout(compact_frame: PanelRect) -> CompactActionButtonLayout {
    let action_size = 26.0;
    let content = resolve_compact_bar_content_layout(CompactBarContentLayoutInput {
        bar_width: compact_frame.width,
        bar_height: compact_frame.height,
    });
    let settings_center_x = content.mascot_center_x;
    let count_group_center_x = (content.active_x + content.total_x + 24.0) / 2.0;
    let settings_x = settings_center_x - action_size / 2.0;
    let quit_x = count_group_center_x - action_size / 2.0;
    let y = ((compact_frame.height - action_size) / 2.0).round();

    CompactActionButtonLayout {
        settings: PanelRect {
            x: compact_frame.x + settings_x,
            y: compact_frame.y + y,
            width: action_size,
            height: action_size,
        },
        quit: PanelRect {
            x: compact_frame.x + quit_x,
            y: compact_frame.y + y,
            width: action_size,
            height: action_size,
        },
    }
}

pub fn resolve_active_count_marquee_frame(
    input: ActiveCountMarqueeInput<'_>,
) -> ActiveCountMarqueeFrame {
    let chars = input.text.chars().collect::<Vec<_>>();
    if chars.is_empty() {
        return ActiveCountMarqueeFrame {
            current: "0".to_string(),
            next: "0".to_string(),
            show_next: false,
            scroll_offset: 0.0,
        };
    }

    let current = chars[0].to_string();
    let next = chars.get(1).copied().unwrap_or(chars[0]).to_string();
    let show_next = chars.len() > 1;
    let phase = if !show_next {
        0.0
    } else {
        let step_elapsed = input.elapsed_ms % ACTIVE_COUNT_SCROLL_STEP_MS;
        if step_elapsed < ACTIVE_COUNT_SCROLL_HOLD_MS {
            0.0
        } else if step_elapsed < ACTIVE_COUNT_SCROLL_HOLD_MS + ACTIVE_COUNT_SCROLL_MOVE_MS {
            ((step_elapsed - ACTIVE_COUNT_SCROLL_HOLD_MS) as f64
                / ACTIVE_COUNT_SCROLL_MOVE_MS as f64)
                .clamp(0.0, 1.0)
        } else {
            1.0
        }
    };

    ActiveCountMarqueeFrame {
        current,
        next,
        show_next,
        scroll_offset: (ACTIVE_COUNT_SCROLL_TRAVEL * phase).round(),
    }
}

pub fn resolve_mascot_visual_frame(input: MascotVisualFrameInput) -> MascotVisualFrame {
    let t = input.elapsed_ms as f64 / 1000.0;
    let (offset_x, offset_y, scale_x, scale_y, shell_alpha, shadow_opacity, shadow_radius) =
        match input.state {
            PanelMascotBaseState::Running => {
                let bounce = (t * 5.8).sin().abs();
                let hang = bounce.powf(0.72);
                let landing = (1.0 - bounce).powf(3.2);
                (
                    (t * 3.1).sin() * 0.28,
                    hang * 5.6,
                    1.0 + landing * 0.18 + hang * 0.018,
                    1.0 - landing * 0.16 + hang * 0.018,
                    1.0,
                    0.46,
                    5.4,
                )
            }
            PanelMascotBaseState::Approval => {
                let pulse = ((t * 7.2).sin() + 1.0) * 0.5;
                (
                    (t * 9.0).sin() * 0.34,
                    0.0,
                    1.0 + pulse * 0.025,
                    1.0 - pulse * 0.018,
                    1.0,
                    0.52,
                    6.0,
                )
            }
            PanelMascotBaseState::Question => {
                let tilt = (t * 4.4).sin();
                (
                    tilt * 0.28,
                    (t * 5.1).sin() * 0.55,
                    1.0 + tilt.abs() * 0.012,
                    1.0,
                    1.0,
                    0.50,
                    5.8,
                )
            }
            PanelMascotBaseState::MessageBubble => {
                let bob = ((t * 3.2).sin() + 1.0) * 0.5;
                (
                    0.0,
                    bob * 1.6,
                    1.0 + bob * 0.012,
                    1.0 - bob * 0.008,
                    1.0,
                    0.46,
                    5.2,
                )
            }
            PanelMascotBaseState::Complete => {
                let bob = ((t * 2.4).sin() + 1.0) * 0.5;
                (
                    0.0,
                    bob * 0.8,
                    1.0 + bob * 0.010,
                    1.0 - bob * 0.006,
                    1.0,
                    0.48,
                    5.4,
                )
            }
            PanelMascotBaseState::Sleepy => {
                let breath = ((t * 0.9).sin() + 1.0) * 0.5;
                let sleepy_phase = (t + 0.9).rem_euclid(7.6);
                let nod = if sleepy_phase > 5.1 && sleepy_phase < 5.95 {
                    (((sleepy_phase - 5.1) / 0.85) * std::f64::consts::PI).sin()
                } else {
                    0.0
                };
                (
                    0.0,
                    nod * -0.7,
                    1.0 + breath * 0.012,
                    0.96 - breath * 0.012 + nod * 0.01,
                    0.70,
                    0.18,
                    3.0,
                )
            }
            PanelMascotBaseState::WakeAngry => {
                let fade = 1.0 - smoothstep_range(0.52, MASCOT_WAKE_ANGRY_SECONDS, t);
                (
                    (t * 30.0).sin() * 1.85 * fade,
                    0.0,
                    1.0 + 0.045 * fade,
                    1.0 - 0.04 * fade,
                    1.0,
                    0.56,
                    6.4,
                )
            }
            PanelMascotBaseState::Idle => {
                let breath = ((t * 1.1).sin() + 1.0) * 0.5;
                (
                    0.0,
                    0.0,
                    1.0 + breath * 0.006,
                    1.0 - breath * 0.004,
                    1.0,
                    0.34,
                    4.0,
                )
            }
        };

    MascotVisualFrame {
        offset_x,
        offset_y,
        scale_x,
        scale_y,
        shell_alpha,
        shadow_opacity,
        shadow_radius,
        eye_open: resolve_mascot_eye_open(input.state, input.elapsed_ms),
    }
}

pub fn resolve_mascot_visual_frame_transition(
    input: MascotVisualFrameTransitionInput,
) -> MascotVisualFrame {
    let progress = if input.duration_ms == 0 {
        1.0
    } else {
        input.elapsed_ms as f64 / input.duration_ms as f64
    };
    let progress = smoothstep_unit(progress);

    MascotVisualFrame {
        offset_x: lerp(input.start.offset_x, input.target.offset_x, progress),
        offset_y: lerp(input.start.offset_y, input.target.offset_y, progress),
        scale_x: lerp(input.start.scale_x, input.target.scale_x, progress),
        scale_y: lerp(input.start.scale_y, input.target.scale_y, progress),
        shell_alpha: lerp(input.start.shell_alpha, input.target.shell_alpha, progress),
        shadow_opacity: lerp(
            input.start.shadow_opacity,
            input.target.shadow_opacity,
            progress,
        ),
        shadow_radius: lerp(
            input.start.shadow_radius,
            input.target.shadow_radius,
            progress,
        ),
        eye_open: lerp(input.start.eye_open, input.target.eye_open, progress),
    }
}

fn resolve_mascot_eye_open(state: PanelMascotBaseState, elapsed_ms: u128) -> f64 {
    let phase = ((elapsed_ms as f64 / 1000.0) + 0.35).rem_euclid(4.8);
    let scale = if phase < 0.09 {
        1.0 - (phase / 0.09) * 0.9
    } else if phase < 0.18 {
        0.1 + ((phase - 0.09) / 0.09) * 0.9
    } else {
        1.0
    };

    let floor = match state {
        PanelMascotBaseState::Approval => 0.34,
        PanelMascotBaseState::Question => 0.48,
        PanelMascotBaseState::Running | PanelMascotBaseState::Complete => 0.72,
        PanelMascotBaseState::Sleepy => 0.16,
        PanelMascotBaseState::WakeAngry => 1.0,
        PanelMascotBaseState::Idle | PanelMascotBaseState::MessageBubble => 0.12,
    };
    let scale = scale.clamp(floor, 1.0);
    if state == PanelMascotBaseState::Sleepy {
        let t = elapsed_ms as f64 / 1000.0;
        let sleepy_phase = (t + 1.1).rem_euclid(7.4);
        let mut sleepy_scale = scale * 0.72;
        if sleepy_phase > 4.7 && sleepy_phase < 5.45 {
            let pct = (sleepy_phase - 4.7) / 0.75;
            sleepy_scale = if pct < 0.5 {
                0.18
            } else {
                0.18 + (pct - 0.5) * 0.36
            };
        }
        return sleepy_scale.max(0.16);
    }
    scale
}

fn smoothstep_range(edge0: f64, edge1: f64, value: f64) -> f64 {
    if (edge1 - edge0).abs() <= f64::EPSILON {
        return if value >= edge1 { 1.0 } else { 0.0 };
    }
    smoothstep_unit((value - edge0) / (edge1 - edge0))
}

fn smoothstep_unit(progress: f64) -> f64 {
    let progress = progress.clamp(0.0, 1.0);
    progress * progress * (3.0 - (2.0 * progress))
}

pub fn resolve_panel_render_progress(frame: PanelTransitionFrame) -> PanelRenderProgress {
    PanelRenderProgress {
        bar: frame.bar_progress.clamp(0.0, 1.0),
        height: frame.height_progress.clamp(0.0, 1.0),
        shoulder: frame.shoulder_progress.clamp(0.0, 1.0),
        drop: frame.drop_progress.clamp(0.0, 1.0),
    }
}

pub fn resolve_shared_expanded_content_state(
    input: SharedExpandedContentInput,
) -> SharedExpandedContentState {
    let visible = input.enabled
        && input.shell_visible
        && input.height_progress > SHARED_CONTENT_REVEAL_PROGRESS
        && input.content_visibility > SHARED_CONTENT_REVEAL_PROGRESS
        && input.cards_height > 4.0
        && !input.status_surface_active;
    let interactive = visible
        && input.height_progress > SHARED_CONTENT_INTERACTIVE_PROGRESS
        && input.bar_progress > SHARED_CONTENT_INTERACTIVE_PROGRESS
        && input.content_visibility > SHARED_CONTENT_INTERACTIVE_PROGRESS;

    SharedExpandedContentState {
        visible,
        interactive,
    }
}

pub fn resolve_shared_expanded_render_state(
    input: SharedExpandedRenderInput,
) -> SharedExpandedRenderState {
    let state = resolve_shared_expanded_content_state(SharedExpandedContentInput {
        enabled: input.enabled,
        shell_visible: input.shell_visible,
        height_progress: input.height_progress,
        bar_progress: input.bar_progress,
        cards_height: input.cards_height,
        status_surface_active: input.status_surface_active,
        content_visibility: input.content_visibility,
    });

    SharedExpandedRenderState {
        enabled: input.enabled,
        visible: state.visible && !input.transitioning,
        interactive: state.interactive && !input.transitioning,
    }
}

pub fn resolve_panel_render_layer_style_state(
    input: PanelRenderLayerStyleInput,
) -> PanelRenderLayerStyleState {
    PanelRenderLayerStyleState {
        shell_visible: input.shell_visible,
        separator_visibility: input.separator_visibility,
        shared_visible: input.shared_visible,
        bar_progress: input.bar_progress,
        height_progress: input.height_progress,
        chrome_transition_progress: input.chrome_transition_progress,
        shoulder_progress: input.shoulder_progress,
        headline_emphasized: input.headline_emphasized,
        edge_actions_visible: input.edge_actions_visible,
    }
}

pub fn resolve_panel_render_state(input: PanelRenderStateInput) -> PanelRenderState {
    let shared = resolve_shared_expanded_render_state(SharedExpandedRenderInput {
        enabled: input.shared_expanded_enabled,
        shell_visible: input.shell_visible,
        height_progress: input.height_progress,
        bar_progress: input.bar_progress,
        cards_height: input.cards_height,
        status_surface_active: input.status_surface_active,
        content_visibility: input.content_visibility,
        transitioning: input.transitioning,
    });
    let layer_style = resolve_panel_render_layer_style_state(PanelRenderLayerStyleInput {
        shell_visible: input.shell_visible,
        separator_visibility: input.separator_visibility,
        shared_visible: shared.visible,
        bar_progress: input.bar_progress,
        height_progress: input.height_progress,
        chrome_transition_progress: input.chrome_transition_progress,
        shoulder_progress: input.shoulder_progress,
        headline_emphasized: input.headline_emphasized,
        edge_actions_visible: input.edge_actions_visible,
    });

    PanelRenderState {
        shared,
        layer_style,
    }
}

pub fn resolve_shared_body_height_decision(
    input: SharedBodyHeightDecisionInput,
) -> SharedBodyHeightDecision {
    let next_height = input.requested_height.max(0.0);
    let threshold = input.update_threshold.max(0.0);
    let should_update = !input
        .current_height
        .is_some_and(|current| (current - next_height).abs() < threshold);

    SharedBodyHeightDecision {
        next_height,
        should_update,
        should_rerender: should_update && input.has_snapshot,
    }
}

pub fn resolve_centered_top_frame(screen_frame: PanelRect, size: PanelSize) -> PanelRect {
    let snapped_width = size.width.max(1.0).round();
    let snapped_height = size.height.max(1.0).round();
    let top_edge = screen_frame.y + screen_frame.height;

    PanelRect {
        x: (screen_frame.x + ((screen_frame.width - snapped_width) / 2.0).max(0.0)).round(),
        y: (top_edge - snapped_height).round(),
        width: snapped_width,
        height: snapped_height,
    }
}

pub fn resolve_native_panel_host_frame(
    descriptor: PanelAnimationDescriptor,
    screen_frame: PanelRect,
    compact_width: f64,
    expanded_width: f64,
) -> PanelRect {
    let width = lerp(
        compact_width.max(1.0),
        expanded_width.max(1.0),
        descriptor.width_progress,
    );
    let height = descriptor
        .canvas_height
        .max(descriptor.visible_height)
        .max(1.0);

    resolve_centered_top_frame(screen_frame, PanelSize { width, height })
}

pub fn rects_nearly_equal(a: PanelRect, b: PanelRect, tolerance: f64) -> bool {
    let tolerance = tolerance.max(0.0);
    (a.x - b.x).abs() < tolerance
        && (a.y - b.y).abs() < tolerance
        && (a.width - b.width).abs() < tolerance
        && (a.height - b.height).abs() < tolerance
}

pub fn absolute_rect(parent_frame: PanelRect, local_frame: PanelRect) -> PanelRect {
    compose_local_rect(parent_frame, local_frame)
}

pub fn compose_local_rect(parent_frame: PanelRect, child_frame: PanelRect) -> PanelRect {
    PanelRect {
        x: parent_frame.x + child_frame.x,
        y: parent_frame.y + child_frame.y,
        width: child_frame.width,
        height: child_frame.height,
    }
}

pub fn point_in_rect(point: PanelPoint, rect: PanelRect) -> bool {
    point.x >= rect.x
        && point.x <= rect.x + rect.width
        && point.y >= rect.y
        && point.y <= rect.y + rect.height
}

pub fn resolve_panel_screen_has_camera_housing(top_area: PanelScreenTopArea) -> bool {
    let center_gap = resolve_panel_screen_center_gap(top_area);
    (top_area.auxiliary_left_width > 0.0 || top_area.auxiliary_right_width > 0.0)
        && center_gap > 40.0
}

pub fn resolve_panel_notch_width(top_area: PanelScreenTopArea) -> f64 {
    if top_area.auxiliary_left_width > 0.0 || top_area.auxiliary_right_width > 0.0 {
        return resolve_panel_screen_center_gap(top_area);
    }

    (top_area.screen_width * 0.18).clamp(160.0, 240.0)
}

pub fn resolve_panel_shell_width(input: PanelScreenWidthInput) -> f64 {
    if !resolve_panel_screen_has_camera_housing(input.top_area) {
        return resolve_panel_shell_width_for_non_camera_housing(
            input.compact_height,
            input.default_compact_width,
        );
    }

    let mascot_size = resolve_compact_mascot_size(input.compact_height);
    let compact_wing = mascot_size + 14.0;
    let notch_width = resolve_panel_notch_width(input.top_area);
    let screen_extra = (input.top_area.screen_width * 0.012).clamp(10.0, 22.0);
    let max_width = (input.top_area.screen_width - 24.0)
        .min(input.default_canvas_width)
        .max(input.default_compact_width);
    (notch_width + compact_wing * 2.0 + 10.0 + screen_extra)
        .clamp(input.default_compact_width, max_width)
}

pub fn resolve_panel_shell_width_for_non_camera_housing(
    compact_height: f64,
    default_compact_width: f64,
) -> f64 {
    let mascot_size = resolve_compact_mascot_size(compact_height);
    let minimum_content_width = mascot_size + 14.0 + 138.0;
    default_compact_width.max(minimum_content_width)
}

pub fn resolve_panel_expanded_width(input: PanelScreenWidthInput) -> f64 {
    if !resolve_panel_screen_has_camera_housing(input.top_area) {
        return input.default_expanded_width;
    }

    resolve_panel_expanded_width_for_camera_housing(
        resolve_panel_shell_width(input),
        input.expanded_width_delta,
        input.default_canvas_width,
    )
}

pub fn resolve_panel_expanded_width_for_camera_housing(
    compact_width: f64,
    expanded_width_delta: f64,
    default_canvas_width: f64,
) -> f64 {
    (compact_width + expanded_width_delta).clamp(compact_width, default_canvas_width)
}

pub fn resolve_panel_canvas_width(input: PanelScreenWidthInput) -> f64 {
    let compact_width = resolve_panel_shell_width(input);
    resolve_panel_expanded_width(input)
        .max(compact_width + 24.0)
        .max(input.default_canvas_width)
}

pub fn resolve_fallback_panel_expanded_width(
    fallback_width: f64,
    default_compact_width: f64,
) -> f64 {
    default_compact_width.min(fallback_width.max(1.0))
}

pub fn resolve_fallback_panel_canvas_width(fallback_width: f64, default_canvas_width: f64) -> f64 {
    fallback_width.max(default_canvas_width)
}

pub fn resolve_panel_layout(input: PanelLayoutInput) -> PanelLayout {
    let canvas_height = input.canvas_height.max(input.collapsed_height);
    let visible_height = input
        .visible_height
        .clamp(input.collapsed_height, canvas_height);
    let content_frame = PanelRect {
        x: 0.0,
        y: 0.0,
        width: input.metrics.panel_width,
        height: canvas_height,
    };
    let drop_offset = input.drop_distance * input.drop_progress;
    let panel_frame = resolve_centered_top_frame(
        input.screen_frame,
        PanelSize {
            width: content_frame.width,
            height: content_frame.height,
        },
    );
    let pill_frame = resolve_island_bar_frame(
        PanelSize {
            width: content_frame.width,
            height: content_frame.height,
        },
        input.bar_progress,
        input.metrics.compact_width,
        input.metrics.expanded_width,
        input.metrics.compact_height,
        drop_offset,
    );
    let expanded_frame = resolve_expanded_background_frame(
        PanelSize {
            width: content_frame.width,
            height: content_frame.height,
        },
        visible_height,
        input.bar_progress,
        input.height_progress,
        input.metrics.compact_width,
        input.metrics.expanded_width,
        input.metrics.compact_height,
        drop_offset,
        input.collapsed_height,
    );
    let cards_frame = resolve_expanded_cards_frame(
        expanded_frame,
        input.metrics.compact_height,
        input.content_top_gap,
        input.content_bottom_inset,
        input.cards_side_inset,
    );
    PanelLayout {
        panel_frame,
        content_frame,
        pill_frame,
        left_shoulder_frame: resolve_left_shoulder_frame(pill_frame, input.shoulder_size),
        right_shoulder_frame: resolve_right_shoulder_frame(pill_frame, input.shoulder_size),
        expanded_frame,
        cards_frame,
        separator_frame: resolve_expanded_separator_frame(
            expanded_frame,
            input.metrics.compact_height,
            input.separator_side_inset,
        ),
        shared_content_frame: absolute_rect(
            panel_frame,
            compose_local_rect(expanded_frame, cards_frame),
        ),
        shell_visible: input.bar_progress > 0.01 || input.height_progress > 0.01,
        separator_visibility: (input.height_progress.min(input.content_visibility) * 0.88)
            .clamp(0.0, 0.88),
    }
}

pub fn resolve_island_bar_frame(
    content_size: PanelSize,
    progress: f64,
    compact_width: f64,
    expanded_width: f64,
    compact_height: f64,
    drop_offset: f64,
) -> PanelRect {
    let width = lerp(compact_width, expanded_width, progress);
    PanelRect {
        x: (content_size.width - width) / 2.0,
        y: content_size.height - compact_height - drop_offset,
        width,
        height: compact_height,
    }
}

pub fn resolve_left_shoulder_frame(pill_frame: PanelRect, shoulder_size: f64) -> PanelRect {
    PanelRect {
        x: pill_frame.x - shoulder_size,
        y: pill_frame.y + pill_frame.height - shoulder_size,
        width: shoulder_size,
        height: shoulder_size,
    }
}

pub fn resolve_right_shoulder_frame(pill_frame: PanelRect, shoulder_size: f64) -> PanelRect {
    PanelRect {
        x: pill_frame.x + pill_frame.width,
        y: pill_frame.y + pill_frame.height - shoulder_size,
        width: shoulder_size,
        height: shoulder_size,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn resolve_expanded_background_frame(
    content_size: PanelSize,
    visible_height: f64,
    bar_progress: f64,
    height_progress: f64,
    compact_width: f64,
    expanded_width: f64,
    compact_height: f64,
    drop_offset: f64,
    collapsed_height: f64,
) -> PanelRect {
    let height_progress = height_progress.clamp(0.0, 1.0);
    let width = lerp(compact_width, expanded_width, bar_progress);
    let visible_height = visible_height
        .max(collapsed_height)
        .min(content_size.height.max(collapsed_height));
    let height = lerp(
        compact_height,
        (visible_height - drop_offset).max(compact_height),
        height_progress,
    );
    PanelRect {
        x: (content_size.width - width) / 2.0,
        y: content_size.height - drop_offset - height,
        width,
        height,
    }
}

pub fn resolve_expanded_cards_frame(
    container_frame: PanelRect,
    compact_height: f64,
    top_gap: f64,
    bottom_inset: f64,
    side_inset: f64,
) -> PanelRect {
    let body_height = (container_frame.height - compact_height - top_gap - bottom_inset).max(0.0);
    PanelRect {
        x: side_inset,
        y: bottom_inset,
        width: resolve_expanded_cards_width(container_frame.width, side_inset),
        height: body_height,
    }
}

pub fn resolve_expanded_separator_frame(
    container_frame: PanelRect,
    compact_height: f64,
    side_inset: f64,
) -> PanelRect {
    PanelRect {
        x: side_inset,
        y: (container_frame.height - compact_height - 0.5).max(0.0),
        width: (container_frame.width - (side_inset * 2.0)).max(0.0),
        height: 1.0,
    }
}

pub fn resolve_expanded_cards_width(container_width: f64, side_inset: f64) -> f64 {
    (container_width - (side_inset * 2.0)).max(0.0)
}

pub fn resolve_expanded_total_height(
    estimated_body_height: f64,
    shared_body_height: Option<f64>,
    compact_height: f64,
    top_gap: f64,
    bottom_inset: f64,
    max_body_height: f64,
) -> f64 {
    let body_height = shared_body_height
        .map(|shared_height| shared_height.max(estimated_body_height))
        .unwrap_or(estimated_body_height)
        .min(max_body_height);
    compact_height + top_gap + bottom_inset + body_height
}

pub fn resolve_panel_cards_visibility_progress(descriptor: PanelAnimationDescriptor) -> f64 {
    match descriptor.kind {
        PanelAnimationKind::Close => 1.0 - descriptor.cards_progress,
        PanelAnimationKind::Open | PanelAnimationKind::SurfaceSwitch => descriptor.cards_progress,
    }
    .clamp(0.0, 1.0)
}

pub fn resolve_panel_transition_canvas_height(
    start_height: f64,
    target_height: f64,
    collapsed_height: f64,
) -> f64 {
    start_height.max(target_height).max(collapsed_height)
}

pub fn resolve_next_stacked_card_frame(
    cursor_y: &mut f64,
    needs_gap: bool,
    height: f64,
    expanded_width: f64,
    card_gap: f64,
    card_overhang: f64,
) -> Option<PanelRect> {
    if needs_gap {
        *cursor_y -= card_gap;
    }
    if *cursor_y < height {
        return None;
    }

    *cursor_y -= height;
    Some(PanelRect {
        x: -card_overhang,
        y: *cursor_y,
        width: expanded_width + (card_overhang * 2.0),
        height,
    })
}

fn lerp(start: f64, end: f64, progress: f64) -> f64 {
    start + ((end - start) * progress.clamp(0.0, 1.0))
}

fn clamp_f64(value: f64, min: f64, max: f64) -> f64 {
    if max < min {
        return min;
    }
    value.clamp(min, max)
}

fn resolve_panel_screen_center_gap(top_area: PanelScreenTopArea) -> f64 {
    (top_area.screen_width - top_area.auxiliary_left_width - top_area.auxiliary_right_width)
        .max(0.0)
}

fn resolve_compact_mascot_size(compact_height: f64) -> f64 {
    (compact_height - 6.0).clamp(20.0, 27.0)
}

#[cfg(test)]
mod tests {
    use super::{clamp_panel_rect_to_bounds, PanelRect};

    #[test]
    fn panel_rect_clamp_moves_offscreen_rect_into_bounds() {
        assert_eq!(
            clamp_panel_rect_to_bounds(
                PanelRect {
                    x: -200.0,
                    y: 900.0,
                    width: 320.0,
                    height: 80.0,
                },
                PanelRect {
                    x: 0.0,
                    y: 0.0,
                    width: 800.0,
                    height: 600.0,
                },
            ),
            PanelRect {
                x: 0.0,
                y: 520.0,
                width: 320.0,
                height: 80.0,
            }
        );
    }

    #[test]
    fn panel_rect_clamp_shrinks_rect_larger_than_bounds() {
        assert_eq!(
            clamp_panel_rect_to_bounds(
                PanelRect {
                    x: 25.0,
                    y: 25.0,
                    width: 1000.0,
                    height: 900.0,
                },
                PanelRect {
                    x: -100.0,
                    y: -50.0,
                    width: 640.0,
                    height: 480.0,
                },
            ),
            PanelRect {
                x: -100.0,
                y: -50.0,
                width: 640.0,
                height: 480.0,
            }
        );
    }
}
