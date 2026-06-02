//! 原生面板场景模型。
//!
//! 场景层把核心状态转换为 Surface、紧凑栏、卡片、设置页和命中目标等结构。
//! 渲染层只需要消费这些结构，不需要重新理解会话、权限、问题等运行时语义。

#![allow(dead_code)]

// 场景装配入口：从 PanelState、RuntimeSnapshot 和构建输入生成完整 PanelScene。
mod build;

// 通用场景节点与面板场景容器定义。
mod scene;

// 具体 Surface/Card 的结构和构建辅助。
mod projection;
mod session_card_scene;
mod settings_projection;
mod settings_scene;
mod status_card_scene;
mod surface_scene;

pub use build::*;
pub(crate) use projection::*;
pub use scene::*;
pub use session_card_scene::*;
pub use settings_projection::*;
pub use settings_scene::*;
pub use status_card_scene::*;
pub use surface_scene::*;

#[cfg(test)]
mod tests;
