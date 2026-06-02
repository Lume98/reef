use tauri::{PhysicalPosition, PhysicalSize, WebviewWindow};

/// 显示器信息
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorInfo {
    pub id: String,
    pub position: PhysicalPosition<i32>,
    pub size: PhysicalSize<u32>,
    pub scale_factor: f64,
    pub is_primary: bool,
}

/// 显示器管理器
pub struct MonitorManager {
    monitors: Vec<MonitorInfo>,
}

/// 矩形区域（用于碰撞检测）
#[allow(dead_code)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    /// 检测两个矩形是否相交
    #[allow(dead_code)]
    fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

impl MonitorManager {
    /// 创建显示器管理器实例
    pub fn new(window: &WebviewWindow) -> Result<Self, String> {
        let monitors = window
            .available_monitors()
            .map_err(|e| format!("获取显示器列表失败: {}", e))?
            .into_iter()
            .enumerate()
            .map(|(idx, m)| MonitorInfo {
                id: format!("monitor_{}", idx),
                position: *m.position(),
                size: *m.size(),
                scale_factor: m.scale_factor(),
                is_primary: false,
            })
            .collect::<Vec<_>>();

        Ok(Self { monitors })
    }

    /// 获取窗口当前所在的显示器
    pub fn get_current_monitor(&self, window: &WebviewWindow) -> Option<MonitorInfo> {
        window
            .current_monitor()
            .ok()
            .flatten()
            .map(|m| MonitorInfo {
                id: "current".to_string(),
                position: *m.position(),
                size: *m.size(),
                scale_factor: m.scale_factor(),
                is_primary: false,
            })
    }

    /// 获取所有显示器列表
    pub fn get_all_monitors(&self) -> &[MonitorInfo] {
        &self.monitors
    }

    /// 检测窗口是否跨越多个显示器
    #[allow(dead_code)]
    pub fn is_window_spanning_monitors(
        &self,
        window_pos: PhysicalPosition<i32>,
        window_size: PhysicalSize<u32>,
    ) -> bool {
        let window_rect = Rect {
            x: window_pos.x,
            y: window_pos.y,
            width: window_size.width as i32,
            height: window_size.height as i32,
        };

        let overlapping_monitors = self
            .monitors
            .iter()
            .filter(|m| {
                let monitor_rect = Rect {
                    x: m.position.x,
                    y: m.position.y,
                    width: m.size.width as i32,
                    height: m.size.height as i32,
                };
                window_rect.intersects(&monitor_rect)
            })
            .count();

        overlapping_monitors > 1
    }

    /// 根据位置查找对应的显示器
    #[allow(dead_code)]
    pub fn get_monitor_for_position(
        &self,
        position: PhysicalPosition<i32>,
    ) -> Option<&MonitorInfo> {
        self.monitors.iter().find(|m| {
            position.x >= m.position.x
                && position.x < m.position.x + m.size.width as i32
                && position.y >= m.position.y
                && position.y < m.position.y + m.size.height as i32
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_intersects() {
        let rect1 = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };

        let rect2 = Rect {
            x: 50,
            y: 50,
            width: 100,
            height: 100,
        };

        assert!(rect1.intersects(&rect2));
        assert!(rect2.intersects(&rect1));

        let rect3 = Rect {
            x: 200,
            y: 200,
            width: 100,
            height: 100,
        };

        assert!(!rect1.intersects(&rect3));
        assert!(!rect3.intersects(&rect1));
    }

    #[test]
    fn test_monitor_info_serialization() {
        let monitor = MonitorInfo {
            id: "test".to_string(),
            position: PhysicalPosition::new(0, 0),
            size: PhysicalSize::new(1920, 1080),
            scale_factor: 1.0,
            is_primary: true,
        };

        let json = serde_json::to_string(&monitor).unwrap();
        assert!(json.contains("\"id\":\"test\""));
        assert!(json.contains("\"isPrimary\":true"));
    }
}
