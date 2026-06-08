use chrono::Utc;
use echoisland_runtime::RuntimeSnapshot;
use reef_native_panel::{
    scene::{build_panel_scene, PanelSceneBuildInput},
    state::PanelState,
};

fn main() -> Result<(), String> {
    let snapshot = preview_snapshot();
    let scene = build_panel_scene(
        &PanelState::default(),
        &snapshot,
        &PanelSceneBuildInput::default(),
    );

    println!(
        "initial-scene compact='{}' cards={} hit_targets={}",
        scene.compact_bar.headline.text,
        scene.cards.len(),
        scene.hit_targets.len()
    );

    reef_native_panel::run_dynamic_island_preview_standalone()
}

fn preview_snapshot() -> RuntimeSnapshot {
    RuntimeSnapshot {
        status: "running".to_string(),
        primary_source: "ui-preview".to_string(),
        active_session_count: 1,
        total_session_count: 1,
        pending_permission_count: 0,
        pending_question_count: 0,
        pending_permission: None,
        pending_question: None,
        pending_permissions: Vec::new(),
        pending_questions: Vec::new(),
        sessions: vec![echoisland_runtime::SessionSnapshotView {
            session_id: "preview-codex".to_string(),
            source: "codex".to_string(),
            project_name: Some("reef".to_string()),
            cwd: Some("D:\\github\\reef".to_string()),
            model: Some("gpt-5-codex".to_string()),
            terminal_app: Some("Windows Terminal".to_string()),
            terminal_bundle: None,
            host_app: Some("Reef".to_string()),
            window_title: Some("Reef preview".to_string()),
            tty: None,
            terminal_pid: None,
            cli_pid: None,
            iterm_session_id: None,
            kitty_window_id: None,
            tmux_env: None,
            tmux_pane: None,
            tmux_client_tty: None,
            status: "Running".to_string(),
            current_tool: Some("cargo test --workspace".to_string()),
            tool_description: Some("构建 Native Panel 预览场景".to_string()),
            last_user_prompt: Some("重构 framework UI 组件库边界".to_string()),
            last_assistant_message: Some(
                "Windows host consumes reef-native-panel facade.".to_string(),
            ),
            tool_history_count: 0,
            tool_history: Vec::new(),
            last_activity: Utc::now(),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_builds_panel_scene_from_ui_facade() {
        let scene = build_panel_scene(
            &PanelState::default(),
            &preview_snapshot(),
            &PanelSceneBuildInput::default(),
        );

        assert!(!scene.compact_bar.headline.text.is_empty());
    }
}
