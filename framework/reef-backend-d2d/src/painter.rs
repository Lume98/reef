use reef_core::geometry::Rect;
use reef_draw::{
    draw_backend::DrawBackend,
    primitive::{DrawPrimitive, PathSegment},
};

use crate::dpi::{DpiScale, PhysicalRect};
use crate::surface::PaintSurface;

#[cfg(target_os = "windows")]
use crate::surface::{color_f, ellipse_f, point_f, rect_f};

#[derive(Default)]
pub struct Direct2DPainter {
    #[cfg(target_os = "windows")]
    surface: Option<PaintSurface>,
    hwnd: Option<isize>,
}

impl Direct2DPainter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_window(&mut self, hwnd: isize) {
        self.hwnd = Some(hwnd);
    }

    #[cfg(target_os = "windows")]
    pub fn render_to_window(
        &mut self,
        primitives: &[DrawPrimitive],
        window_rect: Rect,
    ) -> Result<(), String> {
        let hwnd = self.hwnd.ok_or("No window handle set")?;
        let dpi_scale = crate::dpi::window_dpi_scale(self.hwnd);
        let physical = dpi_scale.rect_to_physical(window_rect);

        self.ensure_surface(physical, dpi_scale)?;

        let surface = self.surface.as_ref().ok_or("Surface not initialized")?;
        surface.begin_draw(physical)?;

        let target = surface.target();
        for primitive in primitives {
            self.draw_primitive(target, primitive)?;
        }

        surface.end_draw()?;
        surface.update_layered_window(hwnd, physical)
    }

    #[cfg(target_os = "windows")]
    fn ensure_surface(
        &mut self,
        physical: PhysicalRect,
        dpi_scale: DpiScale,
    ) -> Result<(), String> {
        if self.surface.is_some() {
            return Ok(());
        }
        let factory = crate::factory::Direct2DFactory::shared()?;
        let d2d_factory = factory.factory().ok_or("D2D factory not initialized")?;
        self.surface = Some(PaintSurface::new(d2d_factory, physical, dpi_scale)?);
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn draw_primitive(
        &self,
        target: &windows::Win32::Graphics::Direct2D::ID2D1DCRenderTarget,
        primitive: &DrawPrimitive,
    ) -> Result<(), String> {
        use windows::Win32::Graphics::Direct2D::{
            D2D1_ANTIALIAS_MODE_PER_PRIMITIVE, D2D1_ROUNDED_RECT,
        };

        match primitive {
            DrawPrimitive::ClipStart { frame } => {
                unsafe {
                    target.PushAxisAlignedClip(&rect_f(*frame), D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);
                }
                Ok(())
            }
            DrawPrimitive::ClipEnd => {
                unsafe {
                    target.PopAxisAlignedClip();
                }
                Ok(())
            }
            DrawPrimitive::RoundRect {
                frame,
                radius,
                color,
                alpha,
            } => {
                let brush = unsafe {
                    target
                        .CreateSolidColorBrush(&color_f(*color, *alpha), None)
                        .map_err(|e| e.to_string())?
                };
                unsafe {
                    target.FillRoundedRectangle(
                        &D2D1_ROUNDED_RECT {
                            rect: rect_f(*frame),
                            radiusX: *radius as f32,
                            radiusY: *radius as f32,
                        },
                        &brush,
                    );
                }
                Ok(())
            }
            DrawPrimitive::Rect {
                frame,
                color,
                alpha,
            } => {
                let brush = unsafe {
                    target
                        .CreateSolidColorBrush(&color_f(*color, *alpha), None)
                        .map_err(|e| e.to_string())?
                };
                unsafe {
                    target.FillRectangle(&rect_f(*frame), &brush);
                }
                Ok(())
            }
            DrawPrimitive::Ellipse {
                frame,
                color,
                alpha,
            } => {
                let brush = unsafe {
                    target
                        .CreateSolidColorBrush(&color_f(*color, *alpha), None)
                        .map_err(|e| e.to_string())?
                };
                unsafe {
                    target.FillEllipse(&ellipse_f(*frame), &brush);
                }
                Ok(())
            }
            DrawPrimitive::StrokeLine {
                from,
                to,
                color,
                width,
                alpha,
            } => {
                let brush = unsafe {
                    target
                        .CreateSolidColorBrush(&color_f(*color, *alpha), None)
                        .map_err(|e| e.to_string())?
                };
                unsafe {
                    target.DrawLine(point_f(*from), point_f(*to), &brush, *width as f32, None);
                }
                Ok(())
            }
            DrawPrimitive::Text {
                origin,
                text,
                color,
                size,
                alpha,
                ..
            } => {
                let brush = unsafe {
                    target
                        .CreateSolidColorBrush(&color_f(*color, *alpha), None)
                        .map_err(|e| e.to_string())?
                };
                let wide: Vec<u16> = text.encode_utf16().collect();
                let text_rect = windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F {
                    left: origin.x as f32,
                    top: origin.y as f32,
                    right: (origin.x + 300.0) as f32,
                    bottom: (origin.y + *size as f64 + 10.0) as f32,
                };
                unsafe {
                    target.DrawText(
                        &wide,
                        &self.create_text_format(*size)?,
                        &text_rect,
                        &brush,
                        windows::Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS_NONE,
                        windows::Win32::Graphics::DirectWrite::DWRITE_MEASURING_MODE_NATURAL,
                    );
                }
                Ok(())
            }
            DrawPrimitive::Image { .. } => {
                // Image rendering requires bitmap loading - skip for now
                Ok(())
            }
            DrawPrimitive::StrokedRoundRect {
                frame,
                radius,
                fill,
                stroke,
                stroke_width,
                alpha,
            } => {
                let fill_brush = unsafe {
                    target
                        .CreateSolidColorBrush(&color_f(*fill, *alpha), None)
                        .map_err(|e| e.to_string())?
                };
                let stroke_brush = unsafe {
                    target
                        .CreateSolidColorBrush(&color_f(*stroke, *alpha), None)
                        .map_err(|e| e.to_string())?
                };
                let rounded = D2D1_ROUNDED_RECT {
                    rect: rect_f(*frame),
                    radiusX: *radius as f32,
                    radiusY: *radius as f32,
                };
                unsafe {
                    target.FillRoundedRectangle(&rounded, &fill_brush);
                    target.DrawRoundedRectangle(
                        &rounded,
                        &stroke_brush,
                        *stroke_width as f32,
                        None,
                    );
                }
                Ok(())
            }
            DrawPrimitive::NineSliceImage { .. } => {
                // Nine-slice image rendering requires bitmap loading - placeholder
                Ok(())
            }
            DrawPrimitive::Path {
                segments,
                fill,
                alpha,
            } => {
                if segments.is_empty() {
                    return Ok(());
                }
                use windows::Win32::Graphics::Direct2D::Common::{
                    D2D1_BEZIER_SEGMENT, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_CLOSED,
                };
                let brush = unsafe {
                    target
                        .CreateSolidColorBrush(&color_f(*fill, *alpha), None)
                        .map_err(|e| e.to_string())?
                };
                let factory = crate::factory::Direct2DFactory::shared()?;
                let d2d_factory = factory.factory().ok_or("D2D factory not initialized")?;
                let start_point = match &segments[0] {
                    PathSegment::LineTo(p) => *p,
                    PathSegment::CubicBezier { end, .. } => *end,
                };
                let geometry = unsafe {
                    let geo = d2d_factory
                        .CreatePathGeometry()
                        .map_err(|e| e.to_string())?;
                    let sink = geo.Open().map_err(|e| e.to_string())?;
                    sink.BeginFigure(point_f(start_point), D2D1_FIGURE_BEGIN_FILLED);
                    for seg in segments.iter().skip(1) {
                        match seg {
                            PathSegment::LineTo(p) => {
                                sink.AddLine(point_f(*p));
                            }
                            PathSegment::CubicBezier {
                                control1,
                                control2,
                                end,
                            } => {
                                sink.AddBezier(&D2D1_BEZIER_SEGMENT {
                                    point1: point_f(*control1),
                                    point2: point_f(*control2),
                                    point3: point_f(*end),
                                });
                            }
                        }
                    }
                    sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                    sink.Close().map_err(|e| e.to_string())?;
                    geo
                };
                unsafe {
                    target.FillGeometry(&geometry, &brush, None);
                }
                Ok(())
            }
            DrawPrimitive::SpriteImage { .. } => {
                // Sprite image rendering requires bitmap loading - placeholder
                Ok(())
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn create_text_format(
        &self,
        size: i32,
    ) -> Result<windows::Win32::Graphics::DirectWrite::IDWriteTextFormat, String> {
        use windows::core::PCWSTR;
        use windows::Win32::Graphics::DirectWrite::{
            DWriteCreateFactory, DWRITE_FACTORY_TYPE_SHARED,
        };

        let factory: windows::Win32::Graphics::DirectWrite::IDWriteFactory = unsafe {
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
                .map_err(|e| format!("DWriteCreateFactory: {e}"))?
        };

        let font_family: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
        let locale: Vec<u16> = "\0".encode_utf16().collect();
        unsafe {
            factory
                .CreateTextFormat(
                    PCWSTR(font_family.as_ptr()),
                    None,
                    windows::Win32::Graphics::DirectWrite::DWRITE_FONT_WEIGHT_NORMAL,
                    windows::Win32::Graphics::DirectWrite::DWRITE_FONT_STYLE_NORMAL,
                    windows::Win32::Graphics::DirectWrite::DWRITE_FONT_STRETCH_NORMAL,
                    size as f32,
                    PCWSTR(locale.as_ptr()),
                )
                .map_err(|e| format!("CreateTextFormat(size={size}): {e}"))
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn render_to_window(
        &mut self,
        _primitives: &[DrawPrimitive],
        _window_rect: Rect,
    ) -> Result<(), String> {
        Ok(())
    }
}

impl DrawBackend for Direct2DPainter {
    type Error = String;

    fn submit_frame(
        &mut self,
        submission: &reef_draw::draw_backend::FrameSubmission,
    ) -> Result<(), Self::Error> {
        if submission.hidden {
            return Ok(());
        }
        Ok(())
    }
}

pub fn resolve_paint_operations(plan: &reef_draw::primitive::DrawPlan) -> Vec<PaintOperation> {
    if plan.hidden {
        return Vec::new();
    }
    plan.primitives
        .iter()
        .map(|p| match p {
            DrawPrimitive::ClipStart { frame } => PaintOperation::PushClip { frame: *frame },
            DrawPrimitive::ClipEnd => PaintOperation::PopClip,
            DrawPrimitive::RoundRect {
                frame,
                radius,
                color,
                alpha,
            } => PaintOperation::FillRoundRect {
                frame: *frame,
                radius: *radius,
                color: *color,
                alpha: *alpha,
            },
            DrawPrimitive::Rect {
                frame,
                color,
                alpha,
            } => PaintOperation::FillRect {
                frame: *frame,
                color: *color,
                alpha: *alpha,
            },
            DrawPrimitive::Ellipse {
                frame,
                color,
                alpha,
            } => PaintOperation::FillEllipse {
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
            } => PaintOperation::StrokeLine {
                from: *from,
                to: *to,
                color: *color,
                width: *width,
                alpha: *alpha,
            },
            DrawPrimitive::Text {
                origin,
                max_width,
                text,
                color,
                size,
                weight,
                alignment,
                alpha,
            } => PaintOperation::DrawText {
                origin: *origin,
                max_width: *max_width,
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
            } => PaintOperation::DrawImage {
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
            } => PaintOperation::FillStrokedRoundRect {
                frame: *frame,
                radius: *radius,
                fill: *fill,
                stroke: *stroke,
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
            } => PaintOperation::DrawNineSliceImage {
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
            } => PaintOperation::FillPath {
                segments: segments.clone(),
                fill: *fill,
                alpha: *alpha,
            },
            DrawPrimitive::SpriteImage {
                key,
                source_rect,
                frame,
                opacity,
            } => PaintOperation::DrawSpriteImage {
                key: key.clone(),
                source_rect: *source_rect,
                frame: *frame,
                opacity: *opacity,
            },
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
pub enum PaintOperation {
    PushClip {
        frame: Rect,
    },
    PopClip,
    FillRoundRect {
        frame: Rect,
        radius: f64,
        color: reef_core::color::Color,
        alpha: f64,
    },
    FillRect {
        frame: Rect,
        color: reef_core::color::Color,
        alpha: f64,
    },
    FillEllipse {
        frame: Rect,
        color: reef_core::color::Color,
        alpha: f64,
    },
    StrokeLine {
        from: reef_core::geometry::Point,
        to: reef_core::geometry::Point,
        color: reef_core::color::Color,
        width: f64,
        alpha: f64,
    },
    DrawText {
        origin: reef_core::geometry::Point,
        max_width: f64,
        text: String,
        color: reef_core::color::Color,
        size: i32,
        weight: reef_draw::primitive::TextWeight,
        alignment: reef_draw::primitive::TextAlignment,
        alpha: f64,
    },
    DrawImage {
        key: String,
        source_rect: Rect,
        frame: Rect,
        opacity: f64,
    },
    FillStrokedRoundRect {
        frame: Rect,
        radius: f64,
        fill: reef_core::color::Color,
        stroke: reef_core::color::Color,
        stroke_width: f64,
        alpha: f64,
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
        segments: Vec<reef_draw::primitive::PathSegment>,
        fill: reef_core::color::Color,
        alpha: f64,
    },
    DrawSpriteImage {
        key: String,
        source_rect: Rect,
        frame: Rect,
        opacity: f64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::{
        color::Color,
        geometry::{Point, Rect, Size},
    };
    use reef_draw::primitive::DrawPrimitive;

    #[test]
    fn painter_handles_empty_plan() {
        let painter = Direct2DPainter::new();
        assert!(painter.surface.is_none());
    }

    #[test]
    fn resolve_paint_operations_maps_all_variants() {
        let mut plan = reef_draw::primitive::DrawPlan::with_viewport(Size {
            width: 100.0,
            height: 100.0,
        });
        plan.primitives = vec![
                DrawPrimitive::ClipStart {
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 100.0,
                        height: 50.0,
                    },
                },
                DrawPrimitive::ClipEnd,
                DrawPrimitive::RoundRect {
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 100.0,
                        height: 40.0,
                    },
                    radius: 20.0,
                    color: Color::rgb(18, 18, 22),
                    alpha: 1.0,
                },
                DrawPrimitive::Rect {
                    frame: Rect {
                        x: 5.0,
                        y: 5.0,
                        width: 90.0,
                        height: 30.0,
                    },
                    color: Color::BLACK,
                    alpha: 0.5,
                },
                DrawPrimitive::Ellipse {
                    frame: Rect {
                        x: 10.0,
                        y: 10.0,
                        width: 20.0,
                        height: 20.0,
                    },
                    color: Color::WHITE,
                    alpha: 1.0,
                },
                DrawPrimitive::StrokeLine {
                    from: Point { x: 0.0, y: 0.0 },
                    to: Point { x: 100.0, y: 100.0 },
                    color: Color::WHITE,
                    width: 1.0,
                    alpha: 1.0,
                },
                DrawPrimitive::Image {
                    key: "test.png".to_string(),
                    source_rect: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 50.0,
                        height: 50.0,
                    },
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 50.0,
                        height: 50.0,
                    },
                    opacity: 1.0,
                },
        ];
        let ops = resolve_paint_operations(&plan);
        assert_eq!(ops.len(), 7);
    }

    #[test]
    fn resolve_paint_operations_returns_empty_for_hidden() {
        let mut plan = reef_draw::primitive::DrawPlan::with_viewport(Size {
            width: 1.0,
            height: 1.0,
        });
        plan.hidden = true;
        plan.primitives = vec![DrawPrimitive::ClipEnd];
        assert!(resolve_paint_operations(&plan).is_empty());
    }
}
