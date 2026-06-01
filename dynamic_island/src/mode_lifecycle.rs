use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{utils::config::Color, Emitter, LogicalSize, Manager, Position, Size, WebviewWindow};

use crate::{
    native_window, DynamicIslandState, DynamicIslandStateMachine, WindowOperationBatch,
    WindowSnapshot,
};

const CAPSULE_WINDOW_WIDTH: u32 = 253;
const CAPSULE_WINDOW_HEIGHT: u32 = 37;
const MAIN_MIN_WIDTH: f64 = 900.0;
const MAIN_MIN_HEIGHT: f64 = 600.0;
const OPAQUE_WHITE: Color = Color(255, 255, 255, 255);
static CAPSULE_OPERATION_ACTIVE: AtomicBool = AtomicBool::new(false);

struct CapsuleOperationGuard;

impl CapsuleOperationGuard {
    fn try_acquire() -> Option<Self> {
        if CAPSULE_OPERATION_ACTIVE
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            Some(Self)
        } else {
            None
        }
    }

    fn force_release() {
        CAPSULE_OPERATION_ACTIVE.store(false, Ordering::SeqCst);
    }
}

impl Drop for CapsuleOperationGuard {
    fn drop(&mut self) {
        CAPSULE_OPERATION_ACTIVE.store(false, Ordering::SeqCst);
    }
}

fn begin_capsule_operation() -> Result<CapsuleOperationGuard, String> {
    CapsuleOperationGuard::try_acquire().ok_or_else(|| {
        log::warn!("灵动岛模式操作正在进行，忽略本次重复请求");
        "灵动岛模式操作正在进行，请稍后再试".to_string()
    })
}

fn main_window(app: &tauri::AppHandle) -> Result<WebviewWindow, String> {
    app.get_webview_window("main")
        .ok_or("主窗口未找到".to_string())
}

fn emit_capsule_state(app: &tauri::AppHandle, state: DynamicIslandState) {
    let _ = app.emit("dynamic-island-state-changed", state);
    let _ = app.emit("capsule-state-changed", state);
}

fn emit_capsule_mode(app: &tauri::AppHandle, enabled: bool) {
    let _ = app.emit("dynamic-island-mode-changed", enabled);
    let _ = app.emit("capsule-mode-changed", enabled);
}

fn reset_after_failure(app: &tauri::AppHandle, context: &str, error: &str) {
    log::error!("{}失败: {}", context, error);
    let _ = emergency_reset_dynamic_island_inner(app);
}

fn is_transient_state(state: DynamicIslandState) -> bool {
    matches!(
        state,
        DynamicIslandState::Entering | DynamicIslandState::Exiting | DynamicIslandState::Restoring
    )
}

fn calculate_native_capsule_bounds(window: &WebviewWindow) -> Result<(i32, i32, u32, u32), String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| format!("获取当前显示器失败: {e}"))?
        .or_else(|| window.primary_monitor().ok().flatten())
        .ok_or("未找到可用显示器".to_string())?;

    let monitor_pos = monitor.position();
    let monitor_size = monitor.size();
    let x = monitor_pos.x + (monitor_size.width as i32 - CAPSULE_WINDOW_WIDTH as i32) / 2;
    let y = monitor_pos.y + 8;

    Ok((x, y, CAPSULE_WINDOW_WIDTH, CAPSULE_WINDOW_HEIGHT))
}

/// 进入灵动岛模式
pub fn enter_dynamic_island_mode(app: &tauri::AppHandle) -> Result<(), String> {
    let _guard = begin_capsule_operation()?;
    let mut initial_state = DynamicIslandStateMachine::global().get_state();
    if is_transient_state(initial_state) {
        log::warn!(
            "灵动岛模式状态卡在 {:?}，进入前先执行紧急恢复",
            initial_state
        );
        emergency_reset_dynamic_island_inner(app)?;
        initial_state = DynamicIslandState::Idle;
    }
    if initial_state == DynamicIslandState::Capsule {
        return Ok(());
    }

    let result = enter_dynamic_island_mode_inner(app);
    if let Err(error) = &result {
        reset_after_failure(app, "进入灵动岛模式", error);
    }
    result
}

fn enter_dynamic_island_mode_inner(app: &tauri::AppHandle) -> Result<(), String> {
    let state_machine = DynamicIslandStateMachine::global();

    let current_state = state_machine.get_state();
    if current_state != DynamicIslandState::Idle {
        return Err(format!("当前状态 {:?} 不允许进入灵动岛模式", current_state));
    }

    state_machine
        .transition_to(DynamicIslandState::Entering)
        .map_err(|e| e.to_string())?;
    emit_capsule_state(app, DynamicIslandState::Entering);

    let main = main_window(app).map_err(|e| format!("{e}，无法进入灵动岛模式"))?;

    let snapshot = WindowSnapshot {
        position: main
            .outer_position()
            .map_err(|e| format!("读取窗口位置失败: {}", e))?,
        size: main
            .outer_size()
            .map_err(|e| format!("读取窗口尺寸失败: {}", e))?,
        maximized: main.is_maximized().unwrap_or(false),
    };
    state_machine.save_snapshot(snapshot);
    log::info!("保存窗口快照: {:?}", snapshot);

    if main.is_maximized().unwrap_or(false) {
        let _ = main.unmaximize();
    }

    let (capsule_x, capsule_y, capsule_width, capsule_height) =
        calculate_native_capsule_bounds(&main)?;
    native_window::show(
        app,
        capsule_x,
        capsule_y,
        capsule_width as i32,
        capsule_height as i32,
    )?;
    let _ = main.hide();
    log::info!("原生灵动岛吸附到顶部居中");

    state_machine
        .transition_to(DynamicIslandState::Capsule)
        .map_err(|e| e.to_string())?;

    emit_capsule_state(app, DynamicIslandState::Capsule);
    emit_capsule_mode(app, true);
    log::info!("成功进入灵动岛模式");

    Ok(())
}

