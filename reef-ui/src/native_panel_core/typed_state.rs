// Phase 1 实现：类型状态模式统一 Input/State 类型
//
// 目标：消除 SharedExpandedContentInput/State/RenderInput/RenderState 的重复
// 策略：使用泛型 + PhantomData 标记阶段，共享字段定义

use std::marker::PhantomData;

// ============ 阶段标记（零大小类型）============

/// 输入阶段：从外部接收的原始数据
pub struct InputStage;

/// 计算阶段：经过业务逻辑计算的中间状态
pub struct ComputedStage;

/// 渲染阶段：准备好传递给渲染器的最终状态
pub struct RenderStage;

// ============ 统一的共享内容状态 ============

/// 统一的共享展开内容状态（替代 4 个独立类型）
///
/// 旧类型映射：
/// - `SharedExpandedContentInput` → `SharedExpandedContent<InputStage>`
/// - `SharedExpandedContentState` → `SharedExpandedContent<ComputedStage>`
/// - `SharedExpandedRenderInput` → `SharedExpandedContent<RenderStage>`
/// - `SharedExpandedRenderState` → `SharedExpandedContent<RenderStage>` (简化版)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharedExpandedContent<Stage> {
    // ===== 所有阶段共享的字段 =====
    pub enabled: bool,
    pub shell_visible: bool,
    pub height_progress: f64,
    pub bar_progress: f64,
    pub cards_height: f64,
    pub status_surface_active: bool,
    pub content_visibility: f64,

    // ===== 阶段特定字段（通过 Option 实现）=====
    /// 仅在 RenderStage 有值
    pub transitioning: Option<bool>,

    /// 仅在 ComputedStage/RenderStage 有值
    pub visible: Option<bool>,
    pub interactive: Option<bool>,

    _stage: PhantomData<Stage>,
}

// ============ 统一的渲染层样式 ============

/// 统一的渲染层样式（替代 PanelRenderLayerStyleInput/State）
///
/// 旧类型映射：
/// - `PanelRenderLayerStyleInput` → `PanelRenderLayerStyle<InputStage>`
/// - `PanelRenderLayerStyleState` → `PanelRenderLayerStyle<ComputedStage>`
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelRenderLayerStyle<Stage> {
    pub shell_visible: bool,
    pub separator_visibility: f64,
    pub shared_visible: bool,
    pub bar_progress: f64,
    pub height_progress: f64,
    pub chrome_transition_progress: f64,
    pub shoulder_progress: f64,
    pub headline_emphasized: bool,
    pub edge_actions_visible: bool,

    _stage: PhantomData<Stage>,
}

impl PanelRenderLayerStyle<InputStage> {
    /// 创建输入阶段实例
    pub fn new_input(
        shell_visible: bool,
        separator_visibility: f64,
        shared_visible: bool,
        bar_progress: f64,
        height_progress: f64,
        chrome_transition_progress: f64,
        shoulder_progress: f64,
        headline_emphasized: bool,
        edge_actions_visible: bool,
    ) -> Self {
        Self {
            shell_visible,
            separator_visibility,
            shared_visible,
            bar_progress,
            height_progress,
            chrome_transition_progress,
            shoulder_progress,
            headline_emphasized,
            edge_actions_visible,
            _stage: PhantomData,
        }
    }

    /// 转换到计算阶段（当前无额外计算，直接转换）
    pub fn compute(self) -> PanelRenderLayerStyle<ComputedStage> {
        PanelRenderLayerStyle {
            shell_visible: self.shell_visible,
            separator_visibility: self.separator_visibility,
            shared_visible: self.shared_visible,
            bar_progress: self.bar_progress,
            height_progress: self.height_progress,
            chrome_transition_progress: self.chrome_transition_progress,
            shoulder_progress: self.shoulder_progress,
            headline_emphasized: self.headline_emphasized,
            edge_actions_visible: self.edge_actions_visible,
            _stage: PhantomData,
        }
    }
}

// 兼容层：从旧类型转换
impl From<crate::native_panel_core::PanelRenderLayerStyleInput>
    for PanelRenderLayerStyle<InputStage>
{
    fn from(old: crate::native_panel_core::PanelRenderLayerStyleInput) -> Self {
        Self::new_input(
            old.shell_visible,
            old.separator_visibility,
            old.shared_visible,
            old.bar_progress,
            old.height_progress,
            old.chrome_transition_progress,
            old.shoulder_progress,
            old.headline_emphasized,
            old.edge_actions_visible,
        )
    }
}

