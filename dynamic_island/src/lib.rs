//! 灵动岛 Tauri 扩展入口。
//!
//! 这个 crate 串起应用设置、显示器选择、运行时快照、平台无关 UI 场景和 Windows
//! 原生窗口实现。`reef-ui` 负责纯 UI 模型，当前 crate 负责生命周期和平台桥接。

pub use echoisland_runtime;

// 应用级配置、显示器枚举和灵动岛模式生命周期。
mod app_settings;
mod config;
mod display_settings;
mod host_platform;
#[cfg(feature = "tauri-host")]
mod error;
#[cfg(feature = "tauri-host")]
mod mode_lifecycle;
#[cfg(feature = "tauri-host")]
mod monitor_manager;
mod native_window;
mod notification_sound;
#[cfg(feature = "tauri-host")]
mod state_machine;
mod updater_service;
#[cfg(feature = "tauri-host")]
mod window_operations;

// 对外暴露平台无关的面板核心类型，避免调用方直接依赖 reef-ui crate 路径。
pub mod native_panel_core {
    pub use reef_ui::native_panel_core::*;
}

// 原生渲染协调层：把场景模型转换为渲染命令，并处理运行时交互。
mod native_panel_renderer;

// 对外暴露平台无关的场景类型，供扩展内外共享 UI 结构。
pub mod native_panel_scene {
    pub use reef_ui::native_panel_scene::*;
}

// 应用设置/显示器信息到场景构建输入的适配层。
mod native_panel_scene_input;

// 保存面板核心状态，并按最新运行时快照构建各 Surface 场景。
mod panel_scene_service;
mod preview_host;

// Windows 平台的原生窗口、消息循环和 Direct2D 绘制实现。
#[cfg(target_os = "windows")]
mod windows_native_panel;

#[cfg(feature = "tauri-host")]
pub use error::*;
#[cfg(feature = "tauri-host")]
pub use mode_lifecycle::{
    emergency_reset_dynamic_island, enter_dynamic_island_mode, exit_dynamic_island_mode,
    is_dynamic_island_mode, snap_dynamic_island_mode,
};
#[cfg(feature = "tauri-host")]
pub use monitor_manager::{MonitorInfo, MonitorManager};
pub use native_panel_core::{
    panel_display_key, PanelDisplayGeometry, PanelIslandWidthPreset, PanelLanguage, PanelRect,
};
#[cfg(feature = "tauri-host")]
pub use preview_host::TauriDynamicIslandUiPreviewHost;
pub use preview_host::{
    run_dynamic_island_ui_preview_standalone, DynamicIslandUiPreviewHost,
    StandaloneDynamicIslandUiPreviewHost,
};
#[cfg(feature = "tauri-host")]
pub use state_machine::{DynamicIslandState, DynamicIslandStateMachine, WindowSnapshot};
#[cfg(feature = "tauri-host")]
pub use window_operations::WindowOperationBatch;

/// 显示灵动岛 UI 预览，用于不依赖主窗口状态的人工检查。
#[cfg(feature = "tauri-host")]
pub fn show_dynamic_island_ui_preview(
    app: &tauri::AppHandle,
    snapshot: &echoisland_runtime::RuntimeSnapshot,
) -> Result<(), String> {
    TauriDynamicIslandUiPreviewHost::new(app).show(snapshot)
}

/// 构造一份稳定的灵动岛 UI 预览数据。
pub fn dynamic_island_ui_preview_snapshot() -> echoisland_runtime::RuntimeSnapshot {
    use chrono::Utc;
    use echoisland_runtime::RuntimeSnapshot;

    let now = Utc::now();
    let sessions = vec![
        preview_session(
            "preview-codex",
            "codex",
            "ai-gateway",
            "gpt-5-codex",
            "Running",
            Some("cargo test -p dynamic_island"),
            Some("检查灵动岛原生面板布局与交互状态。"),
            now,
        ),
        preview_session(
            "preview-claude",
            "claude",
            "desktop-shell",
            "claude-sonnet",
            "Idle",
            None,
            Some("等待下一次任务"),
            now,
        ),
    ];

    RuntimeSnapshot {
        status: "running".to_string(),
        primary_source: "ui-preview".to_string(),
        active_session_count: 1,
        total_session_count: sessions.len(),
        pending_permission_count: 0,
        pending_question_count: 0,
        pending_permission: None,
        pending_question: None,
        pending_permissions: Vec::new(),
        pending_questions: Vec::new(),
        sessions,
    }
}

fn preview_session(
    session_id: &str,
    source: &str,
    project_name: &str,
    model: &str,
    status: &str,
    current_tool: Option<&str>,
    last_assistant_message: Option<&str>,
    now: chrono::DateTime<chrono::Utc>,
) -> echoisland_runtime::SessionSnapshotView {
    use echoisland_runtime::{SessionSnapshotView, ToolHistoryEntryView};

    SessionSnapshotView {
        session_id: session_id.to_string(),
        source: source.to_string(),
        project_name: Some(project_name.to_string()),
        cwd: Some("D:\\github\\ai-gateway".to_string()),
        model: Some(model.to_string()),
        terminal_app: Some("Windows Terminal".to_string()),
        terminal_bundle: None,
        host_app: Some("AI Gateway".to_string()),
        window_title: Some(format!("{source} preview")),
        tty: None,
        terminal_pid: None,
        cli_pid: None,
        iterm_session_id: None,
        kitty_window_id: None,
        tmux_env: None,
        tmux_pane: None,
        tmux_client_tty: None,
        status: status.to_string(),
        current_tool: current_tool.map(str::to_string),
        tool_description: current_tool.map(|tool| format!("正在执行 {tool}")),
        last_user_prompt: Some("只测试灵动岛 UI".to_string()),
        last_assistant_message: last_assistant_message.map(str::to_string),
        tool_history_count: 1,
        tool_history: vec![ToolHistoryEntryView {
            tool: "preview".to_string(),
            description: Some("构造预览数据".to_string()),
            success: true,
            timestamp: now,
        }],
        last_activity: now,
    }
}

/// 初始化灵动岛扩展
#[cfg(feature = "tauri-host")]
pub fn init<R: tauri::Runtime>(
    _app: &tauri::AppHandle<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("初始化灵动岛扩展");
    Ok(())
}
