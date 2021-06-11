use crate::backends::linux::keysym_to_w3c_keycode;
use keyboard_types::KeyState;
use keystroke_decoder::KeystrokeDecoder;

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
            keystroke_decoder::KeyDirection::Up => KeyState::Up,
            keystroke_decoder::KeyDirection::Down => KeyState::Down,
        };
        let event = crate::definitions::SeatEvent::Keyboard(crate::definitions::KeyboardEvent { key, state });
        events.push(crate::definitions::Event::Seat { id, event });
    }

    events
}
