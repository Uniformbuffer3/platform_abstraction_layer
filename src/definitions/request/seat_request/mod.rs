use crate::definitions::SeatId;

mod keyboard_request;
pub use keyboard_request::*;

mod cursor_request;
pub use cursor_request::*;

mod touch_request;
pub use touch_request::*;

mod gamepad_request;
pub use gamepad_request::*;

pub struct SeatRequest {
    pub id: SeatId,
    pub event_type: SeatRequestType,
}

pub enum SeatRequestType {
    Keyboard(KeyboardRequest),
    Cursor(CursorRequest),
    Touch(TouchRequest),
    Gamepad(GamepadRequest)
}





