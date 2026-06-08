use crate::presentation::rendering::native_panel_submit_visual_plan;
use reef::core::{
    color::Color,
    geometry::{Point, Rect},
};
use reef::draw::primitive::{DrawPlan, DrawPrimitive, PathSegment, TextAlignment, TextWeight};

use crate::runtime::facade::visual::resolve_native_panel_visual_plan;
use crate::state::PanelRect;

use super::{d2d_painter::WindowsPanelPainter, window_shell::WindowsPanelShellPaintJob};

#[cfg(all(windows, not(test)))]
thread_local! {
    static DIRECT2D_WINDOWS_PANEL_PAINTER:
        std::cell::RefCell<Option<super::d2d_painter::Direct2DWindowsPanelPainter>> =
            const { std::cell::RefCell::new(None) };
}

pub(super) const WINDOWS_PANEL_TRANSPARENT_COLOR_KEY: u32 = 0x00FF00FF;
pub(super) type WindowsPanelPaintColor = Color;
pub(super) type WindowsPanelPaintPlan = DrawPlan;
pub(super) type WindowsPanelPaintPrimitive = DrawPrimitive;

#[derive(Default)]
struct WindowsPanelFrameSubmissionRecorder;

impl reef::draw::draw_backend::DrawBackend for WindowsPanelFrameSubmissionRecorder {
    type Error = String;

