mod handlers;
use handlers::*;

use std::sync::Arc;

use crate::definitions::*;
use keystroke_decoder::KeystrokeDecoder;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::xcb_ffi::XCBConnection;
use x11rb::wrapper::ConnectionExt as WrapperConnectionExt;
use x11rb::protocol::xproto::ConnectionExt as XProtoConnectionExt;

use raw_window_handle::RawWindowHandle;

pub struct XcbPlatform {
    keystroke_decoder: KeystrokeDecoder,
    connection: Arc<XCBConnection>,
    preferred_screen: usize,
    dummy_window: u32,
    wm_protocols: u32,
    wm_delete_window: u32,
    pending_events: Vec<crate::definitions::Event>,
}
impl XcbPlatform {
    pub fn new() -> Result<Self, ()> {
        let (connection, preferred_screen) = XCBConnection::connect(None).unwrap();
        let connection = Arc::new(connection);
        let wm_protocols = connection.intern_atom(false, b"WM_PROTOCOLS").unwrap().reply().unwrap().atom;
        let wm_delete_window = connection.intern_atom(false, b"WM_DELETE_WINDOW").unwrap().reply().unwrap().atom;

        let keystroke_decoder = KeystrokeDecoder::new();

        let setup = connection.setup();
        let screen = &setup.roots[preferred_screen as usize];
        let dummy_window = connection.generate_id().unwrap();

        connection
            .create_window(
                0,
                dummy_window,
                screen.root,
                0,
                0,
                1,
                1,
                0,
                WindowClass::INPUT_OUTPUT,
                0,
                &CreateWindowAux::new(),
            )
            .unwrap();

        let pending_events = Vec::new();
        let mut platform = Self {
            keystroke_decoder,
            connection,
            preferred_screen,
            dummy_window,
            wm_protocols,
            wm_delete_window,
            pending_events
        };

        platform.detect_monitors();
        //platform.set_cursor_mode(0u32.into(),CursorMode::Absolute).unwrap();
        //platform.set_key_repeat(0u32.into(),true).unwrap();

        Ok(platform)
    }

    fn detect_monitors(&mut self){
        let resources = x11rb::protocol::randr::get_screen_resources(self.connection.as_ref(), self.dummy_window).unwrap().reply().unwrap();
        let monitors = x11rb::protocol::randr::get_monitors(self.connection.as_ref(), self.dummy_window,false).unwrap().reply().unwrap().monitors;

        for (id,monitor) in monitors.iter().enumerate() {
            let output = monitor.outputs[0];

            let output_info = x11rb::protocol::randr::get_output_info(self.connection.as_ref(),output,0).unwrap().reply().unwrap();
            if output_info.connection != x11rb::protocol::randr::Connection::CONNECTED {continue;}

            let position = (monitor.x as u32,monitor.y as u32).into();
            let physical_size = (output_info.mm_width,output_info.mm_height).into();
            let mut available_modes: Vec<Mode> = output_info.modes.iter().filter_map(|mode_id|{
                match resources.modes.iter().find(|mode|&mode.id == mode_id){
                    Some(mode)=>Some(Mode{
                        resolution: (mode.width as u32,mode.height as u32).into(),
                        refresh_rate: ((mode.dot_clock as f32/(mode.htotal as f32 * mode.vtotal as f32))*1000.0) as u32,
                        is_preferred: false
                    }),
                    None=>None
                }
            }).collect();

            if let Some(ref mut mode) = available_modes.iter_mut().next(){
                mode.is_preferred = true;
            }

            let selected_mode = available_modes[0].clone();

            let subpixel = match output_info.subpixel_order {
                x11rb::protocol::render::SubPixel::HORIZONTAL_RGB => Subpixel::HorizontalRgb,
                x11rb::protocol::render::SubPixel::HORIZONTAL_BGR => Subpixel::HorizontalBgr,
                x11rb::protocol::render::SubPixel::VERTICAL_RGB => Subpixel::VerticalRgb,
                x11rb::protocol::render::SubPixel::VERTICAL_BGR => Subpixel::VerticalBgr,
                x11rb::protocol::render::SubPixel::NONE => Subpixel::None,
                _ => Subpixel::Unknown,
            };

            let id = (id as u32).into();
            let output_info = OutputInfo {
                position,
                selected_mode,
                available_modes,
                physical_size,
                subpixel
            };

            let event_type = OutputEventType::Added(output_info);
            let event = OutputEvent::from((id,event_type));
            self.pending_events.push(crate::definitions::Event::Output(event));
        }
    }
}


