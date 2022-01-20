mod cursor_event;
pub use cursor_event::*;
mod touch_event;
pub use touch_event::*;
mod keyboard_event;
pub use keyboard_event::*;
mod gamepad_event;
pub use gamepad_event::*;

#[derive(Clone,Debug,PartialEq)]
/// Possible seat events.
pub enum SeatEvent {
    Added{
        name: String,
    },
    Removed,

    Keyboard(KeyboardEvent),
    Cursor(CursorEvent),
    Touch(TouchEvent),
    Gamepad(GamepadEvent)
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

#[derive(Debug, PartialEq, Hash, Copy, Clone)]
/// Seat identifier.
pub struct SeatId(usize);
impl Into<usize> for SeatId {
    fn into(self) -> usize {
        self.0
    }
}
impl From<usize> for SeatId {
    fn from(hash: usize) -> Self {
        Self(hash)
    }
}
impl From<u32> for SeatId {
    fn from(id: u32) -> Self {
        Self(id as usize)
    }
}
impl From<i32> for SeatId {
    fn from(id: i32) -> Self {
        Self(id as usize)
    }
}
impl Eq for SeatId {}

#[derive(Debug,Clone, PartialEq)]
/// Seat informations.
pub struct SeatInfo {
    pub id: SeatId,
    pub name: String,
}
