use crate::backends::partial_backends::output_manager::OutputConstraintKeeper;
use crate::backends::partial_backends::surface_manager::SurfaceManager;
use crate::definitions::*;

use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use input::{Libinput, LibinputInterface,AsRaw,event::EventTrait};
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::path::Path;

use keystroke_decoder::KeystrokeDecoder;
use keyboard_types::KeyState;

use input::event::Event as LibinputEvent;
use input::event::keyboard::KeyboardEventTrait;
use input::event::device::DeviceEvent;
use input::DeviceCapability;
use input::event::pointer;
use input::event::pointer::Axis;

use crate::backends::linux::keysym_to_w3c_keycode;
use crate::backends::linux::keysym_to_button;


use std::collections::HashMap;

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

struct Seat {
    id: SeatId,
    name: String,
    cursor_position: Position2D,
    cursor_mode: CursorMode,
    cursor_visible: bool,
    keyboard_layout: String,
    keyboard_autorepeat: bool,
    keyboards: Vec<String>,
    cursors: Vec<String>,
    touchs: Vec<String>
}

pub struct LibinputWGpuPlatform {
    //output_manager: OutputConstraintKeeper,
    surface_manager: SurfaceManager,
    libinput: Libinput,
    keystroke_decoder: KeystrokeDecoder,
    id_counter: u32,
    seats: HashMap<*const input::ffi::libinput_seat, Seat>,
}
impl LibinputWGpuPlatform{
    pub fn new()->Self {
        //let output_manager = OutputConstraintKeeper::new();
        let surface_manager = SurfaceManager::new();
        let libinput = Libinput::new_with_udev(Interface);
        let keystroke_decoder = KeystrokeDecoder::new();
        let seats = HashMap::new();
        let id_counter = 0;
        Self {//output_manager,
        surface_manager,libinput,keystroke_decoder,id_counter,seats}
    }
}

