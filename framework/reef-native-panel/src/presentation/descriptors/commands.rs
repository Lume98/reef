use crate::state::{
    HoverTransition, PanelHitAction, PanelHitTarget, PanelInteractionCommand, PanelPoint,
};

// ---- pointer input / platform event / runtime command types ----

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NativePanelPointerInput {
    Move(PanelPoint),
    Click(PanelPoint),
    Leave,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativePanelPlatformEvent {
    FocusSession(String),
    ToggleSettingsSurface,
    QuitApplication,
    CycleDisplay,
    CycleIslandWidth,
    CycleLanguage,
    ToggleCompletionSound,
    ToggleMascot,
    DebugModeTrigger,
    OpenSettingsLocation,
    OpenReleasePage,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativePanelRuntimeCommand {
    FocusSession(String),
    ToggleSettingsSurface,
    QuitApplication,
    CycleDisplay,
    CycleIslandWidth,
    CycleLanguage,
    ToggleCompletionSound,
    ToggleMascot,
    DebugModeTrigger,
    OpenSettingsLocation,
    OpenReleasePage,
}

// ---- pointer input outcome ----

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativePanelPointerInputOutcome {
    Hover(Option<HoverTransition>),
    Click(Option<NativePanelPlatformEvent>),
}

impl NativePanelPointerInputOutcome {
    pub fn into_hover_transition(self) -> Option<HoverTransition> {
        match self {
            NativePanelPointerInputOutcome::Hover(transition) => transition,
            NativePanelPointerInputOutcome::Click(_) => None,
        }
    }

    pub fn into_click_event(self) -> Option<NativePanelPlatformEvent> {
        match self {
            NativePanelPointerInputOutcome::Click(event) => event,
            NativePanelPointerInputOutcome::Hover(_) => None,
        }
    }
}

// ---- command capability / handler traits ----

pub trait NativePanelRuntimeCommandCapability {
    type Error;

    fn focus_session(&mut self, session_id: String) -> Result<(), Self::Error>;
    fn toggle_settings_surface(&mut self) -> Result<(), Self::Error>;
    fn quit_application(&mut self) -> Result<(), Self::Error>;
    fn cycle_display(&mut self) -> Result<(), Self::Error>;
    fn cycle_island_width(&mut self) -> Result<(), Self::Error>;
    fn cycle_language(&mut self) -> Result<(), Self::Error>;
    fn toggle_completion_sound(&mut self) -> Result<(), Self::Error>;
    fn toggle_mascot(&mut self) -> Result<(), Self::Error>;
    fn debug_mode_trigger(&mut self) -> Result<(), Self::Error>;
    fn open_settings_location(&mut self) -> Result<(), Self::Error>;
    fn open_release_page(&mut self) -> Result<(), Self::Error>;
}

pub trait NativePanelRuntimeCommandHandler: NativePanelRuntimeCommandCapability {
    fn execute_runtime_command(
        &mut self,
        command: NativePanelRuntimeCommand,
    ) -> Result<(), Self::Error> {
        match command {
            NativePanelRuntimeCommand::FocusSession(session_id) => self.focus_session(session_id),
            NativePanelRuntimeCommand::ToggleSettingsSurface => self.toggle_settings_surface(),
            NativePanelRuntimeCommand::QuitApplication => self.quit_application(),
            NativePanelRuntimeCommand::CycleDisplay => self.cycle_display(),
            NativePanelRuntimeCommand::CycleIslandWidth => self.cycle_island_width(),
            NativePanelRuntimeCommand::CycleLanguage => self.cycle_language(),
            NativePanelRuntimeCommand::ToggleCompletionSound => self.toggle_completion_sound(),
            NativePanelRuntimeCommand::ToggleMascot => self.toggle_mascot(),
            NativePanelRuntimeCommand::DebugModeTrigger => self.debug_mode_trigger(),
            NativePanelRuntimeCommand::OpenSettingsLocation => self.open_settings_location(),
            NativePanelRuntimeCommand::OpenReleasePage => self.open_release_page(),
        }
    }
}

impl<T> NativePanelRuntimeCommandHandler for T where T: NativePanelRuntimeCommandCapability {}

#[derive(Default)]
pub struct NativePanelQueuedRuntimeCommandHandler {
    events: Vec<NativePanelPlatformEvent>,
}

impl NativePanelQueuedRuntimeCommandHandler {
    pub fn take_events(self) -> Vec<NativePanelPlatformEvent> {
        self.events
    }
}

impl NativePanelRuntimeCommandCapability for NativePanelQueuedRuntimeCommandHandler {
    type Error = String;

    fn focus_session(&mut self, session_id: String) -> Result<(), Self::Error> {
        self.events
            .push(NativePanelPlatformEvent::FocusSession(session_id));
        Ok(())
    }

    fn toggle_settings_surface(&mut self) -> Result<(), Self::Error> {
        self.events
            .push(NativePanelPlatformEvent::ToggleSettingsSurface);
        Ok(())
    }

    fn quit_application(&mut self) -> Result<(), Self::Error> {
        self.events.push(NativePanelPlatformEvent::QuitApplication);
        Ok(())
    }

    fn cycle_display(&mut self) -> Result<(), Self::Error> {
        self.events.push(NativePanelPlatformEvent::CycleDisplay);
        Ok(())
    }

    fn cycle_island_width(&mut self) -> Result<(), Self::Error> {
        self.events.push(NativePanelPlatformEvent::CycleIslandWidth);
        Ok(())
    }

    fn cycle_language(&mut self) -> Result<(), Self::Error> {
        self.events.push(NativePanelPlatformEvent::CycleLanguage);
        Ok(())
    }

    fn toggle_completion_sound(&mut self) -> Result<(), Self::Error> {
        self.events
            .push(NativePanelPlatformEvent::ToggleCompletionSound);
        Ok(())
    }

    fn toggle_mascot(&mut self) -> Result<(), Self::Error> {
        self.events.push(NativePanelPlatformEvent::ToggleMascot);
        Ok(())
    }

    fn debug_mode_trigger(&mut self) -> Result<(), Self::Error> {
        self.events.push(NativePanelPlatformEvent::DebugModeTrigger);
        Ok(())
    }

    fn open_settings_location(&mut self) -> Result<(), Self::Error> {
        self.events
            .push(NativePanelPlatformEvent::OpenSettingsLocation);
        Ok(())
    }

    fn open_release_page(&mut self) -> Result<(), Self::Error> {
        self.events.push(NativePanelPlatformEvent::OpenReleasePage);
        Ok(())
    }
}

// ---- event/command conversion and dispatch ----

pub fn native_panel_runtime_command_for_platform_event(
    event: NativePanelPlatformEvent,
) -> NativePanelRuntimeCommand {
    match event {
        NativePanelPlatformEvent::FocusSession(session_id) => {
            NativePanelRuntimeCommand::FocusSession(session_id)
        }
        NativePanelPlatformEvent::ToggleSettingsSurface => {
            NativePanelRuntimeCommand::ToggleSettingsSurface
        }
        NativePanelPlatformEvent::QuitApplication => NativePanelRuntimeCommand::QuitApplication,
        NativePanelPlatformEvent::CycleDisplay => NativePanelRuntimeCommand::CycleDisplay,
        NativePanelPlatformEvent::CycleIslandWidth => NativePanelRuntimeCommand::CycleIslandWidth,
        NativePanelPlatformEvent::CycleLanguage => NativePanelRuntimeCommand::CycleLanguage,
        NativePanelPlatformEvent::ToggleCompletionSound => {
            NativePanelRuntimeCommand::ToggleCompletionSound
        }
        NativePanelPlatformEvent::ToggleMascot => NativePanelRuntimeCommand::ToggleMascot,
        NativePanelPlatformEvent::DebugModeTrigger => NativePanelRuntimeCommand::DebugModeTrigger,
        NativePanelPlatformEvent::OpenSettingsLocation => {
            NativePanelRuntimeCommand::OpenSettingsLocation
        }
        NativePanelPlatformEvent::OpenReleasePage => NativePanelRuntimeCommand::OpenReleasePage,
    }
}

pub fn dispatch_native_panel_runtime_command<H>(
    handler: &mut H,
    command: NativePanelRuntimeCommand,
) -> Result<(), H::Error>
where
    H: NativePanelRuntimeCommandHandler,
{
    handler.execute_runtime_command(command)
}

pub fn dispatch_native_panel_runtime_commands<H>(
    handler: &mut H,
    commands: impl IntoIterator<Item = NativePanelRuntimeCommand>,
) -> Result<(), H::Error>
where
    H: NativePanelRuntimeCommandHandler,
{
    for command in commands {
        dispatch_native_panel_runtime_command(handler, command)?;
    }
    Ok(())
}

pub fn dispatch_native_panel_platform_event<H>(
    handler: &mut H,
    event: NativePanelPlatformEvent,
) -> Result<(), H::Error>
where
    H: NativePanelRuntimeCommandHandler,
{
    dispatch_native_panel_runtime_command(
        handler,
        native_panel_runtime_command_for_platform_event(event),
    )
}

pub fn dispatch_native_panel_platform_events<H>(
    handler: &mut H,
    events: impl IntoIterator<Item = NativePanelPlatformEvent>,
) -> Result<(), H::Error>
where
    H: NativePanelRuntimeCommandHandler,
{
    dispatch_native_panel_runtime_commands(
        handler,
        events
            .into_iter()
            .map(native_panel_runtime_command_for_platform_event),
    )
}

pub fn native_panel_platform_event_for_hit_target(
    target: &PanelHitTarget,
) -> NativePanelPlatformEvent {
    match target.action {
        PanelHitAction::FocusSession => NativePanelPlatformEvent::FocusSession(
            target
                .session_id()
                .unwrap_or(target.value.as_str())
                .to_string(),
        ),
        PanelHitAction::CycleDisplay => NativePanelPlatformEvent::CycleDisplay,
        PanelHitAction::CycleIslandWidth => NativePanelPlatformEvent::CycleIslandWidth,
        PanelHitAction::CycleLanguage => NativePanelPlatformEvent::CycleLanguage,
        PanelHitAction::ToggleCompletionSound => NativePanelPlatformEvent::ToggleCompletionSound,
        PanelHitAction::ToggleMascot => NativePanelPlatformEvent::ToggleMascot,
        PanelHitAction::OpenSettingsLocation => NativePanelPlatformEvent::OpenSettingsLocation,
        PanelHitAction::OpenReleasePage => NativePanelPlatformEvent::OpenReleasePage,
    }
}

pub fn native_panel_platform_event_for_interaction_command(
    command: &PanelInteractionCommand,
) -> Option<NativePanelPlatformEvent> {
    match command {
        PanelInteractionCommand::HitTarget(target) => {
            Some(native_panel_platform_event_for_hit_target(target))
        }
        PanelInteractionCommand::ToggleSettingsSurface => {
            Some(NativePanelPlatformEvent::ToggleSettingsSurface)
        }
        PanelInteractionCommand::QuitApplication => Some(NativePanelPlatformEvent::QuitApplication),
        PanelInteractionCommand::None => None,
    }
}
