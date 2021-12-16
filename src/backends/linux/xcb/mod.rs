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
//use x11rb::cursor::Handle as CursorHandle;
//use x11rb::resource_manager::Database as CursorDatabase;

pub struct XcbPlatform {
    keystroke_decoder: KeystrokeDecoder,
    connection: Arc<XCBConnection>,
    //cursor_database: CursorDatabase,
    //cursor_handle: CursorHandle,
    preferred_screen: usize,
    dummy_window: u32,
    wm_protocols: u32,
    wm_delete_window: u32,
    windows: Vec<u32>,
    pending_events: Vec<crate::definitions::Event>,
    external_contexts: Vec<Box<dyn ExternalContext>>
}
impl XcbPlatform {
    pub fn new(external_contexts: Vec<Box<dyn ExternalContext>>) -> Result<Self, ()> {
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

        //let mut external_contexts = Vec::new();
        //for context in contexts {external_contexts.push(Box::new(&context as &dyn ExternalContext));}

        //let cursor_database = CursorDatabase::new_from_default(connection.as_ref()).unwrap();
        //let cursor_handle = CursorHandle::new(connection.as_ref(),preferred_screen,&cursor_database).unwrap().reply().unwrap();

        let windows = Vec::new();
        let mut platform = Self {
            keystroke_decoder,
            connection,
            //cursor_database,
            //cursor_handle,
            preferred_screen,
            dummy_window,
            wm_protocols,
            wm_delete_window,
            windows,
            pending_events,
            external_contexts,
        };

        platform.init_seats();
        platform.detect_monitors();
        //platform.set_cursor_mode(0u32.into(),CursorMode::Absolute).unwrap();
        //platform.set_key_repeat(0u32.into(),true).unwrap();

        Ok(platform)
    }

    fn init_seats(&mut self){
        let time = 0;
        let id = 0u32.into();
        let name = String::from("seat-0");
        let event = SeatEvent::Added{name};
        self.pending_events.push(crate::definitions::Event::Seat{time,id,event});

        let keyboard_info = KeyboardInfo {
            layout: self.keystroke_decoder.layout().clone(),
            autorepeat: true,
            encoding: KeyEncoding::XkbV1
        };
        let keyboard_event = KeyboardEvent::Added(keyboard_info);
        let event = SeatEvent::Keyboard(keyboard_event);
        self.pending_events.push(crate::definitions::Event::Seat{time,id,event});

        let cursor_info = CursorInfo {
            mode: CursorMode::Absolute,
            theme: CursorImage::Default,
            visible: true
        };
        let cursor_event = CursorEvent::Added(cursor_info);
        let event = SeatEvent::Cursor(cursor_event);
        self.pending_events.push(crate::definitions::Event::Seat{time,id,event});
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

            let time = 0;
            let event = OutputEvent::Added(output_info);
            self.pending_events.push(crate::definitions::Event::Output{time,id,event});
        }
    }
}

#[cfg(target_os = "linux")]
impl std::os::unix::io::AsRawFd for XcbPlatform {
    fn as_raw_fd(&self)->std::os::unix::io::RawFd {
        self.connection.as_raw_fd()
    }
}

