use super::DispatchContext;
use crate::backends::linux::keysym_to_button;
use keyboard_types::KeyState;
use smithay_client_toolkit::reexports::client::{
    protocol::{
        wl_pointer,
        wl_pointer::{Axis, WlPointer},
    },
    Main,
};

use crate::definitions::*;
pub fn handle_pointer<S: 'static>(id: crate::definitions::SeatId, pointer: Main<WlPointer>) {
    pointer.quick_assign(move |_handle, event, mut dispatch_data| {
        let dispatch_context = dispatch_data.get::<DispatchContext<S>>().unwrap();
        match event {
            wl_pointer::Event::Enter {
                serial: _,
                surface,
                surface_x,
                surface_y,
            } => {
                let event = SeatEvent::Cursor(CursorEvent::Entered {
                    surface_id: surface.as_ref().id().into(),
                    position: (surface_x, surface_y),
                });
                dispatch_context.events.push(Event::Seat { id, event });
            }

            wl_pointer::Event::Leave { serial: _, surface } => {
                let event = SeatEvent::Cursor(CursorEvent::Left {
                    surface_id: surface.as_ref().id().into(),
                });
                dispatch_context.events.push(Event::Seat { id, event });
            }
            wl_pointer::Event::Motion {
                time: _,
                surface_x,
                surface_y,
            } => {
                let event = SeatEvent::Cursor(CursorEvent::Moved {
                    position: (surface_x, surface_y),
                });
                dispatch_context.events.push(Event::Seat { id, event });
            }
            wl_pointer::Event::Button {
                serial: _,
                time: _,
                button,
                state: _,
            } => {
                let keystrokes = dispatch_context.keystroke_decoder.decode(button);
                for (keysym, direction) in keystrokes.as_keysyms() {
                    let key = if let Some(key) = keysym_to_button(keysym) {
                        key
                    } else {
                        continue;
                    };

                    let state = match direction {
                        keystroke_decoder::KeyDirection::Up => KeyState::Up,
                        keystroke_decoder::KeyDirection::Down => KeyState::Down,
                    };
                    let event = SeatEvent::Cursor(CursorEvent::Button { key, state });
                    dispatch_context.events.push(Event::Seat { id, event });
                }
            }
            wl_pointer::Event::Axis {
                time: _,
                axis,
                value,
            } => match axis {
                Axis::VerticalScroll => {
                    let event = SeatEvent::Cursor(CursorEvent::Axis {
                        value: (value, 0.0),
                    });
                    dispatch_context.events.push(Event::Seat { id, event });
                }
                Axis::HorizontalScroll => {
                    let event = SeatEvent::Cursor(CursorEvent::Axis {
                        value: (0.0, value),
                    });
                    dispatch_context.events.push(Event::Seat { id, event });
                }
                _ => (),
            },
            _ => {}
        }
    });
}
