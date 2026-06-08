use std::{
    sync::{Condvar, Mutex, OnceLock},
    thread,
    time::{Duration, Instant},
};

#[cfg(all(windows, not(test)))]
use log::info;

#[derive(Debug, Default)]
struct WindowsPanelPlatformLoopThreadState {
    thread_started: bool,
    thread_id: Option<u32>,
    wake_generation: u64,
    processed_generation: u64,
}

#[derive(Debug, Default)]
struct WindowsPanelPlatformLoopController {
    state: Mutex<WindowsPanelPlatformLoopThreadState>,
    condvar: Condvar,
}

#[derive(Debug, Default)]
struct WindowsPanelDelayedWakeState {
    thread_started: bool,
    deadline: Option<Instant>,
}

#[derive(Debug, Default)]
struct WindowsPanelDelayedWakeController {
    state: Mutex<WindowsPanelDelayedWakeState>,
    condvar: Condvar,
}

static WINDOWS_PANEL_PLATFORM_LOOP_CONTROLLER: OnceLock<WindowsPanelPlatformLoopController> =
    OnceLock::new();
static WINDOWS_PANEL_DELAYED_WAKE_CONTROLLER: OnceLock<WindowsPanelDelayedWakeController> =
    OnceLock::new();

fn windows_panel_platform_loop_controller() -> &'static WindowsPanelPlatformLoopController {
    WINDOWS_PANEL_PLATFORM_LOOP_CONTROLLER.get_or_init(WindowsPanelPlatformLoopController::default)
}

fn windows_panel_delayed_wake_controller() -> &'static WindowsPanelDelayedWakeController {
    WINDOWS_PANEL_DELAYED_WAKE_CONTROLLER.get_or_init(WindowsPanelDelayedWakeController::default)
}

#[cfg(all(windows, not(test)))]
const WINDOWS_PANEL_LOOP_WAKE_MESSAGE: u32 = 0x8001;

pub fn ensure_windows_native_platform_loop_thread(pump_runtime_once: fn() -> Result<(), String>) {
    let controller = windows_panel_platform_loop_controller();
    let mut state = match controller.state.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    if state.thread_started {
        return;
    }
    state.thread_started = true;
    thread::spawn(move || run_windows_native_platform_loop_thread(pump_runtime_once));
}

pub fn platform_loop_thread_started() -> bool {
    windows_panel_platform_loop_controller()
        .state
        .lock()
        .map(|state| state.thread_started)
        .unwrap_or(false)
}

pub fn wake_windows_native_platform_loop() {
    let controller = windows_panel_platform_loop_controller();
    if let Ok(mut state) = controller.state.lock() {
        if !state.thread_started {
            return;
        }
        state.wake_generation += 1;
        #[cfg(all(windows, not(test)))]
        if let Some(thread_id) = state.thread_id {
            unsafe {
                let _ = windows_sys::Win32::UI::WindowsAndMessaging::PostThreadMessageW(
                    thread_id,
                    WINDOWS_PANEL_LOOP_WAKE_MESSAGE,
                    0,
                    0,
                );
            }
        }
        controller.condvar.notify_one();
    }
}

pub fn schedule_windows_native_platform_loop_wake(delay_ms: u64) {
    let controller = windows_panel_delayed_wake_controller();
    let mut state = match controller.state.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    let deadline = Instant::now() + Duration::from_millis(delay_ms);
    state.deadline = Some(match state.deadline {
        Some(current) => current.min(deadline),
        None => deadline,
    });
    if !state.thread_started {
        state.thread_started = true;
        thread::spawn(run_windows_native_platform_loop_delayed_wake_thread);
    }
    controller.condvar.notify_one();
}

fn run_windows_native_platform_loop_delayed_wake_thread() {
    loop {
        let controller = windows_panel_delayed_wake_controller();
        let mut state = match controller.state.lock() {
            Ok(guard) => guard,
            Err(_) => return,
        };
        while state.deadline.is_none() {
            state = match controller.condvar.wait(state) {
                Ok(guard) => guard,
                Err(_) => return,
            };
        }

        let deadline = state.deadline.expect("checked deadline");
        let now = Instant::now();
        if now < deadline {
            let timeout = deadline.saturating_duration_since(now);
            let waited = controller.condvar.wait_timeout(state, timeout);
            match waited {
                Ok((next_state, _)) => {
                    drop(next_state);
                    continue;
                }
                Err(_) => return,
            }
        }

        state.deadline = None;
        drop(state);
        wake_windows_native_platform_loop();
    }
}

