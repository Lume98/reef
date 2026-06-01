use std::sync::{Arc, Mutex, OnceLock};
use tauri::{PhysicalPosition, PhysicalSize};

use crate::error::CapsuleError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DynamicIslandState {
    Idle,
    Entering,
    Capsule,
    Exiting,
    Restoring,
}

#[derive(Clone, Copy, Debug)]
pub struct WindowSnapshot {
    pub position: PhysicalPosition<i32>,
    pub size: PhysicalSize<u32>,
    pub maximized: bool,
}

static STATE_MACHINE: OnceLock<DynamicIslandStateMachine> = OnceLock::new();

pub struct DynamicIslandStateMachine {
    state: Arc<Mutex<DynamicIslandState>>,
    snapshot: Arc<Mutex<Option<WindowSnapshot>>>,
}

impl DynamicIslandStateMachine {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(DynamicIslandState::Idle)),
            snapshot: Arc::new(Mutex::new(None)),
        }
    }

    pub fn global() -> &'static DynamicIslandStateMachine {
        STATE_MACHINE.get_or_init(DynamicIslandStateMachine::new)
    }

    pub fn get_state(&self) -> DynamicIslandState {
        *self.state.lock().unwrap()
    }

    pub fn save_snapshot(&self, snapshot: WindowSnapshot) {
        if let Ok(mut guard) = self.snapshot.lock() {
            *guard = Some(snapshot);
        }
    }

    pub fn get_snapshot(&self) -> Option<WindowSnapshot> {
        self.snapshot.lock().ok().and_then(|guard| *guard)
    }

    pub fn clear_snapshot(&self) {
        if let Ok(mut guard) = self.snapshot.lock() {
            *guard = None;
        }
    }

    pub fn can_transition(&self, from: DynamicIslandState, to: DynamicIslandState) -> bool {
        use DynamicIslandState::*;
        matches!(
            (from, to),
            (Idle, Entering)
                | (Entering, Capsule)
                | (Entering, Idle)
                | (Capsule, Exiting)
                | (Exiting, Restoring)
                | (Exiting, Capsule)
                | (Restoring, Idle)
        )
    }

    pub fn transition_to(&self, target: DynamicIslandState) -> Result<(), CapsuleError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| CapsuleError::StateLockFailed)?;

        let current = *state;

        if !self.can_transition(current, target) {
            return Err(CapsuleError::InvalidTransition {
                from: current,
                to: target,
            });
        }

        *state = target;
        log::info!("状态转换: {:?} -> {:?}", current, target);
        Ok(())
    }

    pub fn emergency_reset(&self) -> Result<(), CapsuleError> {
        log::warn!("触发紧急恢复，强制重置到 Idle 状态");

        let mut state = self
            .state
            .lock()
            .map_err(|_| CapsuleError::StateLockFailed)?;

        *state = DynamicIslandState::Idle;
        self.clear_snapshot();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        let machine = DynamicIslandStateMachine::new();

        assert!(machine.can_transition(DynamicIslandState::Idle, DynamicIslandState::Entering));
        assert!(machine.can_transition(DynamicIslandState::Entering, DynamicIslandState::Capsule));
        assert!(machine.can_transition(DynamicIslandState::Capsule, DynamicIslandState::Exiting));
        assert!(machine.can_transition(DynamicIslandState::Exiting, DynamicIslandState::Restoring));
        assert!(machine.can_transition(DynamicIslandState::Restoring, DynamicIslandState::Idle));
    }

    #[test]
    fn test_invalid_transitions() {
        let machine = DynamicIslandStateMachine::new();

        assert!(!machine.can_transition(DynamicIslandState::Idle, DynamicIslandState::Capsule));
        assert!(!machine.can_transition(DynamicIslandState::Entering, DynamicIslandState::Exiting));
        assert!(!machine.can_transition(DynamicIslandState::Capsule, DynamicIslandState::Restoring));
    }

    #[test]
    fn test_snapshot_save_restore() {
        let machine = DynamicIslandStateMachine::new();
        let snapshot = WindowSnapshot {
            position: PhysicalPosition::new(100, 100),
            size: PhysicalSize::new(800, 600),
            maximized: false,
        };

        machine.save_snapshot(snapshot);
        let restored = machine.get_snapshot();

        assert!(restored.is_some());
        let restored = restored.unwrap();
        assert_eq!(restored.position.x, 100);
        assert_eq!(restored.position.y, 100);
    }

    #[test]
    fn test_transition_to_valid() {
        let machine = DynamicIslandStateMachine::new();

        assert_eq!(machine.get_state(), DynamicIslandState::Idle);
        assert!(machine.transition_to(DynamicIslandState::Entering).is_ok());
        assert_eq!(machine.get_state(), DynamicIslandState::Entering);
    }

    #[test]
    fn test_transition_to_invalid() {
        let machine = DynamicIslandStateMachine::new();

        assert_eq!(machine.get_state(), DynamicIslandState::Idle);
        let result = machine.transition_to(DynamicIslandState::Capsule);
        assert!(result.is_err());
        assert_eq!(machine.get_state(), DynamicIslandState::Idle);
    }

    #[test]
    fn test_emergency_reset() {
        let machine = DynamicIslandStateMachine::new();

        // 转换到某个状态
        let _ = machine.transition_to(DynamicIslandState::Entering);
        let _ = machine.transition_to(DynamicIslandState::Capsule);

        // 保存快照
        let snapshot = WindowSnapshot {
            position: PhysicalPosition::new(100, 100),
            size: PhysicalSize::new(800, 600),
            maximized: false,
        };
        machine.save_snapshot(snapshot);

        // 紧急恢复
        assert!(machine.emergency_reset().is_ok());
        assert_eq!(machine.get_state(), DynamicIslandState::Idle);
        assert!(machine.get_snapshot().is_none());
    }
}
