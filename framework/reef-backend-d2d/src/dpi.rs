use reef_core::geometry::{Point, Rect};

const BASE_DPI: u32 = 96;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DpiScale {
    pub scale: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysicalRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Default for DpiScale {
    fn default() -> Self {
        Self::from_scale(1.0)
    }
}

impl DpiScale {
    pub fn from_scale(scale: f64) -> Self {
        if scale.is_finite() && scale > 0.0 {
            Self { scale }
        } else {
            Self { scale: 1.0 }
        }
    }

    pub fn from_dpi(dpi: u32) -> Self {
        if dpi == 0 {
            return Self::default();
        }
        Self::from_scale(dpi as f64 / BASE_DPI as f64)
    }

    pub fn logical_to_physical(self, value: f64) -> i32 {
        (value * self.scale).round() as i32
    }

    pub fn physical_to_logical(self, value: i32) -> f64 {
        value as f64 / self.scale
    }

    pub fn point_to_logical(self, x: i32, y: i32) -> Point {
        Point {
            x: self.physical_to_logical(x),
            y: self.physical_to_logical(y),
        }
    }

    pub fn rect_to_physical(self, rect: Rect) -> PhysicalRect {
        PhysicalRect {
            x: self.logical_to_physical(rect.x),
            y: self.logical_to_physical(rect.y),
            width: self.logical_to_physical(rect.width),
            height: self.logical_to_physical(rect.height),
        }
    }
}

#[cfg(all(target_os = "windows", not(test)))]
pub fn ensure_process_dpi_awareness() {
    use windows::Win32::UI::HiDpi::{
        SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
    };
    unsafe {
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
}

#[cfg(any(not(target_os = "windows"), test))]
pub fn ensure_process_dpi_awareness() {}

#[cfg(all(target_os = "windows", not(test)))]
pub fn system_dpi_scale() -> DpiScale {
    use windows::Win32::UI::HiDpi::GetDpiForSystem;
    DpiScale::from_dpi(unsafe { GetDpiForSystem() })
}

#[cfg(any(not(target_os = "windows"), test))]
pub fn system_dpi_scale() -> DpiScale {
    DpiScale::default()
}

#[cfg(all(target_os = "windows", not(test)))]
pub fn window_dpi_scale(hwnd: Option<isize>) -> DpiScale {
    use windows::Win32::{Foundation::HWND, UI::HiDpi::GetDpiForWindow};
    let Some(handle) = hwnd else {
        return DpiScale::default();
    };
    let dpi = unsafe { GetDpiForWindow(HWND(handle as _)) };
    DpiScale::from_dpi(dpi)
}

#[cfg(any(not(target_os = "windows"), test))]
pub fn window_dpi_scale(_hwnd: Option<isize>) -> DpiScale {
    DpiScale::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dpi_scale_maps_logical_rect_at_100_percent() {
        let scale = DpiScale::from_scale(1.0);
        assert_eq!(
            scale.rect_to_physical(Rect {
                x: 0.0,
                y: 0.0,
                width: 253.0,
                height: 80.0,
            }),
            PhysicalRect {
                x: 0,
                y: 0,
                width: 253,
                height: 80,
            }
        );
    }

    #[test]
    fn dpi_scale_rounds_logical_rect_at_125_percent() {
        let scale = DpiScale::from_scale(1.25);
        assert_eq!(
            scale.rect_to_physical(Rect {
                x: 10.0,
                y: 20.0,
                width: 253.0,
                height: 80.0,
            }),
            PhysicalRect {
                x: 13,
                y: 25,
                width: 316,
                height: 100,
            }
        );
    }

    #[test]
    fn dpi_scale_maps_physical_point_back_to_logical() {
        let scale = DpiScale::from_scale(1.25);
        assert_eq!(scale.point_to_logical(150, 75), Point { x: 120.0, y: 60.0 });
    }
}