    fn submit_frame(
        &mut self,
        _submission: &reef::draw::draw_backend::FrameSubmission,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum WindowsPanelPainterBackend {
    Direct2D,
    GdiFallback,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum WindowsPanelPaintOperation {
    PushClip {
        frame: Rect,
    },
    PopClip,
    FillRoundRect {
        frame: Rect,
        radius: f64,
        color: WindowsPanelPaintColor,
        alpha: f64,
    },
    FillRect {
        frame: Rect,
        color: WindowsPanelPaintColor,
        alpha: f64,
    },
    FillEllipse {
        frame: Rect,
        color: WindowsPanelPaintColor,
        alpha: f64,
    },
    StrokeLine {
        from: Point,
        to: Point,
        color: WindowsPanelPaintColor,
        width: f64,
        alpha: f64,
    },
    DrawText {
        frame: Rect,
        text: String,
        color: WindowsPanelPaintColor,
        size: i32,
        weight: TextWeight,
        alignment: TextAlignment,
        alpha: f64,
    },
    FillStrokedRoundRect {
        frame: Rect,
        radius: f64,
        fill: WindowsPanelPaintColor,
        stroke_color: WindowsPanelPaintColor,
        stroke_width: f64,
        alpha: f64,
    },
    DrawImage {
        key: String,
        source_rect: Rect,
        frame: Rect,
        opacity: f64,
    },
    DrawNineSliceImage {
        key: String,
        frame: Rect,
        slice_left: f64,
        slice_right: f64,
        slice_top: f64,
        slice_bottom: f64,
        opacity: f64,
    },
    FillPath {
        segments: Vec<PathSegment>,
        fill: WindowsPanelPaintColor,
        alpha: f64,
    },
    DrawSpriteImage {
        key: String,
        source_rect: Rect,
        frame: Rect,
        opacity: f64,
    },
}

const WINDOWS_PANEL_HIT_TEST_BLOCKER_ALPHA: f64 = 1.0 / 255.0;

pub(super) fn resolve_windows_panel_paint_plan(
    job: &WindowsPanelShellPaintJob,
) -> WindowsPanelPaintPlan {
    #[cfg(test)]
    std::env::set_var("ECHOISLAND_MASCOT_SPRITE", "0");
    resolve_native_panel_visual_plan(job)
}

pub(super) fn windows_panel_preferred_painter_backend() -> WindowsPanelPainterBackend {
    WindowsPanelPainterBackend::Direct2D
}

pub(super) fn windows_panel_composition_mode_for_preferred_painter(
) -> super::layered_window::WindowsLayeredWindowCompositionMode {
    match windows_panel_preferred_painter_backend() {
        WindowsPanelPainterBackend::Direct2D => {
            super::layered_window::WindowsLayeredWindowCompositionMode::PerPixelAlpha
        }
        WindowsPanelPainterBackend::GdiFallback => {
            super::layered_window::WindowsLayeredWindowCompositionMode::GdiColorKeyFallback
        }
    }
}

pub(super) fn resolve_windows_panel_paint_operations(
    plan: &WindowsPanelPaintPlan,
) -> Vec<WindowsPanelPaintOperation> {
    if plan.hidden {
        return Vec::new();
    }
    plan.primitives
        .iter()
        .map(windows_panel_paint_operation_from_primitive)
        .collect()
}

pub(super) fn resolve_windows_panel_hit_test_blocker_operations(
    job: &WindowsPanelShellPaintJob,
) -> Vec<WindowsPanelPaintOperation> {
    if job.display_mode != crate::presentation::presentation::NativePanelVisualDisplayMode::Expanded
    {
        return Vec::new();
    }
    let mut primitives = Vec::new();
    push_hit_test_blocker_primitive(&mut primitives, job.shell_frame);
    let content_top = job.content_frame.y + job.content_frame.height;
    let gap_y = job.shell_frame.y + job.shell_frame.height;
    push_hit_test_blocker_primitive(
        &mut primitives,
        PanelRect {
            x: job.shell_frame.x,
            y: gap_y,
            width: job.shell_frame.width,
            height: (content_top - gap_y).max(0.0),
        },
    );
    primitives
        .iter()
        .map(windows_panel_paint_operation_from_primitive)
        .collect()
}

fn push_hit_test_blocker_primitive(primitives: &mut Vec<DrawPrimitive>, frame: PanelRect) {
    if frame.width <= 0.0 || frame.height <= 0.0 {
        return;
    }
    primitives.push(DrawPrimitive::Rect {
        frame: rect_from_panel_rect_model(frame),
        color: Color::BLACK,
        alpha: WINDOWS_PANEL_HIT_TEST_BLOCKER_ALPHA,
    });
}

pub(super) fn windows_panel_paint_operation_from_primitive(
    primitive: &WindowsPanelPaintPrimitive,
) -> WindowsPanelPaintOperation {
    match primitive {
        DrawPrimitive::ClipStart { frame } => {
            WindowsPanelPaintOperation::PushClip { frame: *frame }
        }
        DrawPrimitive::ClipEnd => WindowsPanelPaintOperation::PopClip,
        DrawPrimitive::RoundRect {
            frame,
            radius,
            color,
            alpha,
        } => WindowsPanelPaintOperation::FillRoundRect {
            frame: *frame,
            radius: *radius,
            color: *color,
            alpha: *alpha,
        },
        DrawPrimitive::Rect {
            frame,
            color,
            alpha,
        } => WindowsPanelPaintOperation::FillRect {
            frame: *frame,
            color: *color,
            alpha: *alpha,
        },
        DrawPrimitive::Ellipse {
            frame,
            color,
            alpha,
        } => WindowsPanelPaintOperation::FillEllipse {
            frame: *frame,
            color: *color,
            alpha: *alpha,
        },
        DrawPrimitive::StrokeLine {
            from,
            to,
            color,
            width,
            alpha,
        } => WindowsPanelPaintOperation::StrokeLine {
            from: *from,
            to: *to,
            color: *color,
            width: *width,
            alpha: *alpha,
        },
        DrawPrimitive::Text {
            frame,
            text,
            color,
            size,
            weight,
            alignment,
            alpha,
        } => WindowsPanelPaintOperation::DrawText {
            frame: *frame,
            text: text.clone(),
            color: *color,
            size: *size,
            weight: *weight,
            alignment: *alignment,
            alpha: *alpha,
        },
        DrawPrimitive::Image {
            key,
            source_rect,
            frame,
            opacity,
        } => WindowsPanelPaintOperation::DrawImage {
            key: key.clone(),
            source_rect: *source_rect,
            frame: *frame,
            opacity: *opacity,
        },
        DrawPrimitive::StrokedRoundRect {
            frame,
            radius,
            fill,
            stroke,
            stroke_width,
            alpha,
        } => WindowsPanelPaintOperation::FillStrokedRoundRect {
            frame: *frame,
            radius: *radius,
            fill: *fill,
            stroke_color: *stroke,
            stroke_width: *stroke_width,
            alpha: *alpha,
        },
        DrawPrimitive::NineSliceImage {
            key,
            frame,
            slice_left,
            slice_right,
            slice_top,
            slice_bottom,
            opacity,
        } => WindowsPanelPaintOperation::DrawNineSliceImage {
            key: key.clone(),
            frame: *frame,
            slice_left: *slice_left,
            slice_right: *slice_right,
            slice_top: *slice_top,
            slice_bottom: *slice_bottom,
            opacity: *opacity,
        },
        DrawPrimitive::Path {
            segments,
            fill,
            alpha,
        } => WindowsPanelPaintOperation::FillPath {
            segments: segments.clone(),
            fill: *fill,
            alpha: *alpha,
        },
        DrawPrimitive::SpriteImage {
            key,
            source_rect,
            frame,
            opacity,
        } => WindowsPanelPaintOperation::DrawSpriteImage {
            key: key.clone(),
            source_rect: *source_rect,
            frame: *frame,
            opacity: *opacity,
        },
    }
}

fn rect_from_panel_rect_model(rect: PanelRect) -> Rect {
    Rect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}

pub(super) fn paint_windows_panel_job(
    raw_window_handle: Option<isize>,
    job: &WindowsPanelShellPaintJob,
) -> Result<WindowsPanelPaintPlan, String> {
    #[cfg(all(windows, not(test)))]
    {
        match windows_panel_preferred_painter_backend() {
            WindowsPanelPainterBackend::Direct2D => {
                paint_windows_panel_job_with_direct2d(raw_window_handle, job)
            }
            WindowsPanelPainterBackend::GdiFallback => {
                paint_windows_panel_job_with_gdi(raw_window_handle, job)
            }
        }
    }

    #[cfg(any(not(windows), test))]
    {
        let _ = raw_window_handle;
        let mut painter = super::d2d_painter::PlanOnlyWindowsPanelPainter;
        let plan = painter.paint(job)?;
        let mut recorder = WindowsPanelFrameSubmissionRecorder;
        let _ = native_panel_submit_visual_plan(&mut recorder, &plan);
        Ok(plan)
    }
}

#[cfg(all(windows, not(test)))]
fn paint_windows_panel_job_with_direct2d(
    raw_window_handle: Option<isize>,
    job: &WindowsPanelShellPaintJob,
) -> Result<WindowsPanelPaintPlan, String> {
    DIRECT2D_WINDOWS_PANEL_PAINTER.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            *slot = Some(super::d2d_painter::Direct2DWindowsPanelPainter::new(
                raw_window_handle,
            )?);
        }
        let painter = slot
            .as_mut()
            .expect("Direct2D painter initialized when slot is Some");
        painter.set_raw_window_handle(raw_window_handle);
        painter.paint(job)
    })
}

