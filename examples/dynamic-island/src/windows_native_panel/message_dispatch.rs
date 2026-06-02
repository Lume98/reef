use crate::native_panel_renderer::facade::shell::{
    pump_native_panel_platform_window_messages, NativePanelPlatformWindowMessagePump,
};

pub(super) fn pump_window_messages<R>(runtime: &mut R) -> Result<(), String>
where
    R: NativePanelPlatformWindowMessagePump,
{
    pump_native_panel_platform_window_messages(runtime)
}
