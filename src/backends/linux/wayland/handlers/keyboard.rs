use super::DispatchContext;
use crate::backends::linux::keysym_to_w3c_keycode;
use keyboard_types::KeyState;
use smithay_client_toolkit::reexports::client::{
    protocol::{wl_keyboard, wl_keyboard::WlKeyboard},
    Main,
};

use crate::definitions::*;

pub fn handle_keyboard(id: crate::definitions::SeatId, keyboard: Main<WlKeyboard>) {
    keyboard.quick_assign(move |_handle, event, mut dispatch_data| {
        let dispatch_context = dispatch_data.get::<DispatchContext>().unwrap();
        match event {
            wl_keyboard::Event::Key {
                serial: _,
                time: _,
                key,
                state: _,
            } => {
                let keystrokes = dispatch_context.keystroke_decoder.decode(key);
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
                    let event = SeatEvent::Keyboard(KeyboardEvent { key, state });
                    dispatch_context.events.push(Event::Seat { id, event });
                }
            }
            /*
            Event::Modifiers {
                serial: u32,
                mods_depressed: u32,
                mods_latched: u32,
                mods_locked: u32,
                group: u32,
            },
            */
            _ => {}
        }
    });
}