impl PlatformBackend for XcbPlatform {
    fn platform_type(&self)->PlatformType {PlatformType::Compositor}
    fn events(&mut self) -> Vec<crate::definitions::Event> {
        let mut events: Vec<crate::definitions::Event> = self.pending_events.drain(..).collect();

        while let Ok(Some(event)) = self.connection.poll_for_event() {
            match event {
                Event::KeyPress(event) => {
                    events.append(&mut handle_keyboard(
                        &mut self.keystroke_decoder,
                        0.into(),
                        event.detail as u32-8,
                        event.sequence as u32,
                        event.time
                    ));
                }
                Event::KeyRelease(event) => {
                    events.append(&mut handle_keyboard(
                        &mut self.keystroke_decoder,
                        0.into(),
                        event.detail as u32-8,
                        event.sequence as u32,
                        event.time
                    ));
                }
                Event::ButtonPress(event) => {
                    let time = event.time;
                    let id = 0u32.into();
                    let state = State::Down;
                    // Key codes taken from https://sources.debian.org/src/xserver-xorg-input-libinput/1.2.0-1/src/xf86libinput.c/#L249-L256
                    let code = match event.detail as u32 {
                        0 => 0,
                        1 => 272,           // Left mouse
                        2 => 273,           // Middle mouse
                        3 => 274,           // Right mouse
                        4 => 275,           // Scroll up
                        5 => 276,           // Scroll down
                        value => {
                            println!("Scroll: {}",value);
                            event.detail as u32 - 8 + 275
                        }, // BTN_SIZE
                    };
                    match code {
                        272 => {
                            let key = Some(crate::definitions::Button::Left);
                            let event = SeatEvent::Cursor(CursorEvent::Button {code,key,state});
                            events.push(crate::definitions::Event::Seat{time,id,event});
                        },
                        273 => {
                            let key = Some(crate::definitions::Button::Middle);
                            let event = SeatEvent::Cursor(CursorEvent::Button {code,key,state});
                            events.push(crate::definitions::Event::Seat{time,id,event});
                        },
                        274 => {
                            let key = Some(crate::definitions::Button::Right);
                            let event = SeatEvent::Cursor(CursorEvent::Button {code,key,state});
                            events.push(crate::definitions::Event::Seat{time,id,event});
                        },
                        /*
                        275 => {
                            let source = AxisSource::Wheel;
                            let direction = AxisDirection::Vertical;
                            let value = AxisValue::Discrete(1);
                            let event = SeatEvent::Cursor(CursorEvent::Axis {source,direction,value});
                            events.push(crate::definitions::Event::Seat{time,id,event});
                        },
                        276 => {
                            let source = AxisSource::Wheel;
                            let direction = AxisDirection::Vertical;
                            let value = AxisValue::Discrete(-1);
                            let event = SeatEvent::Cursor(CursorEvent::Axis {source,direction,value});
                            events.push(crate::definitions::Event::Seat{time,id,event});
                        },
                        */
                        _=> ()
                    }
                }
                Event::ButtonRelease(event) => {
                    let time = event.time;
                    let id = 0u32.into();
                    let state = State::Up;
                    // Key codes taken from https://sources.debian.org/src/xserver-xorg-input-libinput/1.2.0-1/src/xf86libinput.c/#L249-L256
                    let code = match event.detail as u32 {
                        0 => 0,
                        1 => 272,           // Left mouse
                        2 => 273,           // Middle mouse
                        3 => 274,           // Right mouse
                        4 => 275,           // Scroll up
                        5 => 276,           // Scroll down
                        value => {
                            println!("Scroll: {}",value);
                            event.detail as u32 - 8 + 275
                        }, // BTN_SIZE
                    };
                    match code {
                        272 => {
                            let key = Some(crate::definitions::Button::Left);
                            let event = SeatEvent::Cursor(CursorEvent::Button {code,key,state});
                            events.push(crate::definitions::Event::Seat{time,id,event});
                        },
                        273 => {
                            let key = Some(crate::definitions::Button::Middle);
                            let event = SeatEvent::Cursor(CursorEvent::Button {code,key,state});
                            events.push(crate::definitions::Event::Seat{time,id,event});
                        },
                        274 => {
                            let key = Some(crate::definitions::Button::Right);
                            let event = SeatEvent::Cursor(CursorEvent::Button {code,key,state});
                            events.push(crate::definitions::Event::Seat{time,id,event});
                        },
                        275 => {
                            let source = AxisSource::Wheel;
                            let direction = AxisDirection::Vertical;
                            let value = AxisValue::Discrete(-1);
                            let event = SeatEvent::Cursor(CursorEvent::Axis {source,direction,value});
                            events.push(crate::definitions::Event::Seat{time,id,event});
                        },
                        276 => {
                            let source = AxisSource::Wheel;
                            let direction = AxisDirection::Vertical;
                            let value = AxisValue::Discrete(1);
                            let event = SeatEvent::Cursor(CursorEvent::Axis {source,direction,value});
                            events.push(crate::definitions::Event::Seat{time,id,event});
                        },
                        _=> ()
                    }
                }
                Event::EnterNotify(event)=>{
                    let time = event.time;
                    let id = 0.into();
                    let surface_id = SurfaceId::from(event.event);
                    let position = Position2D::from((event.event_x as i32,event.event_y as i32));
                    let event = SeatEvent::Cursor(CursorEvent::Entered {surface_id,position});
                    events.push(crate::definitions::Event::Seat{time,id,event});
                }
                Event::LeaveNotify(event)=>{
                    let time = event.time;
                    let id = 0.into();
                    let surface_id = SurfaceId::from(event.event);
                    let event = SeatEvent::Cursor(CursorEvent::Left {surface_id});
                    events.push(crate::definitions::Event::Seat{time,id,event});
                }
                Event::MotionNotify(event)=>{
                    let time = event.time;
                    let id = 0.into();
                    let position = Position2D::from((event.event_x as i32,event.event_y as i32));
                    let event = SeatEvent::Cursor(CursorEvent::AbsoluteMovement{position});
                    events.push(crate::definitions::Event::Seat{time,id,event});
                }
                Event::ConfigureNotify(event) => {
                    if event.response_type == x11rb::protocol::xproto::CONFIGURE_NOTIFY_EVENT {
                        let time = 0;
                        let id = SurfaceId::from(event.window);
                        let event = SurfaceEvent::Resized(Size2D::from((event.width as u32,event.height as u32)));
                        events.push(crate::definitions::Event::Surface{time,id,event});
                    }
                }
                Event::DestroyNotify(event)=>{
                    let time = 0;
                    let id = SurfaceId::from(event.window);
                    let event = SurfaceEvent::Removed;
                    events.push(crate::definitions::Event::Surface{time,id,event});
                }
                Event::ClientMessage(event) => {
                    let data = event.data.as_data32();
                    if event.format == 32 && data[0] == self.wm_delete_window {
                        self.request(vec![Request::Surface{request: SurfaceRequest::Destroy(event.window.into())}]);
                        //self.connection.destroy_window(event.window).unwrap();
                        //self.connection.flush().unwrap();
                    }
                }
                /*
                Event::XinputChangeDeviceNotify(event)=>{
                    println!("{:#?}",event);
                }

                Event::PropertyNotify(event)=>{
                    println!("{:#?}",&event);
                }
                */
                _ => {}
            }
        }
        events
    }

