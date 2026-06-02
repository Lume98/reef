use crate::dpi::{DpiScale, PhysicalRect};

#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Direct2D::ID2D1DCRenderTarget;

#[cfg(target_os = "windows")]
pub struct PaintSurface {
    dib: DibSection,
    target: ID2D1DCRenderTarget,
}

#[cfg(target_os = "windows")]
impl PaintSurface {
    pub fn new(
        factory: &windows::Win32::Graphics::Direct2D::ID2D1Factory,
        physical: PhysicalRect,
        dpi_scale: DpiScale,
    ) -> Result<Self, String> {
        use windows::Win32::Graphics::{
            Direct2D::{
                Common::{D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_PIXEL_FORMAT},
                D2D1_FEATURE_LEVEL_DEFAULT, D2D1_RENDER_TARGET_PROPERTIES,
                D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_RENDER_TARGET_USAGE_NONE,
            },
            Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM,
        };

        let dib = DibSection::new(physical.width, physical.height)?;
        let target_props = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: (96.0 * dpi_scale.scale) as f32,
            dpiY: (96.0 * dpi_scale.scale) as f32,
            usage: D2D1_RENDER_TARGET_USAGE_NONE,
            minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
        };
        let target = unsafe { factory.CreateDCRenderTarget(&target_props) }
            .map_err(|e| e.to_string())?;
        Ok(Self { dib, target })
    }

    pub fn begin_draw(&self, physical: PhysicalRect) -> Result<(), String> {
        use windows::Win32::Foundation::RECT;
        use windows::Win32::Graphics::Direct2D::{
            Common::D2D1_COLOR_F, D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
            D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE,
        };
        use windows::Win32::Graphics::Gdi::HDC;

        let bind_rect = RECT {
            left: 0,
            top: 0,
            right: physical.width,
            bottom: physical.height,
        };
        unsafe {
            self.target
                .BindDC(HDC(self.dib.hdc), &bind_rect)
                .map_err(|e| e.to_string())?;
            self.target
                .SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);
            self.target
                .SetTextAntialiasMode(D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE);
            self.target.BeginDraw();
            self.target.Clear(Some(&D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }));
        }
        Ok(())
    }

    pub fn target(&self) -> &ID2D1DCRenderTarget {
        &self.target
    }

    pub fn end_draw(&self) -> Result<(), String> {
        unsafe {
            self.target
                .EndDraw(None, None)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn update_layered_window(
        &self,
        hwnd: isize,
        position: PhysicalRect,
    ) -> Result<(), String> {
        self.dib.update_layered_window(hwnd, position.x, position.y)
    }
}

#[cfg(target_os = "windows")]
#[derive(Debug)]
struct DibSection {
    hdc: windows_sys::Win32::Graphics::Gdi::HDC,
    bitmap: windows_sys::Win32::Graphics::Gdi::HBITMAP,
    previous: windows_sys::Win32::Graphics::Gdi::HGDIOBJ,
    width: i32,
    height: i32,
}

#[cfg(target_os = "windows")]
impl DibSection {
    fn new(width: i32, height: i32) -> Result<Self, String> {
        use std::ptr;
        use windows_sys::Win32::Graphics::Gdi::{
            CreateCompatibleDC, CreateDIBSection, SelectObject, BITMAPINFO, BITMAPINFOHEADER,
            BI_RGB, DIB_RGB_COLORS, RGBQUAD,
        };

        let width = width.max(1);
        let height = height.max(1);
        unsafe {
            let hdc = CreateCompatibleDC(ptr::null_mut());
            if hdc.is_null() {
                return Err("CreateCompatibleDC failed".to_string());
            }
            let bitmap_info = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB,
                    biSizeImage: (width * height * 4) as u32,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [RGBQUAD {
                    rgbBlue: 0,
                    rgbGreen: 0,
                    rgbRed: 0,
                    rgbReserved: 0,
                }],
            };
            let mut bits = ptr::null_mut();
            let bitmap = CreateDIBSection(
                hdc,
                &bitmap_info,
                DIB_RGB_COLORS,
                &mut bits,
                ptr::null_mut(),
                0,
            );
            if bitmap.is_null() || bits.is_null() {
                windows_sys::Win32::Graphics::Gdi::DeleteDC(hdc);
                return Err("CreateDIBSection failed".to_string());
            }
            let previous = SelectObject(hdc, bitmap as _);
            Ok(Self {
                hdc,
                bitmap,
                previous,
                width,
                height,
            })
        }
    }

    fn update_layered_window(&self, hwnd: isize, x: i32, y: i32) -> Result<(), String> {
        use windows_sys::Win32::{
            Foundation::{POINT, SIZE},
            Graphics::Gdi::{AC_SRC_ALPHA, AC_SRC_OVER, BLENDFUNCTION},
            UI::WindowsAndMessaging::{UpdateLayeredWindow, ULW_ALPHA},
        };

        let destination = POINT { x, y };
        let size = SIZE {
            cx: self.width,
            cy: self.height,
        };
        let source = POINT { x: 0, y: 0 };
        let blend = BLENDFUNCTION {
            BlendOp: AC_SRC_OVER as u8,
            BlendFlags: 0,
            SourceConstantAlpha: 255,
            AlphaFormat: AC_SRC_ALPHA as u8,
        };
        let ok = unsafe {
            UpdateLayeredWindow(
                hwnd as _,
                std::ptr::null_mut(),
                &destination,
                &size,
                self.hdc,
                &source,
                0,
                &blend,
                ULW_ALPHA,
            )
        };
        if ok == 0 {
            return Err(format!(
                "UpdateLayeredWindow failed: {}",
                std::io::Error::last_os_error()
            ));
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
impl Drop for DibSection {
    fn drop(&mut self) {
        unsafe {
            windows_sys::Win32::Graphics::Gdi::SelectObject(self.hdc, self.previous);
            windows_sys::Win32::Graphics::Gdi::DeleteObject(self.bitmap as _);
            windows_sys::Win32::Graphics::Gdi::DeleteDC(self.hdc);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub struct PaintSurface;

#[cfg(not(target_os = "windows"))]
impl PaintSurface {
    pub fn new(_factory: &(), _physical: PhysicalRect, _dpi_scale: DpiScale) -> Result<Self, String> {
        Ok(Self)
    }
    pub fn begin_draw(&self, _physical: PhysicalRect) -> Result<(), String> { Ok(()) }
    pub fn target(&self) -> &() { &() }
    pub fn end_draw(&self) -> Result<(), String> { Ok(()) }
    pub fn update_layered_window(&self, _hwnd: isize, _position: PhysicalRect) -> Result<(), String> { Ok(()) }
}

#[cfg(target_os = "windows")]
mod d2d_helpers {
    use reef_core::{color::Color, geometry::{Point, Rect}};

    pub fn color_f(color: Color, alpha: f64) -> windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F {
        windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F {
            r: color.r as f32 / 255.0,
            g: color.g as f32 / 255.0,
            b: color.b as f32 / 255.0,
            a: alpha.clamp(0.0, 1.0) as f32,
        }
    }

    pub fn rect_f(rect: Rect) -> windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F {
        windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F {
            left: rect.x as f32,
            top: rect.y as f32,
            right: (rect.x + rect.width) as f32,
            bottom: (rect.y + rect.height) as f32,
        }
    }

    pub fn ellipse_f(rect: Rect) -> windows::Win32::Graphics::Direct2D::D2D1_ELLIPSE {
        windows::Win32::Graphics::Direct2D::D2D1_ELLIPSE {
            point: windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2F {
                x: (rect.x + rect.width / 2.0) as f32,
                y: (rect.y + rect.height / 2.0) as f32,
            },
            radiusX: (rect.width / 2.0) as f32,
            radiusY: (rect.height / 2.0) as f32,
        }
    }

    pub fn point_f(point: Point) -> windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2F {
        windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2F {
            x: point.x as f32,
            y: point.y as f32,
        }
    }
}

#[cfg(target_os = "windows")]
pub use d2d_helpers::*;
