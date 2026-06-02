use crate::error::CapsuleError;
use tauri::utils::config::Color;
use tauri::{Position, Size, WebviewWindow};

/// 窗口操作批量结构
///
/// 用于批量执行窗口操作以减少 IPC 调用和窗口重绘。
/// 所有字段都是 Option，只有设置的字段才会被执行。
pub struct WindowOperationBatch {
    pub min_size: Option<Size>,
    pub resizable: Option<bool>,
    pub always_on_top: Option<bool>,
    pub decorations: Option<bool>,
    pub background_color: Option<Color>,
    pub shadow: Option<bool>,
    pub skip_taskbar: Option<bool>,
    pub size: Option<Size>,
    pub position: Option<Position>,
}

impl WindowOperationBatch {
    /// 创建一个空的批量操作
    pub fn new() -> Self {
        Self {
            min_size: None,
            resizable: None,
            always_on_top: None,
            decorations: None,
            background_color: None,
            shadow: None,
            skip_taskbar: None,
            size: None,
            position: None,
        }
    }

    /// 创建一个 Builder
    pub fn builder() -> WindowOperationBatchBuilder {
        WindowOperationBatchBuilder::new()
    }

    /// 批量执行所有设置的窗口操作
    ///
    /// # 参数
    /// - `window`: 要操作的窗口
    ///
    /// # 返回
    /// - `Ok(())`: 所有操作成功
    /// - `Err(CapsuleError)`: 某个操作失败
    pub fn execute(self, window: &WebviewWindow) -> Result<(), CapsuleError> {
        // 设置最小尺寸
        if let Some(min_size) = self.min_size {
            window.set_min_size(Some(min_size)).map_err(|e| {
                CapsuleError::WindowOperationFailed(format!("设置最小尺寸失败: {}", e))
            })?;
        }

        // 设置可缩放
        if let Some(resizable) = self.resizable {
            window.set_resizable(resizable).map_err(|e| {
                CapsuleError::WindowOperationFailed(format!("设置可缩放失败: {}", e))
            })?;
        }

        // 设置置顶
        if let Some(always_on_top) = self.always_on_top {
            window
                .set_always_on_top(always_on_top)
                .map_err(|e| CapsuleError::WindowOperationFailed(format!("设置置顶失败: {}", e)))?;
        }

        // 设置装饰
        if let Some(decorations) = self.decorations {
            window
                .set_decorations(decorations)
                .map_err(|e| CapsuleError::WindowOperationFailed(format!("设置装饰失败: {}", e)))?;
        }

        // 设置背景色（忽略错误，因为某些平台可能不支持）
        if let Some(color) = self.background_color {
            let _ = window.set_background_color(Some(color));
        }

        // 设置阴影（忽略错误，因为某些平台可能不支持）
        if let Some(shadow) = self.shadow {
            let _ = window.set_shadow(shadow);
        }

        // 设置任务栏显示（仅 Windows）
        #[cfg(target_os = "windows")]
        if let Some(skip) = self.skip_taskbar {
            let _ = window.set_skip_taskbar(skip);
        }

        // 设置尺寸
        if let Some(size) = self.size {
            window
                .set_size(size)
                .map_err(|e| CapsuleError::WindowOperationFailed(format!("设置尺寸失败: {}", e)))?;
        }

        // 设置位置
        if let Some(position) = self.position {
            window
                .set_position(position)
                .map_err(|e| CapsuleError::WindowOperationFailed(format!("设置位置失败: {}", e)))?;
        }

        Ok(())
    }
}

impl Default for WindowOperationBatch {
    fn default() -> Self {
        Self::new()
    }
}

/// 窗口操作批量构建器
///
/// 使用 Builder 模式构建窗口操作批量。
pub struct WindowOperationBatchBuilder {
    batch: WindowOperationBatch,
}

impl WindowOperationBatchBuilder {
    /// 创建一个新的 Builder
    pub fn new() -> Self {
        Self {
            batch: WindowOperationBatch::new(),
        }
    }

    /// 设置最小尺寸
    pub fn min_size(mut self, size: Option<Size>) -> Self {
        self.batch.min_size = size;
        self
    }

    /// 设置可缩放
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.batch.resizable = Some(resizable);
        self
    }

    /// 设置置顶
    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.batch.always_on_top = Some(always_on_top);
        self
    }

    /// 设置装饰
    pub fn decorations(mut self, decorations: bool) -> Self {
        self.batch.decorations = Some(decorations);
        self
    }

    /// 设置背景色
    pub fn background_color(mut self, color: Color) -> Self {
        self.batch.background_color = Some(color);
        self
    }

    /// 设置阴影
    #[allow(dead_code)]
    pub fn shadow(mut self, shadow: bool) -> Self {
        self.batch.shadow = Some(shadow);
        self
    }

    /// 设置任务栏显示
    pub fn skip_taskbar(mut self, skip: bool) -> Self {
        self.batch.skip_taskbar = Some(skip);
        self
    }

    /// 设置尺寸
    #[allow(dead_code)]
    pub fn size(mut self, size: Size) -> Self {
        self.batch.size = Some(size);
        self
    }

    /// 设置位置
    #[allow(dead_code)]
    pub fn position(mut self, position: Position) -> Self {
        self.batch.position = Some(position);
        self
    }

    /// 构建批量操作
    pub fn build(self) -> WindowOperationBatch {
        self.batch
    }
}

impl Default for WindowOperationBatchBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::{LogicalSize, PhysicalPosition};

    #[test]
    fn test_builder_pattern() {
        let batch = WindowOperationBatch::builder()
            .resizable(false)
            .always_on_top(true)
            .decorations(false)
            .shadow(true)
            .size(Size::Logical(LogicalSize::new(800.0, 600.0)))
            .position(Position::Physical(PhysicalPosition::new(100, 100)))
            .build();

        assert_eq!(batch.resizable, Some(false));
        assert_eq!(batch.always_on_top, Some(true));
        assert_eq!(batch.decorations, Some(false));
        assert_eq!(batch.shadow, Some(true));
        assert!(batch.size.is_some());
        assert!(batch.position.is_some());
    }

    #[test]
    fn test_empty_batch() {
        let batch = WindowOperationBatch::new();

        assert!(batch.min_size.is_none());
        assert!(batch.resizable.is_none());
        assert!(batch.always_on_top.is_none());
        assert!(batch.decorations.is_none());
        assert!(batch.background_color.is_none());
        assert!(batch.shadow.is_none());
        assert!(batch.skip_taskbar.is_none());
        assert!(batch.size.is_none());
        assert!(batch.position.is_none());
    }

    #[test]
    fn test_partial_batch() {
        let batch = WindowOperationBatch::builder()
            .resizable(false)
            .always_on_top(true)
            .build();

        assert_eq!(batch.resizable, Some(false));
        assert_eq!(batch.always_on_top, Some(true));
        assert!(batch.decorations.is_none());
        assert!(batch.size.is_none());
    }
}
