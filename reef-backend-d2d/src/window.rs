use reef_core::geometry::Rect;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowStyle {
    LayeredTopmost,
    Normal,
}

pub struct WindowConfig {
    pub rect: Rect,
    pub style: WindowStyle,
    pub title: String,
}

impl WindowConfig {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            style: WindowStyle::LayeredTopmost,
            title: String::new(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn style(mut self, style: WindowStyle) -> Self {
        self.style = style;
        self
    }
}

#[cfg(target_os = "windows")]
pub struct NativeWindow {
    hwnd: isize,
}

#[cfg(target_os = "windows")]
impl NativeWindow {
    pub fn create(config: &WindowConfig) -> Result<Self, String> {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, RegisterClassW, CS_HREDRAW, CS_VREDRAW,
            WS_EX_LAYERED, WS_EX_TOPMOST, WS_EX_TOOLWINDOW, WS_OVERLAPPED, WNDCLASSW,
        };

        let class_name = encode_wide("ReefWindowClass");
        let instance = unsafe {
            let mut buf = [0u16; 1];
            windows_sys::Win32::System::LibraryLoader::GetModuleHandleW(buf.as_mut_ptr())
        };
        if instance.is_null() {
            return Err("GetModuleHandleW failed".to_string());
        }

        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(DefWindowProcW),
            hInstance: instance,
            lpszClassName: class_name.as_ptr(),
            ..unsafe { std::mem::zeroed() }
        };
        unsafe {
            RegisterClassW(&wnd_class);
        }

        let ex_style: u32 = match config.style {
            WindowStyle::LayeredTopmost => WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
            WindowStyle::Normal => 0,
        };
        let style: u32 = WS_OVERLAPPED;

        let title_wide = encode_wide(&config.title);
        let hwnd = unsafe {
            CreateWindowExW(
                ex_style,
                class_name.as_ptr(),
                title_wide.as_ptr(),
                style,
                config.rect.x.round() as i32,
                config.rect.y.round() as i32,
                config.rect.width.round() as i32,
                config.rect.height.round() as i32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                instance,
                std::ptr::null(),
            )
        };
        if hwnd.is_null() {
            return Err(format!(
                "CreateWindowExW failed: {}",
                std::io::Error::last_os_error()
            ));
        }

        Ok(Self {
            hwnd: hwnd as isize,
        })
    }

    pub fn hwnd(&self) -> isize {
        self.hwnd
    }

    pub fn show(&self) {
        use windows_sys::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_SHOW};
        unsafe {
            ShowWindow(self.hwnd as _, SW_SHOW);
        }
    }

    pub fn poll_message(&self) -> bool {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, TranslateMessage, MSG,
        };
        let mut msg: MSG = unsafe { std::mem::zeroed() };
        let result = unsafe { GetMessageW(&mut msg, self.hwnd as _, 0, 0) };
        if result == 0 {
            return false;
        }
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        true
    }
}

#[cfg(target_os = "windows")]
impl Drop for NativeWindow {
    fn drop(&mut self) {
        use windows_sys::Win32::UI::WindowsAndMessaging::DestroyWindow;
        unsafe {
            DestroyWindow(self.hwnd as _);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub struct NativeWindow {
    _dummy: (),
}

#[cfg(not(target_os = "windows"))]
impl NativeWindow {
    pub fn create(_config: &WindowConfig) -> Result<Self, String> {
        Ok(Self { _dummy: () })
    }
    pub fn hwnd(&self) -> isize {
        0
    }
    pub fn show(&self) {}
    pub fn poll_message(&self) -> bool {
        false
    }
}

fn encode_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::geometry::Rect;

    #[test]
    fn window_config_builder() {
        let config = WindowConfig::new(Rect {
            x: 100.0,
            y: 100.0,
            width: 320.0,
            height: 48.0,
        })
        .title("Test")
        .style(WindowStyle::LayeredTopmost);

        assert_eq!(config.title, "Test");
        assert_eq!(config.style, WindowStyle::LayeredTopmost);
    }
}