    fn request(&mut self, requests: Vec<Request>) {
        requests.into_iter().for_each(|request|{
            match request {
                crate::definitions::Request::Seat{request: SeatRequest::Keyboard(keyboard_request)}=>{
                    match keyboard_request {
                        KeyboardRequest::ModifyLayout{layout}=>{
                            self.keystroke_decoder.set_layout(layout);
                        }
                        KeyboardRequest::SetAutoRepeat{rate,delay}=>{
                            let device_spec = 0;
                            let current_controls = x11rb::protocol::xkb::get_controls(self.connection.as_ref(),device_spec)
                            .unwrap()
                            .reply()
                            .unwrap();

                            match x11rb::protocol::xkb::set_controls(
                                self.connection.as_ref(),
                                device_spec,
                                0,//affect_internal_real_mods
                                0,//internal_real_mods
                                0,//affect_ignore_lock_real_mods
                                0,//ignore_lock_real_mods
                                0 as u16,//affect_internal_virtual_mods
                                0 as u16,//internal_virtual_mods
                                0 as u16,//affect_ignore_lock_virtual_mods
                                0 as u16,//ignore_lock_virtual_mods
                                current_controls.mouse_keys_dflt_btn,
                                current_controls.groups_wrap,
                                0 as u16,//access_x_options
                                0 as u32,//affect_enabled_controls
                                current_controls.enabled_controls,
                                0 as u32,//change_controls
                                delay as u16,
                                rate as u16,
                                current_controls.slow_keys_delay,
                                current_controls.debounce_delay,
                                current_controls.mouse_keys_delay,
                                current_controls.mouse_keys_interval,
                                current_controls.mouse_keys_time_to_max,
                                current_controls.mouse_keys_max_speed,
                                current_controls.mouse_keys_curve,
                                current_controls.access_x_timeout,
                                current_controls.access_x_timeout_mask,
                                current_controls.access_x_timeout_values,
                                current_controls.access_x_timeout_options_mask,
                                current_controls.access_x_timeout_options_values,
                                &current_controls.per_key_repeat
                            ){
                                Ok(_)=>{
                                    let time = 0;
                                    let keyboard_event = KeyboardEvent::AutoRepeat{rate,delay};
                                    let event = SeatEvent::Keyboard(keyboard_event);
                                    let id = 0u32.into();
                                    self.pending_events.push(crate::definitions::Event::Seat{time,id,event});
                                }
                                Err(_)=>{println!("Failed to set repeat mode");}
                            }
                        }
                    }

                }
                crate::definitions::Request::Seat{request: SeatRequest::Cursor(cursor_request)}=>{
                    match cursor_request {
                        CursorRequest::ChangeImage(theme)=>{
                            match theme {
                                CursorImage::Custom(_data)=>{
                                    self.windows.iter().cloned().for_each(|window|{
                                        match x11rb::protocol::xfixes::show_cursor(self.connection.as_ref(),window){
                                            Ok(cookie)=>cookie.check().unwrap(),//cookie.ignore_error(),
                                            Err(err)=>eprint!("Error: {}",err)
                                        }
                                    });
                                }
                                CursorImage::Default=>{
                                    self.windows.iter().cloned().for_each(|window|{
                                        match x11rb::protocol::xfixes::show_cursor(self.connection.as_ref(),window){
                                            Ok(cookie)=>cookie.check().unwrap(),
                                            Err(err)=>eprint!("Error: {}",err)
                                        }
                                    });
                                }
                                CursorImage::Hidden=>{
                                    self.windows.iter().cloned().for_each(|window|{
                                        match x11rb::protocol::xfixes::hide_cursor(self.connection.as_ref(),window){
                                            Ok(cookie)=>cookie.check().unwrap(),//cookie.ignore_error(),
                                            Err(err)=>eprint!("Error: {}",err)
                                        }
                                    });
                                }
                            }
                        }
                        _=>{}
                    }
                }
                crate::definitions::Request::Seat{request: SeatRequest::Touch(_touch_request)}=>{
                }
                crate::definitions::Request::Seat{request: SeatRequest::Gamepad(_gamepad_request)}=>{
                }
                crate::definitions::Request::Output{request:_}=>{

                }
                crate::definitions::Request::Surface{request:SurfaceRequest::Create(output)}=>{
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

                    let raw_surface_handle = RawSurfaceHandle::Xcb(xcb_handle);

                    let surface = match self.external_contexts.iter().find_map(|context|context.create_surface(&raw_surface_handle).ok()){
                        Some(surface)=>surface,
                        None=>{
                            unimplemented!();
                        }
                    };

                    self.windows.push(window);

                    let time = 0;
                    let id = window.into();
                    let position = Position2D{x,y};
                    let size = Size2D{width,height};
                    let surface_info = SurfaceInfo{position,size,surface};
                    let event = SurfaceEvent::Added(surface_info);
                    self.pending_events.push(crate::definitions::Event::Surface{time,id,event});
                }
                crate::definitions::Request::Surface{request: SurfaceRequest::Destroy(surface_id)}=>{
                    let id: usize = surface_id.into();
                    let id = id as u32;
                    if let Some(position) = self.windows.iter().position(|window|window == &id){
                        self.connection.destroy_window(id).unwrap();
                        self.connection.flush().unwrap();
                        self.windows.remove(position);
                    }
                }
                crate::definitions::Request::Surface{request: SurfaceRequest::Commit(_surface_id)}=>{
                }
            }
        });
    }
}
