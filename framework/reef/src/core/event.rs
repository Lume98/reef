use crate::core::geometry::Point;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Pointer(PointerEvent),
    Keyboard(KeyboardEvent),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerButton {
    Primary,
    Secondary,
    Middle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerEventKind {
    Move,
    Press,
    Release,
    Enter,
    Leave,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PointerEvent {
    pub kind: PointerEventKind,
    pub position: Point,
    pub button: Option<PointerButton>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardEventKind {
    Press,
    Release,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyboardEvent {
    pub kind: KeyboardEventKind,
    pub key: String,
    pub modifiers: Modifiers,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventResponse {
    Ignored,
    Consumed,
}
