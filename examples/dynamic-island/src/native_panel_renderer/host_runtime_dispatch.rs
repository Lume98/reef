#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NativePanelRuntimeDispatchMode {
    Scheduled,
    Immediate,
}
