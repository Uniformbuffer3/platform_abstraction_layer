mod cursor_event;
pub use cursor_event::{Button, CursorEvent};
mod touch_event;
pub use touch_event::TouchEvent;
mod keyboard_event;
pub use keyboard_event::KeyboardEvent;

#[derive(Clone,Debug,PartialEq)]
pub struct SeatEvent {
    pub id: SeatId,
    pub event_type: SeatEventType,
}
impl From<(SeatId,SeatEventType)> for SeatEvent {
    fn from(tuple: (SeatId,SeatEventType))->Self {Self{id: tuple.0,event_type: tuple.1}}
}

#[derive(Clone,Debug,PartialEq)]
pub enum SeatEventType {
    Added(SeatInfo),
    Changed(SeatCapability),
    Removed,

    Keyboard(KeyboardEvent),
    Cursor(CursorEvent),
    Touch(TouchEvent),
}

impl From<KeyboardEvent> for SeatEventType {
    fn from(event: KeyboardEvent) -> Self {
        Self::Keyboard(event)
    }
}

impl From<CursorEvent> for SeatEventType {
    fn from(event: CursorEvent) -> Self {
        Self::Cursor(event)
    }
}

impl From<TouchEvent> for SeatEventType {
    fn from(event: TouchEvent) -> Self {
        Self::Touch(event)
    }
}

impl From<SeatCapability> for SeatEventType {
    fn from(event: SeatCapability) -> Self {
        Self::Changed(event)
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum SeatCapability {
    PointerAdded,
    PointerRemoved,
    KeyboardAdded,
    KeyboardRemoved,
    TouchAdded,
    TouchRemoved,
}

#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub struct SeatId(u32);
impl From<u32> for SeatId {
    fn from(hash: u32) -> Self {
        Self(hash)
    }
}
impl Eq for SeatId {}

#[derive(Debug,Clone, PartialEq)]
pub struct SeatInfo {
    pub id: SeatId,
    pub name: String,
    pub has_pointer: bool,
    pub has_keyboard: bool,
    pub has_touch: bool,
}
