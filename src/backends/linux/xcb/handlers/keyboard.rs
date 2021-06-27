use crate::backends::linux::keysym_to_w3c_keycode;
use keystroke_decoder::KeystrokeDecoder;
use crate::definitions::{Event,SeatEventType,SeatEvent,KeyboardEvent,State};

pub fn handle_keyboard(
    keystroke_decoder: &mut KeystrokeDecoder,
    id: crate::definitions::SeatId,
    keycode: u32,
) -> Vec<crate::definitions::Event> {
    let mut events = Vec::new();

    let keystrokes = keystroke_decoder.decode(keycode - 8);
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

        let event_type = SeatEventType::Keyboard(KeyboardEvent::Key{key,state});
        let event = SeatEvent::from((id,event_type));
        events.push(Event::Seat(event));
    }

    events
}
