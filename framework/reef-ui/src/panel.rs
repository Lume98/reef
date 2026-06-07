//! 原生面板的稳定聚合入口。
//!
//! 新代码优先通过这里访问 core / scene / ui 三层，避免直接依赖分散的中间模块名。

pub use crate::native_panel_core as core;
pub use crate::native_panel_scene as scene;
pub use crate::native_panel_ui as ui;

pub use crate::native_panel_ui::components;
pub use crate::native_panel_ui::descriptor;
pub use crate::native_panel_ui::migration;
pub use crate::native_panel_ui::presentation;
pub use crate::native_panel_ui::render;
pub use crate::native_panel_ui::rendering;
pub use crate::native_panel_ui::visual;
pub use crate::native_panel_ui::visual_plan;
pub use crate::native_panel_ui::widgets;
pub use crate::updater_service::{AppUpdatePhase, AppUpdateStatus};
