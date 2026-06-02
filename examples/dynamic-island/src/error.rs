use std::fmt;

#[derive(Debug)]
pub enum CapsuleError {
    StateLockFailed,
    InvalidTransition {
        from: DynamicIslandState,
        to: DynamicIslandState,
    },
    WindowOperationFailed(String),
    #[allow(dead_code)]
    SnapshotFailed(String),
}

impl fmt::Display for CapsuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StateLockFailed => write!(f, "状态锁定失败"),
            Self::InvalidTransition { from, to } => {
                write!(f, "非法状态转换: {:?} -> {:?}", from, to)
            }
            Self::WindowOperationFailed(msg) => write!(f, "窗口操作失败: {}", msg),
            Self::SnapshotFailed(msg) => write!(f, "快照操作失败: {}", msg),
        }
    }
}

impl std::error::Error for CapsuleError {}

use crate::state_machine::DynamicIslandState;
