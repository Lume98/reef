use chrono::Utc;
use echoisland_runtime::{RuntimeSnapshot, SessionSnapshotView, ToolHistoryEntryView};

pub trait DynamicIslandUiPreviewHost {
    fn show(&self, snapshot: &RuntimeSnapshot) -> Result<(), String>;
    fn run(&self) -> Result<(), String>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StandaloneDynamicIslandUiPreviewHost;

impl DynamicIslandUiPreviewHost for StandaloneDynamicIslandUiPreviewHost {
    fn show(&self, snapshot: &RuntimeSnapshot) -> Result<(), String> {
        crate::native_window::show_without_app(snapshot)
    }

    fn run(&self) -> Result<(), String> {
        loop {
            std::thread::park();
        }
    }
}

#[cfg(feature = "tauri-host")]
pub struct TauriDynamicIslandUiPreviewHost<'a, R: tauri::Runtime> {
    app: &'a tauri::AppHandle<R>,
}

#[cfg(feature = "tauri-host")]
impl<'a, R: tauri::Runtime> TauriDynamicIslandUiPreviewHost<'a, R> {
    pub fn new(app: &'a tauri::AppHandle<R>) -> Self {
        Self { app }
    }
}

#[cfg(feature = "tauri-host")]
impl<R: tauri::Runtime + 'static> DynamicIslandUiPreviewHost
    for TauriDynamicIslandUiPreviewHost<'_, R>
{
    fn show(&self, snapshot: &RuntimeSnapshot) -> Result<(), String> {
        crate::native_window::show(self.app, 0, 0, 0, 0)?;
        let _ = snapshot;
        Ok(())
    }

    fn run(&self) -> Result<(), String> {
        Ok(())
    }
}

pub fn run_dynamic_island_ui_preview_standalone() -> Result<(), String> {
    let host = StandaloneDynamicIslandUiPreviewHost;
    let snapshot = dynamic_island_ui_preview_snapshot();
    host.show(&snapshot)?;
    host.run()
}

#[cfg(feature = "tauri-host")]
pub fn show_dynamic_island_ui_preview(
    app: &tauri::AppHandle,
    snapshot: &RuntimeSnapshot,
) -> Result<(), String> {
    TauriDynamicIslandUiPreviewHost::new(app).show(snapshot)
}

pub fn dynamic_island_ui_preview_snapshot() -> RuntimeSnapshot {
    let now = Utc::now();
    let sessions = vec![
        preview_session(PreviewSessionInput {
            session_id: "preview-codex",
            source: "codex",
            project_name: "ai-gateway",
            model: "gpt-5-codex",
            status: "Running",
            current_tool: Some("cargo test -p dynamic_island"),
            last_assistant_message: Some("检查灵动岛原生面板布局与交互状态。"),
            now,
        }),
        preview_session(PreviewSessionInput {
            session_id: "preview-claude",
            source: "claude",
            project_name: "desktop-shell",
            model: "claude-sonnet",
            status: "Idle",
            current_tool: None,
            last_assistant_message: Some("等待下一次任务"),
            now,
        }),
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

struct PreviewSessionInput<'a> {
    session_id: &'a str,
    source: &'a str,
    project_name: &'a str,
    model: &'a str,
    status: &'a str,
    current_tool: Option<&'a str>,
    last_assistant_message: Option<&'a str>,
    now: chrono::DateTime<chrono::Utc>,
}

fn preview_session(input: PreviewSessionInput<'_>) -> SessionSnapshotView {
    SessionSnapshotView {
        session_id: input.session_id.to_string(),
        source: input.source.to_string(),
        project_name: Some(input.project_name.to_string()),
        cwd: Some("D:\\github\\ai-gateway".to_string()),
        model: Some(input.model.to_string()),
        terminal_app: Some("Windows Terminal".to_string()),
        terminal_bundle: None,
        host_app: Some("AI Gateway".to_string()),
        window_title: Some(format!("{} preview", input.source)),
        tty: None,
        terminal_pid: None,
        cli_pid: None,
        iterm_session_id: None,
        kitty_window_id: None,
        tmux_env: None,
        tmux_pane: None,
        tmux_client_tty: None,
        status: input.status.to_string(),
        current_tool: input.current_tool.map(str::to_string),
        tool_description: input.current_tool.map(|tool| format!("正在执行 {tool}")),
        last_user_prompt: Some("只测试灵动岛 UI".to_string()),
        last_assistant_message: input.last_assistant_message.map(str::to_string),
        tool_history_count: 1,
        tool_history: vec![ToolHistoryEntryView {
            tool: "preview".to_string(),
            description: Some("构造预览数据".to_string()),
            success: true,
            timestamp: input.now,
        }],
        last_activity: input.now,
    }
}
