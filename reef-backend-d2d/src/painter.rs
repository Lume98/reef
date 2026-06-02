use reef_core::geometry::Rect;
use reef_render::{primitive::VisualPrimitive, render_backend::RenderBackend};

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
        primitives: &[VisualPrimitive],
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
    fn ensure_surface(&mut self, physical: PhysicalRect, dpi_scale: DpiScale) -> Result<(), String> {
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
        primitive: &VisualPrimitive,
    ) -> Result<(), String> {
        use windows::Win32::Graphics::Direct2D::{
            D2D1_ANTIALIAS_MODE_PER_PRIMITIVE, D2D1_ROUNDED_RECT,
        };

        match primitive {
            VisualPrimitive::ClipStart { frame } => {
                unsafe {
                    target.PushAxisAlignedClip(
                        &rect_f(*frame),
                        D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
                    );
                }
                Ok(())
            }
            VisualPrimitive::ClipEnd => {
                unsafe { target.PopAxisAlignedClip(); }
                Ok(())
            }
            VisualPrimitive::RoundRect {
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
            VisualPrimitive::Rect {
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
            VisualPrimitive::Ellipse {
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
            VisualPrimitive::StrokeLine {
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
                    target.DrawLine(
                        point_f(*from),
                        point_f(*to),
                        &brush,
                        *width as f32,
                        None,
                    );
                }
                Ok(())
            }
            VisualPrimitive::Text {
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
            VisualPrimitive::Image { .. } => {
                // Image rendering requires bitmap loading - skip for now
                Ok(())
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn create_text_format(
        &self,
        size: i32,
    ) -> Result<windows::Win32::Graphics::DirectWrite::IDWriteTextFormat, String> {
        use windows::Win32::Graphics::DirectWrite::{DWriteCreateFactory, DWRITE_FACTORY_TYPE_SHARED};
        use windows::core::PCWSTR;

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
        _primitives: &[VisualPrimitive],
        _window_rect: Rect,
    ) -> Result<(), String> {
        Ok(())
    }
}

impl RenderBackend for Direct2DPainter {
    type Error = String;

    fn submit_frame(
        &mut self,
        submission: &reef_render::render_backend::FrameSubmission,
    ) -> Result<(), Self::Error> {
        if submission.hidden {
            return Ok(());
        }
        Ok(())
    }
}

pub fn resolve_paint_operations(plan: &reef_render::primitive::VisualPlan) -> Vec<PaintOperation> {
    if plan.hidden {
        return Vec::new();
    }
    plan.primitives
        .iter()
        .map(|p| match p {
            VisualPrimitive::ClipStart { frame } => PaintOperation::PushClip { frame: *frame },
            VisualPrimitive::ClipEnd => PaintOperation::PopClip,
            VisualPrimitive::RoundRect {
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
            VisualPrimitive::Rect {
                frame,
                color,
                alpha,
            } => PaintOperation::FillRect {
                frame: *frame,
                color: *color,
                alpha: *alpha,
            },
            VisualPrimitive::Ellipse {
                frame,
                color,
                alpha,
            } => PaintOperation::FillEllipse {
                frame: *frame,
                color: *color,
                alpha: *alpha,
            },
            VisualPrimitive::StrokeLine {
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
            VisualPrimitive::Text {
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
            VisualPrimitive::Image {
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
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
pub enum PaintOperation {
    PushClip { frame: Rect },
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
        weight: reef_render::primitive::FontWeight,
        alignment: reef_render::primitive::TextAlignment,
        alpha: f64,
    },
    DrawImage {
        key: String,
        source_rect: Rect,
        frame: Rect,
        opacity: f64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::{color::Color, geometry::{Point, Rect}};
    use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

    #[test]
    fn painter_handles_empty_plan() {
        let painter = Direct2DPainter::new();
        assert!(painter.surface.is_none());
    }

    #[test]
    fn resolve_paint_operations_maps_all_variants() {
        let plan = reef_render::primitive::VisualPlan {
            hidden: false,
            primitives: vec![
                VisualPrimitive::ClipStart {
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 100.0,
                        height: 50.0,
                    },
                },
                VisualPrimitive::ClipEnd,
                VisualPrimitive::RoundRect {
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
                VisualPrimitive::Rect {
                    frame: Rect {
                        x: 5.0,
                        y: 5.0,
                        width: 90.0,
                        height: 30.0,
                    },
                    color: Color::BLACK,
                    alpha: 0.5,
                },
                VisualPrimitive::Ellipse {
                    frame: Rect {
                        x: 10.0,
                        y: 10.0,
                        width: 20.0,
                        height: 20.0,
                    },
                    color: Color::WHITE,
                    alpha: 1.0,
                },
                VisualPrimitive::StrokeLine {
                    from: Point { x: 0.0, y: 0.0 },
                    to: Point {
                        x: 100.0,
                        y: 100.0,
                    },
                    color: Color::WHITE,
                    width: 1.0,
                    alpha: 1.0,
                },
                VisualPrimitive::Image {
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
            ],
        };
        let ops = resolve_paint_operations(&plan);
        assert_eq!(ops.len(), 7);
    }

    #[test]
    fn resolve_paint_operations_returns_empty_for_hidden() {
        let plan = reef_render::primitive::VisualPlan {
            hidden: true,
            primitives: vec![VisualPrimitive::ClipEnd],
        };
        assert!(resolve_paint_operations(&plan).is_empty());
    }
}
