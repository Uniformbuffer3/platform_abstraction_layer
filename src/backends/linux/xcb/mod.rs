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
    pub fn new(_contexts: Vec<crate::definitions::ExternalContext>) -> Result<Self, ()> {
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

        let mut pending_events = Vec::new();

        let resources = x11rb::protocol::randr::get_screen_resources(connection.as_ref(), dummy_window).unwrap().reply().unwrap();
        let monitors = x11rb::protocol::randr::get_monitors(connection.as_ref(), dummy_window,false).unwrap().reply().unwrap().monitors;

        for (id,monitor) in monitors.iter().enumerate() {
            let output = monitor.outputs[0];

            let output_info = x11rb::protocol::randr::get_output_info(connection.as_ref(),output,0).unwrap().reply().unwrap();
            if output_info.connection != x11rb::protocol::randr::Connection::CONNECTED {continue;}

            let x = monitor.x as u32;
            let y = monitor.y as u32;
            let physical_width = output_info.mm_width;
            let physical_height = output_info.mm_height;
            let mut available_modes: Vec<Mode> = output_info.modes.iter().filter_map(|mode_id|{
                match resources.modes.iter().find(|mode|&mode.id == mode_id){
                    Some(mode)=>Some(Mode{
                        width: mode.width as u32,
                        height: mode.height as u32,
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
            let event = OutputEvent::Added {
                x,
                y,
                selected_mode,
                available_modes,
                physical_width,
                physical_height,
                subpixel
            };

            pending_events.push(crate::definitions::Event::Output{id,event});
        }


        Ok(Self {
            keystroke_decoder,
            connection,
            preferred_screen,
            dummy_window,
            wm_protocols,
            wm_delete_window,
            pending_events
        })
    }
}

impl GraphicBackend for XcbPlatform {
    fn create_surface(
        &mut self,
        output: Option<crate::definitions::OutputId>,
    ) {
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
            .background_pixel(screen.white_pixel);

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


        let id = window.into();
        let event = SurfaceEvent::Added{x,y,width,height};

        self.pending_events.push(crate::definitions::Event::Surface{id,event});
    }
    fn raw_surface_handle(&self, surface: SurfaceId) -> RawSurfaceHandle {
        let xcb_handle = raw_window_handle::unix::XcbHandle {
            window: surface.id(),
            connection: self.connection.get_raw_xcb_connection(),
            ..raw_window_handle::unix::XcbHandle::empty()
        };
        RawSurfaceHandle::Xcb(xcb_handle)
    }
}

impl InputBackend for XcbPlatform {
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
                Event::ButtonPress(_event) => {
                    /*
                    let id = 0.into();
                    let key;
                    let state;
                    let position;

                    let event = SeatEvent::Cursor(CursorEvent::CursorButton {key,state,position});
                    println!("{:#?}",event);
                    */
                }
                Event::EnterNotify(event)=>{
                    let id = 0.into();
                    let surface_id = SurfaceId::from(event.event);
                    let position = (event.event_x as f64,event.event_y as f64);
                    let event = SeatEvent::Cursor(CursorEvent::CursorEntered {surface_id,position});
                    events.push(crate::definitions::Event::Seat{id,event});
                }
                Event::LeaveNotify(event)=>{
                    let id = 0.into();
                    let surface_id = SurfaceId::from(event.event);
                    let event = SeatEvent::Cursor(CursorEvent::CursorLeft {surface_id});
                    events.push(crate::definitions::Event::Seat{id,event});
                }
                Event::ConfigureNotify(event) => {
                    let id = SurfaceId::from(event.window);
                     match event.response_type {
                        22 => { //TODO Find equivalent constant
                            let event = SurfaceEvent::Resized{width: event.width as u32, height: event.height as u32};
                            events.push(crate::definitions::Event::Surface{id,event});
                        }
                        150 => { //TODO Find equivalent constant
                            let event = SurfaceEvent::Moved{x: event.x as u32,y: event.y as u32};
                            events.push(crate::definitions::Event::Surface{id,event});
                        }
                        _=>{}
                    }
                }
                Event::DestroyNotify(event)=>{
                    let id = SurfaceId::from(event.window);
                    let event = SurfaceEvent::Destroyed;
                    events.push(crate::definitions::Event::Surface{id,event});
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
    fn keyboard_layout(&self) -> String {
        self.keystroke_decoder.layout().clone()
    }
    fn set_keyboard_layout(&mut self, layout: String) {
        self.keystroke_decoder.set_layout(layout);
    }
}
