mod cursor_event;
pub use cursor_event::{Button, CursorEvent};
mod touch_event;
pub use touch_event::TouchEvent;
mod keyboard_event;
pub use keyboard_event::KeyboardEvent;
/*
#[derive(Debug)]
pub struct SeatEvent {
    seat: crate::definitions::SeatId,
    event: SeatEventInner,
}
impl From<(crate::definitions::SeatId, SeatEventInner)> for SeatEvent {
    fn from(tuple: (crate::definitions::SeatId, SeatEventInner)) -> Self {
        let seat = tuple.0;
        let event = tuple.1;
        Self { seat, event }
    }
}
*/
#[derive(Debug,PartialEq)]
pub enum SeatEvent {
    SeatAdded(crate::definitions::SeatInfo),
    SeatChanged(SeatCapability),
    SeatRemoved,

    Keyboard(KeyboardEvent),
    Cursor(CursorEvent),
    Touch(TouchEvent),
}

impl From<KeyboardEvent> for SeatEvent {
    fn from(event: KeyboardEvent) -> Self {
        Self::Keyboard(event)
    }
}

impl From<CursorEvent> for SeatEvent {
    fn from(event: CursorEvent) -> Self {
        Self::Cursor(event)
    }
}

impl From<TouchEvent> for SeatEvent {
    fn from(event: TouchEvent) -> Self {
        Self::Touch(event)
    }
}

impl From<SeatCapability> for SeatEvent {
    fn from(event: SeatCapability) -> Self {
        Self::SeatChanged(event)
    }
}

#[derive(Debug,PartialEq)]
pub enum SeatCapability {
    PointerAdded,
    PointerRemoved,
    KeyboardAdded,
    KeyboardRemoved,
    TouchAdded,
    TouchRemoved,
}
