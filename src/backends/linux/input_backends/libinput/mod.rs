use libc::{O_RDONLY, O_RDWR, O_WRONLY};

use input::{Libinput, LibinputInterface};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::path::Path;

use keystroke_decoder::KeystrokeDecoder;

mod handlers;
use handlers::*;

use input::event::EventTrait;
use input::AsRaw;

struct Interface;
impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into_raw_fd())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: RawFd) {
        unsafe {
            File::from_raw_fd(fd);
        }
    }
}

pub struct LibinputBackend {
    context: Libinput,
    keystroke_decoder: KeystrokeDecoder,
    seats: HashMap<*const input::ffi::libinput_seat, u32>,
    id_counter: u32,
}

impl LibinputBackend {
    pub fn new() -> Result<Self, ()> {
        let context = Libinput::new_with_udev(Interface);
        let keystroke_decoder = KeystrokeDecoder::new();
        let seats = HashMap::new();
        let id_counter = 0;
        Ok(Self {
            context,
            keystroke_decoder,
            seats,
            id_counter,
        })
    }
}

impl crate::definitions::InputBackend for LibinputBackend {
    fn dispatch(&mut self) -> Vec<crate::definitions::Event> {
        self.context.dispatch().unwrap();
        let mut events = Vec::new();

        while let Some(event) = self.context.next() {
            let raw_seat = event.device().seat().as_raw();
            let seat_id: crate::definitions::SeatId = match self.seats.get(&raw_seat) {
                Some(seat_id) => (*seat_id).into(),
                None => {
                    let seat_id = self.id_counter;
                    self.seats.insert(raw_seat, seat_id);
                    self.id_counter += 1;
                    seat_id.into()
                }
            };

            let mut current_events = match event {
                input::event::Event::Device(_device_event) => unimplemented!(),
                input::event::Event::Keyboard(keyboard_event) => {
                    handle_keyboard(&mut self.keystroke_decoder, seat_id, keyboard_event)
                }
                input::event::Event::Pointer(pointer_event) => {
                    handle_pointer(&mut self.keystroke_decoder, seat_id, pointer_event)
                }
                input::event::Event::Touch(_touch_event) => unimplemented!(),
                input::event::Event::Tablet(_table_tool_event) => unimplemented!(),
                input::event::Event::TabletPad(_tablet_pad_event) => unimplemented!(),
                input::event::Event::Gesture(_gesture_event) => unimplemented!(),
                input::event::Event::Switch(_switch_event) => unimplemented!(),
            };
            events.append(&mut current_events);
        }

        events
    }
    fn keyboard_layout(&self) -> String {
        self.keystroke_decoder.layout().clone()
    }
    fn set_keyboard_layout(&mut self, layout: String) {
        self.keystroke_decoder.set_layout(layout);
    }
}
