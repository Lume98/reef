//! 桥接模块：将 `RuntimeSnapshot` 转换为 `reef-widgets` 的 `IslandWidget`。
//!
//! 这里只做运行时数据到组件树的映射，以及渲染期 override 的集中封装。

use echoisland_runtime::RuntimeSnapshot;
use reef_widgets::{
    build_island_widget as build_framework_island_widget, island_widget::DisplayMode,
    island_widget::IslandRenderOverrides,
    island_widget::IslandPendingApprovalInput, island_widget::IslandPendingQuestionInput,
    island_widget::IslandSessionInput, island_widget::IslandWidgetContentInput, IslandWidget,
    ChromeVisibility,
};

/// 将运行时快照转换为可复用的岛屿输入模型。
pub fn build_island_widget_input(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> IslandWidgetContentInput {
    IslandWidgetContentInput {
        mode: if panel_expanded {
            DisplayMode::Expanded
        } else {
            DisplayMode::Compact
        },
        layout: Default::default(),
        settings_active,
        active_session_count: snapshot.active_session_count,
        total_session_count: snapshot.total_session_count,
        pending_permissions: snapshot
            .pending_permissions
            .iter()
            .map(|pending| IslandPendingApprovalInput {
                session_id: pending.session_id.clone(),
                source: pending.source.clone(),
                tool_description: pending.tool_description.clone(),
            })
            .collect(),
        pending_questions: snapshot
            .pending_questions
            .iter()
            .map(|pending| IslandPendingQuestionInput {
                session_id: pending.session_id.clone(),
                source: pending.source.clone(),
                header: pending.header.clone(),
                text: pending.text.clone(),
            })
            .collect(),
        sessions: snapshot
            .sessions
            .iter()
            .map(|session| IslandSessionInput {
                status: session.status.clone(),
                source: session.source.clone(),
                model: session.model.clone(),
                last_user_prompt: session.last_user_prompt.clone(),
                last_assistant_message: session.last_assistant_message.clone(),
                current_tool: session.current_tool.clone(),
                tool_description: session.tool_description.clone(),
            })
            .collect(),
    }
}

/// 将运行时快照转换为顶层 `IslandWidget`。
///
/// `panel_expanded` 控制展示模式（Compact / Expanded）。
/// `settings_active` 控制是否展示设置卡片。
pub fn build_island_widget(
    snapshot: &RuntimeSnapshot,
    panel_expanded: bool,
    settings_active: bool,
) -> IslandWidget {
    build_framework_island_widget(&build_island_widget_input(
        snapshot,
        panel_expanded,
        settings_active,
    ))
}

pub fn island_render_overrides(
    width: f64,
    compact_height: f64,
    expanded_height: f64,
    chrome: ChromeVisibility,
    reveal_progress: f64,
    entering: bool,
) -> IslandRenderOverrides {
    IslandRenderOverrides::new(
        width,
        compact_height,
        expanded_height,
        chrome,
        reveal_progress,
        entering,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_snapshot() -> RuntimeSnapshot {
        RuntimeSnapshot {
            status: "idle".to_string(),
            primary_source: "reef".to_string(),
            active_session_count: 0,
            total_session_count: 0,
            pending_permission_count: 0,
            pending_question_count: 0,
            pending_permission: None,
            pending_question: None,
            pending_permissions: vec![],
            pending_questions: vec![],
            sessions: vec![],
        }
    }

    #[test]
    fn bridge_maps_runtime_snapshot_to_input_model() {
        let snapshot = empty_snapshot();
        let input = build_island_widget_input(&snapshot, true, true);

        assert_eq!(input.mode, DisplayMode::Expanded);
        assert!(input.settings_active);
        assert_eq!(input.layout, Default::default());
        assert_eq!(input.active_session_count, 0);
        assert_eq!(input.total_session_count, 0);
    }

    #[test]
    fn bridge_builds_widget_from_snapshot() {
        let snapshot = empty_snapshot();
        let widget = build_island_widget(&snapshot, false, false);
        let default_layout: reef_widgets::island_widget::IslandWidgetLayout = Default::default();

        assert_eq!(widget.mode, DisplayMode::Compact);
        assert_eq!(widget.width, default_layout.width);
    }

    #[test]
    fn bridge_builds_render_overrides() {
        let overrides = island_render_overrides(
            320.0,
            48.0,
            180.0,
            ChromeVisibility::expanded(),
            0.5,
            true,
        );

        assert_eq!(overrides.width, 320.0);
        assert_eq!(overrides.compact_height, 48.0);
        assert_eq!(overrides.expanded_height, 180.0);
        assert_eq!(overrides.reveal_progress, 0.5);
        assert!(overrides.entering);
    }
}
