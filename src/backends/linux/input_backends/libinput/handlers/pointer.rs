use crate::backends::linux::keysym_to_button;
use input::event::pointer;
use input::event::pointer::Axis;
use keyboard_types::KeyState;
use keystroke_decoder::KeystrokeDecoder;

use crate::definitions::*;
pub fn handle_pointer<S>(
    keystroke_decoder: &mut KeystrokeDecoder,
    id: crate::definitions::SeatId,
    event: pointer::PointerEvent,
) -> Vec<crate::definitions::Event<S>> {
    let mut events = Vec::new();
    //let keystrokes = keystroke_decoder.decode(event.key());
    match event {
        pointer::PointerEvent::Motion(motion_event) => {
            let event = SeatEventType::Cursor(CursorEvent::Moved {
                position: (motion_event.dx(), motion_event.dy()),
            });
            events.push(Event::Seat { id, event });
        }

        //pointer::PointerEvent::MotionAbsolute(motion_absolute_event),
        pointer::PointerEvent::Button(button_event) => {
            let keystrokes = keystroke_decoder.decode(button_event.button());
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
                let event = SeatEventType::Cursor(CursorEvent::Button { key, state });
                events.push(Event::Seat { id, event });
            }
        }
        pointer::PointerEvent::Axis(axis_event) => {
            let event = SeatEventType::Cursor(CursorEvent::Axis {
                value: (
                    axis_event.axis_value(Axis::Horizontal),
                    axis_event.axis_value(Axis::Vertical),
                ),
            });
            events.push(Event::Seat { id, event });
        }
        _ => {}
    }
    events
}
