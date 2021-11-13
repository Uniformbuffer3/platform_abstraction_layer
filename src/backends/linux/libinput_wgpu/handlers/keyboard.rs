use crate::backends::linux::keysym_to_w3c_keycode;
use keystroke_decoder::KeystrokeDecoder;

use crate::definitions::*;
use input::event::keyboard;
use input::event::keyboard::KeyboardEventTrait;

pub fn handle_keyboard<S>(
    keystroke_decoder: &mut KeystrokeDecoder,
    id: crate::definitions::SeatId,
    event: keyboard::KeyboardEvent,
) -> Vec<crate::definitions::Event> {
    let mut events = Vec::new();
    let keystrokes = keystroke_decoder.decode(event.key());
    for (keysym, direction) in keystrokes.as_keysyms() {
        let key = if let Some(key) = keysym_to_w3c_keycode(keysym) {
            key
        } else {
            continue;
        };
        let state = match direction {
            keystroke_decoder::KeyDirection::Up => State::Up,
            keystroke_decoder::KeyDirection::Down => State::Down,
        };

        let event = SeatEvent::Keyboard(KeyboardEvent::Key{key,state});
        let event = SeatEvent::from((id,event));
        events.push(Event::Seat(event));
    }
    events
}
