mod keyboard_request;
pub use keyboard_request::*;

mod cursor_request;
pub use cursor_request::*;

mod touch_request;
pub use touch_request::*;

mod gamepad_request;
pub use gamepad_request::*;

pub enum SeatRequest {
    Keyboard(KeyboardRequest),
    Cursor(CursorRequest),
    Touch(TouchRequest),
    Gamepad(GamepadRequest)
}