#[cfg(all(windows, not(test)))]
pub(super) fn paint_windows_panel_job_with_gdi(
    _raw_window_handle: Option<isize>,
    job: &WindowsPanelShellPaintJob,
) -> Result<WindowsPanelPaintPlan, String> {
    let plan = resolve_windows_panel_paint_plan(job);
    let mut recorder = WindowsPanelFrameSubmissionRecorder;
    let _ = native_panel_submit_visual_plan(&mut recorder, &plan);
    Ok(plan)
}

#[cfg(all(windows, not(test)))]
fn color_ref(color: WindowsPanelPaintColor) -> u32 {
    color.r as u32 | ((color.g as u32) << 8) | ((color.b as u32) << 16)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef::draw::primitive::{DrawPrimitive, PathSegment, TextAlignment, TextWeight};

    #[test]
    fn text_operation_uses_explicit_frame() {
        let primitive = DrawPrimitive::Text {
            frame: Rect {
                x: 1.0,
                y: 2.0,
                width: 30.0,
                height: 14.0,
            },
            text: "Ready".to_string(),
            color: Color::WHITE,
            size: 12,
            weight: TextWeight::Semibold,
            alignment: TextAlignment::Center,
            alpha: 0.75,
        };
        let operation = windows_panel_paint_operation_from_primitive(&primitive);
        assert!(
            matches!(operation, WindowsPanelPaintOperation::DrawText { frame, text, weight, alignment, alpha, .. }
            if frame.height == 14.0 && text == "Ready" && weight == TextWeight::Semibold && alignment == TextAlignment::Center && alpha == 0.75)
        );
    }

    #[test]
    fn generic_shape_operations_map_without_business_variants() {
        let stroked =
            windows_panel_paint_operation_from_primitive(&DrawPrimitive::StrokedRoundRect {
                frame: Rect {
                    x: 1.0,
                    y: 2.0,
                    width: 3.0,
                    height: 4.0,
                },
                radius: 2.0,
                fill: Color::BLACK,
                stroke: Color::WHITE,
                stroke_width: 1.5,
                alpha: 1.0,
            });
        let path = windows_panel_paint_operation_from_primitive(&DrawPrimitive::Path {
            segments: vec![PathSegment::LineTo(Point { x: 0.0, y: 0.0 })],
            fill: Color::WHITE,
            alpha: 0.5,
        });
        assert!(matches!(
            stroked,
            WindowsPanelPaintOperation::FillStrokedRoundRect { .. }
        ));
        assert!(matches!(
            path,
            WindowsPanelPaintOperation::FillPath { alpha: 0.5, .. }
        ));
    }
}