/// 退出灵动岛模式
pub fn exit_dynamic_island_mode(app: &tauri::AppHandle) -> Result<(), String> {
    let _guard = begin_capsule_operation()?;
    let initial_state = DynamicIslandStateMachine::global().get_state();
    if is_transient_state(initial_state) {
        log::warn!("灵动岛模式状态卡在 {:?}，退出时执行紧急恢复", initial_state);
        return emergency_reset_dynamic_island_inner(app);
    }

    let result = exit_dynamic_island_mode_inner(app);
    if initial_state == DynamicIslandState::Capsule {
        if let Err(error) = &result {
            reset_after_failure(app, "退出灵动岛模式", error);
        }
    }
    result
}

fn exit_dynamic_island_mode_inner(app: &tauri::AppHandle) -> Result<(), String> {
    let state_machine = DynamicIslandStateMachine::global();

    let current_state = state_machine.get_state();
    if current_state != DynamicIslandState::Capsule {
        return Err(format!("当前状态 {:?} 不允许退出灵动岛模式", current_state));
    }

    state_machine
        .transition_to(DynamicIslandState::Exiting)
        .map_err(|e| e.to_string())?;
    emit_capsule_state(app, DynamicIslandState::Exiting);

    let main = main_window(app).map_err(|e| format!("{e}，无法退出灵动岛模式"))?;

    state_machine
        .transition_to(DynamicIslandState::Restoring)
        .map_err(|e| e.to_string())?;
    emit_capsule_state(app, DynamicIslandState::Restoring);

    let _ = native_window::hide();

    restore_main_window_basics(&main).map_err(|e| e.to_string())?;
    restore_main_window_snapshot(&main, state_machine.get_snapshot());

    let _ = main.show();
    let _ = main.set_focus();

    state_machine
        .transition_to(DynamicIslandState::Idle)
        .map_err(|e| e.to_string())?;

    state_machine.clear_snapshot();

    emit_capsule_state(app, DynamicIslandState::Idle);
    emit_capsule_mode(app, false);
    log::info!("成功退出灵动岛模式");

    Ok(())
}

/// 紧急恢复灵动岛模式状态和主窗口基础属性
pub fn emergency_reset_dynamic_island(app: &tauri::AppHandle) -> Result<(), String> {
    CapsuleOperationGuard::force_release();
    let _guard = begin_capsule_operation()?;
    emergency_reset_dynamic_island_inner(app)
}

fn emergency_reset_dynamic_island_inner(app: &tauri::AppHandle) -> Result<(), String> {
    let state_machine = DynamicIslandStateMachine::global();
    let snapshot = state_machine.get_snapshot();

    let _ = native_window::hide();

    if let Some(main) = app.get_webview_window("main") {
        let _ = restore_main_window_basics(&main);
        restore_main_window_snapshot(&main, snapshot);

        let _ = main.show();
        let _ = main.set_focus();
    }

    state_machine.emergency_reset().map_err(|e| e.to_string())?;
    emit_capsule_state(app, DynamicIslandState::Idle);
    emit_capsule_mode(app, false);
    Ok(())
}

fn restore_main_window_basics(main: &WebviewWindow) -> Result<(), String> {
    let batch = WindowOperationBatch::builder()
        .always_on_top(false)
        .resizable(true)
        .min_size(Some(Size::Logical(LogicalSize::new(
            MAIN_MIN_WIDTH,
            MAIN_MIN_HEIGHT,
        ))))
        .decorations(false)
        .background_color(OPAQUE_WHITE)
        .skip_taskbar(false)
        .build();

    batch.execute(main).map_err(|e| e.to_string())
}

fn restore_main_window_snapshot(main: &WebviewWindow, snapshot: Option<WindowSnapshot>) {
    if let Some(snapshot) = snapshot {
        log::info!("恢复窗口快照: {:?}", snapshot);
        let _ = main.set_position(Position::Physical(snapshot.position));
        let _ = main.set_size(Size::Physical(snapshot.size));
        if snapshot.maximized {
            let _ = main.maximize();
        }
    } else {
        log::warn!("未找到窗口快照，跳过恢复");
    }
}

/// 吸附灵动岛窗口到最佳位置
pub fn snap_dynamic_island_mode(_app: &tauri::AppHandle) -> Result<(), String> {
    let _guard = begin_capsule_operation()?;
    let state_machine = DynamicIslandStateMachine::global();

    if state_machine.get_state() != DynamicIslandState::Capsule {
        return Ok(());
    }

    native_window::snap()?;
    log::info!("重新吸附原生灵动岛");

    Ok(())
}

/// 检查是否处于灵动岛模式
pub fn is_dynamic_island_mode() -> bool {
    DynamicIslandStateMachine::global().get_state() == DynamicIslandState::Capsule
}
