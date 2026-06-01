//! EchoIsland 运行时共享数据契约。
//!
//! 这个 crate 只放可序列化的快照类型，供主应用、UI 场景模型和原生窗口扩展共同使用。
//! 它不依赖 Tauri 窗口或渲染实现，避免跨 crate 传递运行态数据时引入平台细节。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ToolHistoryEntryView {
    pub tool: String,
    pub description: Option<String>,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SessionSnapshotView {
    pub session_id: String,
    pub source: String,
    pub project_name: Option<String>,
    pub cwd: Option<String>,
    pub model: Option<String>,
    pub terminal_app: Option<String>,
    pub terminal_bundle: Option<String>,
    pub host_app: Option<String>,
    pub window_title: Option<String>,
    pub tty: Option<String>,
    pub terminal_pid: Option<u32>,
    pub cli_pid: Option<u32>,
    pub iterm_session_id: Option<String>,
    pub kitty_window_id: Option<String>,
    pub tmux_env: Option<String>,
    pub tmux_pane: Option<String>,
    pub tmux_client_tty: Option<String>,
    pub status: String,
    pub current_tool: Option<String>,
    pub tool_description: Option<String>,
    pub last_user_prompt: Option<String>,
    pub last_assistant_message: Option<String>,
    pub tool_history_count: usize,
    pub tool_history: Vec<ToolHistoryEntryView>,
    pub last_activity: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RuntimeSnapshot {
    pub status: String,
    pub primary_source: String,
    pub active_session_count: usize,
    pub total_session_count: usize,
    pub pending_permission_count: usize,
    pub pending_question_count: usize,
    pub pending_permission: Option<PendingPermissionView>,
    pub pending_question: Option<PendingQuestionView>,
    pub pending_permissions: Vec<PendingPermissionView>,
    pub pending_questions: Vec<PendingQuestionView>,
    pub sessions: Vec<SessionSnapshotView>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PendingPermissionView {
    pub request_id: String,
    pub session_id: String,
    pub source: String,
    pub tool_name: Option<String>,
    pub tool_description: Option<String>,
    pub requested_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PendingQuestionView {
    pub request_id: String,
    pub session_id: String,
    pub source: String,
    pub header: Option<String>,
    pub text: String,
    pub options: Vec<String>,
    pub requested_at: DateTime<Utc>,
}

impl RuntimeSnapshot {
    pub fn idle() -> Self {
        Self {
            status: "idle".to_string(),
            primary_source: "ai-gateway".to_string(),
            active_session_count: 0,
            total_session_count: 0,
            pending_permission_count: 0,
            pending_question_count: 0,
            pending_permission: None,
            pending_question: None,
            pending_permissions: Vec::new(),
            pending_questions: Vec::new(),
            sessions: Vec::new(),
        }
    }
}