impl PlatformBackend for XcbPlatform {
    fn platform_type(&self)->PlatformType {PlatformType::Compositor}
    fn dispatch(&mut self) -> Vec<crate::definitions::Event> {
        let mut events: Vec<crate::definitions::Event> = self.pending_events.drain(..).collect();

        while let Ok(Some(event)) = self.connection.poll_for_event() {
            match event {
                Event::KeyPress(event) => {
                    events.append(&mut handle_keyboard(
                        &mut self.keystroke_decoder,
                        0.into(),
                        event.detail as u32,
                    ));
                }
                Event::KeyRelease(event) => {
                    events.append(&mut handle_keyboard(
                        &mut self.keystroke_decoder,
                        0.into(),
                        event.detail as u32,
                    ));
                }
                Event::ButtonPress(event) => {
                    let id = 0u32.into();
                    //let surface_id = SurfaceId::from(event.event);
                    //let position = Position::from((event.event_x as u32,event.event_y as u32));
                    let key = match event.detail {
                        1 => crate::definitions::Button::Left,
                        2 => crate::definitions::Button::Middle,
                        3 => crate::definitions::Button::Right,
                        _=> continue
                    };
                    let state = State::Down;
                    let event_type = SeatEventType::Cursor(CursorEvent::Button {key,state});
                    let event = SeatEvent::from((id,event_type));
                    events.push(crate::definitions::Event::Seat(event));
                }
                Event::ButtonRelease(event) => {
                    let id = 0u32.into();
                    //let surface_id = SurfaceId::from(event.event);
                    //let position = Position::from((event.event_x as u32,event.event_y as u32));
                    let key = match event.detail {
                        1 => crate::definitions::Button::Left,
                        2 => crate::definitions::Button::Middle,
                        3 => crate::definitions::Button::Right,
                        _=> continue
                    };
                    let state = State::Up;
                    let event_type = SeatEventType::Cursor(CursorEvent::Button {key,state});
                    let event = SeatEvent::from((id,event_type));
                    events.push(crate::definitions::Event::Seat(event));
                }
                Event::EnterNotify(event)=>{
                    let id = 0.into();
                    let surface_id = SurfaceId::from(event.event);
                    let position = Position::from((event.event_x as u32,event.event_y as u32));
                    let event_type = SeatEventType::Cursor(CursorEvent::Entered {surface_id,position});
                    let event = SeatEvent::from((id,event_type));
                    events.push(crate::definitions::Event::Seat(event));
                }
                Event::LeaveNotify(event)=>{
                    let id = 0.into();
                    let surface_id = SurfaceId::from(event.event);
                    let event_type = SeatEventType::Cursor(CursorEvent::Left {surface_id});
                    let event = SeatEvent::from((id,event_type));
                    events.push(crate::definitions::Event::Seat(event));
                }
                Event::MotionNotify(event)=>{
                    let id = 0.into();
                    //let surface_id = SurfaceId::from(event.event);
                    let position = Position::from((event.event_x as u32,event.event_y as u32));
                    let event_type = SeatEventType::Cursor(CursorEvent::AbsoluteMovement{position});
                    let event = SeatEvent::from((id,event_type));
                    events.push(crate::definitions::Event::Seat(event));
                }
                Event::ConfigureNotify(event) => {
                    let id = SurfaceId::from(event.window);
                    let event_type = match event.response_type {
                        x11rb::protocol::xproto::CONFIGURE_NOTIFY_EVENT => SurfaceEventType::Resized(Size::from((event.width as u32,event.height as u32))),
                        150 => SurfaceEventType::Moved(Position::from((event.x as u32,event.y as u32))),
                        _=>continue
                    };
                    let event = SurfaceEvent::from((id,event_type));
                    events.push(crate::definitions::Event::Surface(event));
                }
                Event::DestroyNotify(event)=>{
                    let id = SurfaceId::from(event.window);
                    let event_type = SurfaceEventType::Removed;
                    let event = SurfaceEvent::from((id,event_type));
                    events.push(crate::definitions::Event::Surface(event));
                }
                Event::ClientMessage(event) => {
                    let data = event.data.as_data32();
                    if event.format == 32 && data[0] == self.wm_delete_window {
                        self.connection.destroy_window(event.window).unwrap();
                        self.connection.flush().unwrap();
                    }
                }
                _ => {}
            }
        }
        events
    }

