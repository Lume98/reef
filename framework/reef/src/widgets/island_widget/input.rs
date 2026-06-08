use super::{spec::IslandWidgetLayout, DisplayMode};

#[derive(Clone, Debug)]
pub struct IslandPendingApprovalInput {
    pub session_id: String,
    pub source: String,
    pub tool_description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct IslandPendingQuestionInput {
    pub session_id: String,
    pub source: String,
    pub header: Option<String>,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct IslandSessionInput {
    pub status: String,
    pub source: String,
    pub model: Option<String>,
    pub last_user_prompt: Option<String>,
    pub last_assistant_message: Option<String>,
    pub current_tool: Option<String>,
    pub tool_description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct IslandWidgetContentInput {
    pub mode: DisplayMode,
    pub layout: IslandWidgetLayout,
    pub settings_active: bool,
    pub active_session_count: usize,
    pub total_session_count: usize,
    pub pending_permissions: Vec<IslandPendingApprovalInput>,
    pub pending_questions: Vec<IslandPendingQuestionInput>,
    pub sessions: Vec<IslandSessionInput>,
}

impl Default for IslandWidgetContentInput {
    fn default() -> Self {
        Self {
            mode: DisplayMode::Hidden,
            layout: IslandWidgetLayout::default(),
            settings_active: false,
            active_session_count: 0,
            total_session_count: 0,
            pending_permissions: Vec::new(),
            pending_questions: Vec::new(),
            sessions: Vec::new(),
        }
    }
}
