use crate::backends::linux::keysym_to_w3c_keycode;
use keystroke_decoder::KeystrokeDecoder;
use crate::definitions::{Event,SeatEvent,KeyboardEvent,State};

pub fn handle_keyboard(
    keystroke_decoder: &mut KeystrokeDecoder,
    id: crate::definitions::SeatId,
    code: u32,
    serial: u32,
    time: u32
) -> Vec<crate::definitions::Event> {
    let mut events = Vec::new();

    let keystrokes = keystroke_decoder.decode(code - 8);
    for (keysym, direction) in keystrokes.as_keysyms() {
        let key = keysym_to_w3c_keycode(keysym);

        let state = match direction {
            keystroke_decoder::KeyDirection::Up => State::Up,
            keystroke_decoder::KeyDirection::Down => State::Down,
        };

        let event = SeatEvent::Keyboard(KeyboardEvent::Key{code,key,state,serial,time});
        events.push(Event::Seat{time,id,event});    }

    events
}