    fn request(&mut self, requests: Vec<Request>) {
        requests.into_iter().for_each(|request|{
            match request {
                crate::definitions::Request::Seat(request)=>{
                    let _id = request.id;
                    match request.event_type {
                        SeatRequestType::Keyboard(keyboard_request)=>{
                            match keyboard_request {
                                KeyboardRequest::ModifyLayout{layout}=>{
                                    self.keystroke_decoder.set_layout(layout);
                                }
                                KeyboardRequest::SetAutoRepeat(value)=>{
                                    let repeat_mode = match value {
                                        true=>AutoRepeatMode::ON,
                                        false=>AutoRepeatMode::OFF,
                                    };
                                    let parameters = ChangeKeyboardControlAux::new().auto_repeat_mode(repeat_mode);
                                    match x11rb::protocol::xproto::change_keyboard_control(self.connection.as_ref(),&parameters){
                                        Ok(_)=>{
                                            let keyboard_event = KeyboardEvent::AutoRepeat(value);
                                            let event_type = SeatEventType::Keyboard(keyboard_event);
                                            let id = 0u32.into();
                                            let event = SeatEvent::from((id,event_type));
                                            self.pending_events.push(crate::definitions::Event::Seat(event));
                                        }
                                        Err(_)=>{println!("Failed to set repeat mode");}
                                    }
                                }
                            }
                        }
                        SeatRequestType::Cursor(_cursor_request)=>{
                        }
                        SeatRequestType::Touch(_touch_request)=>{
                        }
                        SeatRequestType::Gamepad(_gamepad_request)=>{
                        }
                    }

                }
                crate::definitions::Request::Output(_request)=>{

                }
                crate::definitions::Request::Surface(request)=>{
                    match request {
                        SurfaceRequest::Create(output)=>{
                            let setup = self.connection.setup();
                            let screen = &setup.roots[0];

                            let monitors = x11rb::protocol::randr::get_monitors(self.connection.as_ref(), self.dummy_window,false).unwrap().reply().unwrap().monitors;

                            let window = self.connection.generate_id().unwrap();
                            let win_aux = CreateWindowAux::new()
                                .event_mask(
                                    EventMask::EXPOSURE
                                        | EventMask::STRUCTURE_NOTIFY
                                        | EventMask::NO_EVENT
                                        | EventMask::KEY_PRESS
                                        | EventMask::KEY_RELEASE
                                        | EventMask::BUTTON_PRESS
                                        | EventMask::BUTTON_RELEASE
                                        | EventMask::ENTER_WINDOW
                                        | EventMask::LEAVE_WINDOW
                                        | EventMask::PROPERTY_CHANGE
                                        | EventMask::POINTER_MOTION,
                                )
                                .background_pixel(screen.black_pixel);

                            let (x,y) = match output {
                                Some(output)=>{
                                    let output_index: usize = output.clone().into();
                                    let monitor = &monitors[output_index];
                                    (monitor.x as u32,monitor.y as u32)
                                }
                                None=>(0,0)
                            };

                            let width = 400u32;
                            let height = 400u32;

                            self.connection
                                .create_window(
                                    screen.root_depth,
                                    window,
                                    screen.root,
                                    x as i16,y as i16,
                                    width as u16,height as u16,
                                    0,
                                    WindowClass::INPUT_OUTPUT,
                                    0,
                                    &win_aux,
                                )
                                .unwrap();

                            self.connection.change_property32(
                                PropMode::APPEND,
                                window,
                                self.wm_protocols,
                                AtomEnum::ATOM,
                                &[self.wm_delete_window],
                            ).unwrap();

                            self.connection.map_window(window).unwrap();
                            self.connection.flush().unwrap();

                            let xcb_handle = raw_window_handle::unix::XcbHandle {
                                window: window.clone().into(),
                                connection: self.connection.get_raw_xcb_connection(),
                                ..raw_window_handle::unix::XcbHandle::empty()
                            };
                            let raw_window_handle = RawWindowHandle::Xcb(xcb_handle);

                            let surface = Surface::Raw(raw_window_handle);

                            let id = window.into();
                            let position = Position{x,y};
                            let size = Size{width,height};
                            let surface_info = SurfaceInfo{position,size,surface};
                            let event_type = SurfaceEventType::Added(surface_info);
                            let event = SurfaceEvent::from((id,event_type));
                            self.pending_events.push(crate::definitions::Event::Surface(event));
                        }
                        SurfaceRequest::Destroy(_surface_id)=>{

                        }
                        SurfaceRequest::Commit(_surface_id)=>{
                        }
                    }
                }
            }
        });
    }
}