// 兼容层：转换到旧类型
impl From<PanelRenderLayerStyle<ComputedStage>>
    for crate::native_panel_core::PanelRenderLayerStyleState
{
    fn from(new: PanelRenderLayerStyle<ComputedStage>) -> Self {
        Self {
            shell_visible: new.shell_visible,
            separator_visibility: new.separator_visibility,
            shared_visible: new.shared_visible,
            bar_progress: new.bar_progress,
            height_progress: new.height_progress,
            chrome_transition_progress: new.chrome_transition_progress,
            shoulder_progress: new.shoulder_progress,
            headline_emphasized: new.headline_emphasized,
            edge_actions_visible: new.edge_actions_visible,
        }
    }
}

// ============ InputStage 实现 ============

impl SharedExpandedContent<InputStage> {
    /// 创建输入阶段实例（对应旧的 SharedExpandedContentInput）
    pub fn new_input(
        enabled: bool,
        shell_visible: bool,
        height_progress: f64,
        bar_progress: f64,
        cards_height: f64,
        status_surface_active: bool,
        content_visibility: f64,
    ) -> Self {
        Self {
            enabled,
            shell_visible,
            height_progress,
            bar_progress,
            cards_height,
            status_surface_active,
            content_visibility,
            transitioning: None,
            visible: None,
            interactive: None,
            _stage: PhantomData,
        }
    }

    /// 转换到计算阶段（对应旧的 SharedExpandedContentState）
    pub fn compute(self) -> SharedExpandedContent<ComputedStage> {
        const REVEAL_PROGRESS: f64 = 0.94;
        const INTERACTIVE_PROGRESS: f64 = 0.985;

        let visible = self.enabled && self.height_progress >= REVEAL_PROGRESS;
        let interactive = self.enabled && self.height_progress >= INTERACTIVE_PROGRESS;

        SharedExpandedContent {
            enabled: self.enabled,
            shell_visible: self.shell_visible,
            height_progress: self.height_progress,
            bar_progress: self.bar_progress,
            cards_height: self.cards_height,
            status_surface_active: self.status_surface_active,
            content_visibility: self.content_visibility,
            transitioning: None,
            visible: Some(visible),
            interactive: Some(interactive),
            _stage: PhantomData,
        }
    }

    /// 直接转换到渲染阶段（对应旧的 SharedExpandedRenderInput）
    pub fn to_render_input(self, transitioning: bool) -> SharedExpandedContent<RenderStage> {
        SharedExpandedContent {
            enabled: self.enabled,
            shell_visible: self.shell_visible,
            height_progress: self.height_progress,
            bar_progress: self.bar_progress,
            cards_height: self.cards_height,
            status_surface_active: self.status_surface_active,
            content_visibility: self.content_visibility,
            transitioning: Some(transitioning),
            visible: None,
            interactive: None,
            _stage: PhantomData,
        }
    }
}

// ============ ComputedStage 实现 ============

impl SharedExpandedContent<ComputedStage> {
    /// 转换到渲染阶段
    pub fn prepare_render(self, transitioning: bool) -> SharedExpandedContent<RenderStage> {
        SharedExpandedContent {
            enabled: self.enabled,
            shell_visible: self.shell_visible,
            height_progress: self.height_progress,
            bar_progress: self.bar_progress,
            cards_height: self.cards_height,
            status_surface_active: self.status_surface_active,
            content_visibility: self.content_visibility,
            transitioning: Some(transitioning),
            visible: self.visible,
            interactive: self.interactive,
            _stage: PhantomData,
        }
    }

    /// 获取可见性（类型安全访问）
    pub fn is_visible(&self) -> bool {
        self.visible.unwrap_or(false)
    }

    /// 获取交互性（类型安全访问）
    pub fn is_interactive(&self) -> bool {
        self.interactive.unwrap_or(false)
    }
}

// ============ RenderStage 实现 ============

