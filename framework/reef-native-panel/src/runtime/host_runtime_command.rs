use crate::state::PanelRect;

use crate::presentation::render::{
    sync_native_panel_host_window_screen_frame, sync_native_panel_host_window_shared_body_height,
    sync_native_panel_host_window_timeline, sync_native_panel_host_window_visibility,
    NativePanelHostWindowDescriptor, NativePanelHostWindowState, NativePanelPointerRegion,
    NativePanelRuntimeInputDescriptor, NativePanelTimelineDescriptor,
};

pub(crate) trait NativePanelRuntimeHostState {
    fn host_window_descriptor_mut(&mut self) -> &mut NativePanelHostWindowDescriptor;

    fn pointer_regions_mut(&mut self) -> &mut Vec<NativePanelPointerRegion>;
}

pub(crate) trait NativePanelRuntimeHostController {
    type Error;

    fn runtime_host_create_panel(&mut self) -> Result<(), Self::Error>;

    fn runtime_host_hide_panel(&mut self) -> Result<(), Self::Error>;

    fn runtime_host_reposition(
        &mut self,
        reposition: NativePanelHostDisplayReposition,
    ) -> Result<(), Self::Error>;

    fn runtime_host_set_shared_body_height(&mut self, body_height: f64) -> Result<(), Self::Error>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct NativePanelHostDisplayReposition {
    pub(crate) preferred_display_index: usize,
    pub(crate) screen_frame: Option<PanelRect>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum NativePanelRuntimeHostCommand {
    Create,
    Hide,
    Reposition(NativePanelHostDisplayReposition),
    SetSharedBodyHeight(f64),
}

pub(crate) fn native_panel_host_display_reposition(
    preferred_display_index: usize,
    screen_frame: Option<PanelRect>,
) -> NativePanelHostDisplayReposition {
    NativePanelHostDisplayReposition {
        preferred_display_index,
        screen_frame,
    }
}

pub(crate) fn native_panel_host_display_reposition_from_input_descriptor(
    input: &NativePanelRuntimeInputDescriptor,
) -> NativePanelHostDisplayReposition {
    native_panel_host_display_reposition(input.selected_display_index(), input.screen_frame)
}

pub(crate) fn native_panel_runtime_host_reposition_command_from_input_descriptor(
    input: &NativePanelRuntimeInputDescriptor,
) -> NativePanelRuntimeHostCommand {
    NativePanelRuntimeHostCommand::Reposition(
        native_panel_host_display_reposition_from_input_descriptor(input),
    )
}

pub(crate) fn execute_native_panel_runtime_host_command<H>(
    host: &mut H,
    command: NativePanelRuntimeHostCommand,
) -> Result<(), H::Error>
where
    H: NativePanelRuntimeHostController,
{
    match command {
        NativePanelRuntimeHostCommand::Create => host.runtime_host_create_panel(),
        NativePanelRuntimeHostCommand::Hide => host.runtime_host_hide_panel(),
        NativePanelRuntimeHostCommand::Reposition(reposition) => {
            host.runtime_host_reposition(reposition)
        }
        NativePanelRuntimeHostCommand::SetSharedBodyHeight(body_height) => {
            host.runtime_host_set_shared_body_height(body_height)
        }
    }
}

pub(crate) fn create_native_panel_via_host_controller<H>(host: &mut H) -> Result<(), H::Error>
where
    H: NativePanelRuntimeHostController,
{
    execute_native_panel_runtime_host_command(host, NativePanelRuntimeHostCommand::Create)
}

pub(crate) fn hide_native_panel_via_host_controller<H>(host: &mut H) -> Result<(), H::Error>
where
    H: NativePanelRuntimeHostController,
{
    execute_native_panel_runtime_host_command(host, NativePanelRuntimeHostCommand::Hide)
}

pub(crate) fn reposition_native_panel_host_from_input_descriptor_via_controller<H>(
    host: &mut H,
    input: &NativePanelRuntimeInputDescriptor,
) -> Result<(), H::Error>
where
    H: NativePanelRuntimeHostController,
{
    execute_native_panel_runtime_host_command(
        host,
        native_panel_runtime_host_reposition_command_from_input_descriptor(input),
    )
}

pub(crate) fn set_native_panel_host_shared_body_height_via_controller<H>(
    host: &mut H,
    body_height: f64,
) -> Result<(), H::Error>
where
    H: NativePanelRuntimeHostController,
{
    execute_native_panel_runtime_host_command(
        host,
        NativePanelRuntimeHostCommand::SetSharedBodyHeight(body_height),
    )
}

pub(crate) fn sync_native_panel_host_display_reposition(
    descriptor: &mut NativePanelHostWindowDescriptor,
    reposition: NativePanelHostDisplayReposition,
) {
    sync_native_panel_host_window_screen_frame(
        descriptor,
        reposition.preferred_display_index,
        reposition.screen_frame,
    );
}