#[cfg(all(windows, not(test)))]
fn run_windows_native_platform_loop_thread(pump_runtime_once: fn() -> Result<(), String>) {
    use std::mem::MaybeUninit;
    use windows_sys::Win32::{
        System::Threading::GetCurrentThreadId,
        UI::WindowsAndMessaging::{
            DispatchMessageW, MsgWaitForMultipleObjectsEx, PeekMessageW, TranslateMessage, MSG,
            MWMO_INPUTAVAILABLE, PM_NOREMOVE, PM_REMOVE, QS_ALLINPUT, WM_QUIT,
        },
    };

    const WAIT_TIMEOUT_STATUS: u32 = 258;

    unsafe {
        let mut bootstrap = MaybeUninit::<MSG>::zeroed();
        let _ = PeekMessageW(bootstrap.as_mut_ptr(), 0 as _, 0, 0, PM_NOREMOVE);
    }

    let controller = windows_panel_platform_loop_controller();
    if let Ok(mut state) = controller.state.lock() {
        state.thread_id = Some(unsafe { GetCurrentThreadId() });
        controller.condvar.notify_all();
    } else {
        return;
    }
    if windows_native_hover_probe_enabled() {
        info!("windows native platform loop thread started");
    }

    loop {
        loop {
            let mut message = unsafe { std::mem::zeroed::<MSG>() };
            let has_message = unsafe { PeekMessageW(&mut message, 0 as _, 0, 0, PM_REMOVE) };
            if has_message == 0 {
                break;
            }
            if message.message == WM_QUIT {
                return;
            }
            if message.message == WINDOWS_PANEL_LOOP_WAKE_MESSAGE {
                pump_windows_native_platform_loop_and_notify(controller, pump_runtime_once);
                continue;
            }

            unsafe {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
            pump_windows_native_platform_loop_and_notify(controller, pump_runtime_once);
        }

        let wait_result = unsafe {
            MsgWaitForMultipleObjectsEx(
                0,
                std::ptr::null(),
                crate::state::HOVER_POLL_MS as u32,
                QS_ALLINPUT,
                MWMO_INPUTAVAILABLE,
            )
        };
        if wait_result == WAIT_TIMEOUT_STATUS {
            pump_windows_native_platform_loop_and_notify(controller, pump_runtime_once);
        }
    }
}

#[cfg(all(windows, not(test)))]
fn windows_native_hover_probe_enabled() -> bool {
    std::env::var("ECHOISLAND_WINDOWS_HOVER_PROBE")
        .ok()
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
}

#[cfg(all(windows, not(test)))]
fn pump_windows_native_platform_loop_and_notify(
    controller: &WindowsPanelPlatformLoopController,
    pump_runtime_once: fn() -> Result<(), String>,
) {
    let wake_generation = controller
        .state
        .lock()
        .ok()
        .map(|state| state.wake_generation)
        .unwrap_or(0);
    let _ = pump_runtime_once();
    if let Ok(mut state) = controller.state.lock() {
        state.processed_generation = wake_generation;
        controller.condvar.notify_all();
    }
}

#[cfg(any(not(windows), test))]
fn run_windows_native_platform_loop_thread(pump_runtime_once: fn() -> Result<(), String>) {
    loop {
        let wake_generation = {
            let controller = windows_panel_platform_loop_controller();
            let mut state = match controller.state.lock() {
                Ok(guard) => guard,
                Err(_) => return,
            };
            while state.wake_generation == state.processed_generation {
                state = match controller.condvar.wait(state) {
                    Ok(guard) => guard,
                    Err(_) => return,
                };
            }
            state.wake_generation
        };

        let _ = pump_runtime_once();

        let controller = windows_panel_platform_loop_controller();
        if let Ok(mut state) = controller.state.lock() {
            state.processed_generation = wake_generation;
            controller.condvar.notify_all();
        } else {
            return;
        }
    }
}

pub fn windows_native_platform_loop_generations() -> Option<(u64, u64)> {
    windows_panel_platform_loop_controller()
        .state
        .lock()
        .ok()
        .map(|state| (state.wake_generation, state.processed_generation))
}

pub fn wait_windows_native_platform_loop_processed_at_least(
    target_generation: u64,
    timeout_ms: u64,
) -> bool {
    let controller = windows_panel_platform_loop_controller();
    let Ok(state) = controller.state.lock() else {
        return false;
    };
    let waited =
        controller
            .condvar
            .wait_timeout_while(state, Duration::from_millis(timeout_ms), |state| {
                state.processed_generation < target_generation
            });
    match waited {
        Ok((state, _)) => state.processed_generation >= target_generation,
        Err(_) => false,
    }
}