impl SharedExpandedContent<RenderStage> {
    /// 获取是否正在过渡（类型安全访问）
    pub fn is_transitioning(&self) -> bool {
        self.transitioning.unwrap_or(false)
    }

    /// 简化的渲染状态（对应旧的 SharedExpandedRenderState）
    pub fn to_render_state(&self) -> RenderState {
        RenderState {
            enabled: self.enabled,
            visible: self.visible.unwrap_or(false),
            interactive: self.interactive.unwrap_or(false),
        }
    }
}

/// 简化的渲染状态（替代 SharedExpandedRenderState）
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderState {
    pub enabled: bool,
    pub visible: bool,
    pub interactive: bool,
}

// ============ 兼容层：与旧类型互转 ============

// 从旧的 SharedExpandedContentInput 转换
impl From<crate::native_panel_core::SharedExpandedContentInput>
    for SharedExpandedContent<InputStage>
{
    fn from(old: crate::native_panel_core::SharedExpandedContentInput) -> Self {
        Self::new_input(
            old.enabled,
            old.shell_visible,
            old.height_progress,
            old.bar_progress,
            old.cards_height,
            old.status_surface_active,
            old.content_visibility,
        )
    }
}

// 转换到旧的 SharedExpandedContentState
impl From<SharedExpandedContent<ComputedStage>>
    for crate::native_panel_core::SharedExpandedContentState
{
    fn from(new: SharedExpandedContent<ComputedStage>) -> Self {
        Self {
            visible: new.visible.unwrap_or(false),
            interactive: new.interactive.unwrap_or(false),
        }
    }
}

// ============ 测试 ============

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_state_pipeline() {
        // 输入阶段
        let input = SharedExpandedContent::new_input(
            true,  // enabled
            true,  // shell_visible
            0.99,  // height_progress (> 0.985 interactive threshold)
            0.8,   // bar_progress
            200.0, // cards_height
            true,  // status_surface_active
            1.0,   // content_visibility
        );

        // 计算阶段
        let computed = input.compute();
        assert!(computed.is_visible()); // 0.99 > 0.94
        assert!(computed.is_interactive()); // 0.99 > 0.985

        // 渲染阶段
        let render = computed.prepare_render(false);
        assert!(!render.is_transitioning());

        let state = render.to_render_state();
        assert!(state.enabled);
        assert!(state.visible);
    }

    #[test]
    fn test_visibility_thresholds() {
        // 低于可见阈值
        let input = SharedExpandedContent::new_input(true, true, 0.93, 0.8, 200.0, true, 1.0);
        let computed = input.compute();
        assert!(!computed.is_visible()); // 0.93 < 0.94

        // 达到可见但未达到交互
        let input = SharedExpandedContent::new_input(true, true, 0.95, 0.8, 200.0, true, 1.0);
        let computed = input.compute();
        assert!(computed.is_visible()); // 0.95 > 0.94
        assert!(!computed.is_interactive()); // 0.95 < 0.985

        // 达到交互阈值
        let input = SharedExpandedContent::new_input(true, true, 0.99, 0.8, 200.0, true, 1.0);
        let computed = input.compute();
        assert!(computed.is_visible());
        assert!(computed.is_interactive()); // 0.99 > 0.985
    }

    #[test]
    fn test_compile_time_safety() {
        let input = SharedExpandedContent::new_input(true, true, 0.5, 0.5, 200.0, true, 1.0);

        // ❌ 编译错误：InputStage 没有 is_visible 方法
        // input.is_visible();

        // ✅ 必须先转换到 ComputedStage
        let computed = input.compute();
        let _ = computed.is_visible();
    }

    #[test]
    fn test_backward_compatibility() {
        // 旧类型 → 新类型
        let old_input = crate::native_panel_core::SharedExpandedContentInput {
            enabled: true,
            shell_visible: true,
            height_progress: 0.99,
            bar_progress: 0.8,
            cards_height: 200.0,
            status_surface_active: true,
            content_visibility: 1.0,
        };

        let new_input: SharedExpandedContent<InputStage> = old_input.into();
        let computed = new_input.compute();

        // 新类型 → 旧类型
        let old_state: crate::native_panel_core::SharedExpandedContentState = computed.into();
        assert!(old_state.visible);
        assert!(old_state.interactive);
    }
}
