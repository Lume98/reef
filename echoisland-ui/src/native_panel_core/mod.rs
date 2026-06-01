//! 原生面板核心状态与纯逻辑。
//!
//! 该模块负责把 `RuntimeSnapshot` 同步为稳定的 `PanelState`，并维护交互、动画、
//! 布局尺寸和样式常量。它不构建具体卡片场景，也不触碰平台窗口 API。

#![allow(dead_code, unused_imports)]

// 状态推进与动画时序。
mod animation;
mod card_metrics;
mod constants;
mod interaction;
mod lightweight_refresh;
mod mascot_runtime;
mod queue;
mod reminder;
mod render;
mod settings;
mod style;
mod transitions;
mod types;

// 类型状态模式：把输入、计算和渲染阶段拆开，减少阶段字段误用。
mod typed_state;

pub use animation::*;
pub use card_metrics::*;
pub use constants::*;
pub use interaction::*;
pub use lightweight_refresh::*;
pub use mascot_runtime::*;
pub use queue::*;
pub use reminder::*;
pub use render::*;
pub use settings::*;
pub use style::*;
pub use transitions::*;
pub use types::*;

// 导出新的类型状态 API（与旧类型并存）
pub use typed_state::{
    ComputedStage, InputStage, PanelRenderLayerStyle, RenderStage,
    RenderState as SharedExpandedRenderStateV2, SharedExpandedContent,
};

#[cfg(test)]
mod tests;