impl PlatformBackend for LibinputWGpuPlatform {
    fn platform_type(&self)->PlatformType {PlatformType::Direct}
    fn events(&mut self) -> Vec<Event> {
        match self.libinput.events(){
            Ok(_)=>(),
            Err(_)=>()
        }

        let mut new_events = Vec::new();
        while let Some(event) = self.libinput.next() {
            let raw_seat = event.device().seat();
            match event {
                LibinputEvent::Device(device_event) => {
                    match device_event {
                        DeviceEvent::Added(device_event)=>{
                            let device = device_event.device();

                            if !self.seats.contains_key(&raw_seat.as_raw()){
                                let position = self.output_manager.apply_limit(Position2D{x: 0,y: 0},Position2D{x: 0,y: 0});
                                let seat = Seat {
                                    id: SeatId::from(self.id_counter),
                                    name: String::from(raw_seat.logical_name()),
                                    cursor_position: position,
                                    cursor_mode: CursorMode::Relative,
                                    cursor_visible: false,
                                    keyboard_layout: self.keystroke_decoder.layout().clone(),
                                    keyboard_autorepeat: true,
                                    keyboards: Vec::new(),
                                    cursors: Vec::new(),
                                    touchs: Vec::new()
                                };
                                self.id_counter += 1;
                                self.seats.insert(raw_seat.as_raw(),seat);
                            }

                            let seat = match self.seats.get_mut(&raw_seat.as_raw()) {
                                Some(seat)=>seat,
                                None=>continue
                            };

                            if device.has_capability(DeviceCapability::Keyboard){
                                if seat.keyboards.is_empty() {
                                    let id = seat.id;
                                    let info = KeyboardInfo {
                                        layout: seat.keyboard_layout.clone(),
                                        autorepeat: seat.keyboard_autorepeat
                                    };
                                    let event = SeatEvent::Keyboard(KeyboardEvent::Added(info));
                                    let event = SeatEvent::from((id,event));
                                    new_events.push(Event::Seat(event));
                                }
                                seat.keyboards.push(String::from(device.sysname()));
                            }
                            if device.has_capability(DeviceCapability::Pointer){
                                if seat.cursors.is_empty() {
                                    let id = seat.id;
                                    let info = CursorInfo {
                                        mode: seat.cursor_mode,
                                        visible: seat.cursor_visible
                                    };
                                    let event = SeatEvent::Cursor(CursorEvent::Added(info));
                                    let event = SeatEvent::from((id,event));
                                    new_events.push(Event::Seat(event));
                                }
                                seat.cursors.push(String::from(device.sysname()));
                            }
                            if device.has_capability(DeviceCapability::Touch){
                                if seat.touchs.is_empty() {
                                    let id = seat.id;
                                    let info = TouchInfo {

                                    };
                                    let event = SeatEvent::Touch(TouchEvent::Added(info));
                                    let event = SeatEvent::from((id,event));
                                    new_events.push(Event::Seat(event));
                                }
                                seat.touchs.push(String::from(device.sysname()));
                            }

                        }
                        DeviceEvent::Removed(device_event)=>{
                            let device = device_event.device();
                            if let Some(seat) = self.seats.get_mut(&raw_seat.as_raw()){
                                if let Some(index) = seat.keyboards.iter().position(|name|name == device.sysname()){
                                    seat.keyboards.remove(index);
                                    if seat.keyboards.is_empty(){
                                        let id = seat.id;
                                        let event = SeatEvent::Keyboard(KeyboardEvent::Removed);
                                        let event = SeatEvent::from((id,event));
                                        new_events.push(Event::Seat(event));
                                    }
                                }
                                if let Some(index) = seat.cursors.iter().position(|name|name == device.sysname()){
                                    seat.cursors.remove(index);
                                    if seat.keyboards.is_empty(){
                                        let id = seat.id;
                                        let event = SeatEvent::Cursor(CursorEvent::Removed);
                                        let event = SeatEvent::from((id,event));
                                        new_events.push(Event::Seat(event));
                                    }
                                }
                                if let Some(index) = seat.touchs.iter().position(|name|name == device.sysname()){
                                    seat.touchs.remove(index);
                                    if seat.touchs.is_empty() {
                                        let id = seat.id;
                                        let event = SeatEvent::Touch(TouchEvent::Removed);
                                        let event = SeatEvent::from((id,event));
                                        new_events.push(Event::Seat(event));
                                    }
                                }
                            }
                        }
                    }
                },
                LibinputEvent::Keyboard(keyboard_event) => {
                    if let Some(seat) = self.seats.get(&raw_seat.as_raw()){
                        let code = keyboard_event.key();
                        let keystrokes = self.keystroke_decoder.decode(code);
                        for (keysym, direction) in keystrokes.as_keysyms() {
                            let key = keysym_to_w3c_keycode(keysym);

                            let state = match direction {
                                keystroke_decoder::KeyDirection::Up => State::Up,
                                keystroke_decoder::KeyDirection::Down => State::Down,
                            };
                            let id = seat.id;
                            let event = SeatEvent::Keyboard(KeyboardEvent::Key{code,key,state});
                            let event = SeatEvent::from((id,event));
                            new_events.push(Event::Seat(event));
                        }
                    }
                }
                LibinputEvent::Pointer(pointer_event) => {
                    if let Some(seat) = self.seats.get_mut(&raw_seat.as_raw()){
                        let id = seat.id;
                        match pointer_event {

                            pointer::PointerEvent::Motion(motion_event) => {
                                 match seat.cursor_mode {
                                    CursorMode::Absolute=>{
                                        let x = seat.cursor_position.x + motion_event.dx() as u32;
                                        let y = seat.cursor_position.y + motion_event.dy() as u32;
                                        let position = Position2D{x,y};
                                        seat.cursor_position = position;

                                        let event = SeatEvent::Cursor(CursorEvent::AbsoluteMovement{position});
                                        let event = SeatEvent::from((id,event));
                                        new_events.push(Event::Seat(event));


                                    }
                                    CursorMode::Relative=>{
                                        let offset = Offset2D<f32>{x: motion_event.dx() as f32,y: motion_event.dy() as f32};
                                        let event = SeatEvent::Cursor(CursorEvent::RelativeMovement{offset});
                                        let event = SeatEvent::from((id,event));
                                        new_events.push(Event::Seat(event));
                                    }
                                }
                            }
                            pointer::PointerEvent::Button(button_event) => {
                                let keystrokes = self.keystroke_decoder.decode(button_event.button());
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
                                    let event = SeatEvent::from((id,event));
                                    new_events.push(Event::Seat(event));
                                }
                            }
                            pointer::PointerEvent::Axis(axis_event) => {
                                let event = SeatEvent::Cursor(CursorEvent::Axis {
                                    value: Offset2D<f32>{
                                        x: axis_event.axis_value(Axis::Horizontal) as f32,
                                        y: axis_event.axis_value(Axis::Vertical) as f32
                                    }
                                });
                                let event = SeatEvent::from((id,event));
                                new_events.push(Event::Seat(event));
                            }
                            _ => {}
                        }
                    }
                }
                /*
                LibinputEvent::Touch(_touch_event) => unimplemented!(),
                LibinputEvent::Tablet(_table_tool_event) => unimplemented!(),
                LibinputEvent::TabletPad(_tablet_pad_event) => unimplemented!(),
                LibinputEvent::Gesture(_gesture_event) => unimplemented!(),
                LibinputEvent::Switch(_switch_event) => unimplemented!(),
                */
                _=>{}
            }
        }
        new_events
    }
    fn request(&mut self, requests: Vec<Request>) {
        requests.into_iter().for_each(|request|{
            match request {
                crate::definitions::Request::Seat(_request)=>{

                }
                crate::definitions::Request::Output(_request)=>{

                }
                crate::definitions::Request::Surface(_request)=>{

                }
            }
        });
    }
    /*
    fn set_keyboard_layout(&mut self, _layout: String)->Result<(),KeyboardLayoutError> {Err(KeyboardLayoutError::Unsupported)}
    fn set_key_repeat(&mut self, _seat_id: SeatId, _value: bool)->Result<(),KeyRepeatError> {Err(KeyRepeatError::Unsupported)}
    fn set_cursor_mode(&mut self, _seat_id: SeatId, _mode: CursorMode)->Result<(),CursorModeError> {Err(CursorModeError::Unsupported)}

    fn create_surface(&mut self, _output: Option<OutputId>)->Result<(),SurfaceError> {Err(SurfaceError::Unsupported)}
*/
}

