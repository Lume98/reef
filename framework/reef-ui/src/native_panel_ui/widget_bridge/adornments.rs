use crate::native_panel_core::{PanelChromeVisibilitySpec, PanelPoint, PanelRect};
use crate::native_panel_scene::SceneMascotPose;
use crate::native_panel_ui::visual_plan::NativePanelPaintInput;
use reef_core::color::Color;
use reef_core::geometry::Rect;
use reef_theme::compact_bar as compact_theme;
use reef_widgets::compact_bar::{CompactShoulder, CompletionGlow, ShoulderSide};
use reef_widgets::mascot::{CompletionBadge, MascotPose, MascotWidget};

pub(super) fn mascot(
    input: &NativePanelPaintInput,
    chrome: PanelChromeVisibilitySpec,
) -> Option<MascotWidget> {
    if input.mascot_pose == SceneMascotPose::Hidden || !chrome.collapsed_mascot_visible {
        return None;
    }

    let compact_frame = non_zero_panel_rect(input.compact_bar_frame).unwrap_or(input.panel_frame);
    let center = mascot_center(compact_frame);
    let mut mascot = MascotWidget::new(center.x, center.y, 11.0)
        .pose(mascot_pose(input.mascot_pose))
        .alpha(if chrome.collapsed_mascot_visible {
            1.0
        } else {
            0.0
        });
    mascot.elapsed_ms = input.mascot_elapsed_ms;

    if let Some(frame) = input.mascot_motion_frame {
        mascot.offset_x = frame.offset_x;
        mascot.offset_y = frame.offset_y;
        mascot.scale_x = frame.scale_x;
        mascot.scale_y = frame.scale_y;
        mascot.alpha = frame.shell_alpha;
        mascot.shadow_opacity = frame.shadow_opacity;
        mascot.shadow_radius = frame.shadow_radius;
    }

    if input.completion_count > 0 {
        mascot.completion_badge = Some(CompletionBadge::new(
            center.x,
            center.y - 14.0,
            input.completion_count,
        ));
    }

    Some(mascot)
}

pub(super) fn glow(input: &NativePanelPaintInput) -> Option<CompletionGlow> {
    input.glow_visible.then(|| CompletionGlow {
        frame: rect_from_panel(input.compact_bar_frame),
        base_opacity: input.glow_opacity,
        elapsed_ms: input.mascot_elapsed_ms,
    })
}

pub(super) fn shoulder_left(input: &NativePanelPaintInput) -> Option<CompactShoulder> {
    shoulder(
        input.left_shoulder_frame,
        ShoulderSide::Left,
        input.shoulder_progress,
    )
}

pub(super) fn shoulder_right(input: &NativePanelPaintInput) -> Option<CompactShoulder> {
    shoulder(
        input.right_shoulder_frame,
        ShoulderSide::Right,
        input.shoulder_progress,
    )
}

fn shoulder(frame: PanelRect, side: ShoulderSide, progress: f64) -> Option<CompactShoulder> {
    non_zero_panel_rect(frame).map(|frame| CompactShoulder {
        frame: rect_from_panel(frame),
        side,
        progress: progress.clamp(0.0, 1.0),
        fill_color: Color::from(compact_theme::FILL),
        border_color: Color::from(compact_theme::BORDER),
    })
}

fn mascot_pose(pose: SceneMascotPose) -> MascotPose {
    match pose {
        SceneMascotPose::Hidden => MascotPose::Hidden,
        SceneMascotPose::Idle => MascotPose::Idle,
        SceneMascotPose::Running => MascotPose::Running,
        SceneMascotPose::Approval => MascotPose::Approval,
        SceneMascotPose::Question => MascotPose::Question,
        SceneMascotPose::MessageBubble => MascotPose::MessageBubble,
        SceneMascotPose::Complete => MascotPose::Complete,
        SceneMascotPose::Sleepy => MascotPose::Sleepy,
        SceneMascotPose::WakeAngry => MascotPose::WakeAngry,
    }
}

fn mascot_center(compact_frame: PanelRect) -> PanelPoint {
    PanelPoint {
        x: compact_frame.x + 26.0,
        y: compact_frame.y + compact_frame.height / 2.0,
    }
}

fn non_zero_panel_rect(rect: PanelRect) -> Option<PanelRect> {
    (rect.width > 0.0 && rect.height > 0.0).then_some(rect)
}

fn rect_from_panel(rect: PanelRect) -> Rect {
    Rect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}
