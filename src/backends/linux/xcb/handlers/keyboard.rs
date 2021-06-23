use crate::backends::linux::keysym_to_w3c_keycode;
use keystroke_decoder::KeystrokeDecoder;
use crate::definitions::{Event,SeatEventType,SeatEvent,KeyboardEvent};

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
        let event_type = match direction {
            keystroke_decoder::KeyDirection::Up => SeatEventType::Keyboard(KeyboardEvent::KeyRelease{key}),
            keystroke_decoder::KeyDirection::Down => SeatEventType::Keyboard(KeyboardEvent::KeyPress{key}),
        };
        let event = SeatEvent::from((id,event_type));
        events.push(Event::Seat(event));
    }

    events
}
