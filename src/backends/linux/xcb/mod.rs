mod handlers;
use handlers::*;

use crate::definitions::ExternalContext;

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

        let mut pending_events = Vec::new();

        let resources = x11rb::protocol::randr::get_screen_resources(connection.as_ref(), dummy_window).unwrap().reply().unwrap();
        let monitors = x11rb::protocol::randr::get_monitors(connection.as_ref(), dummy_window,false).unwrap().reply().unwrap().monitors;

        for (id,monitor) in monitors.iter().enumerate() {
            let output = monitor.outputs[0];

            let output_info = x11rb::protocol::randr::get_output_info(connection.as_ref(),output,0).unwrap().reply().unwrap();
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
            pending_events.push(crate::definitions::Event::Output(event));
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

impl SeatBackend for XcbPlatform {
    fn set_keyboard_layout(&mut self, layout: String) {
        self.keystroke_decoder.set_layout(layout);
    }
}

impl OutputBackend for XcbPlatform {}


impl SurfaceBackend for XcbPlatform {
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
}


impl PlatformBackend for XcbPlatform {
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

                    let event = SeatEventType::Cursor(CursorEvent::CursorButton {key,state,position});
                    println!("{:#?}",event);
                    */
                }
                Event::EnterNotify(event)=>{
                    let id = 0.into();
                    let surface_id = SurfaceId::from(event.event);
                    let position = (event.event_x as f64,event.event_y as f64);
                    let event_type = SeatEventType::Cursor(CursorEvent::CursorEntered {surface_id,position});
                    let event = SeatEvent::from((id,event_type));
                    events.push(crate::definitions::Event::Seat(event));
                }
                Event::LeaveNotify(event)=>{
                    let id = 0.into();
                    let surface_id = SurfaceId::from(event.event);
                    let event_type = SeatEventType::Cursor(CursorEvent::CursorLeft {surface_id});
                    let event = SeatEvent::from((id,event_type));
                    events.push(crate::definitions::Event::Seat(event));
                }
                Event::ConfigureNotify(event) => {
                    let id = SurfaceId::from(event.window);
                     match event.response_type {
                        22 => { //TODO Find equivalent constant
                            let event_type = SurfaceEventType::Resized(Size::from((event.width as u32,event.height as u32)));
                            let event = SurfaceEvent::from((id,event_type));
                            events.push(crate::definitions::Event::Surface(event));
                        }
                        150 => { //TODO Find equivalent constant
                            let event_type = SurfaceEventType::Moved(Position::from((event.x as u32,event.y as u32)));
                            let event = SurfaceEvent::from((id,event_type));
                            events.push(crate::definitions::Event::Surface(event));
                        }
                        _=>{}
                    }
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
}
