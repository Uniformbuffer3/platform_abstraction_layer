mod conv;
mod handlers;
use handlers::*;

use crate::definitions::*;
use std::collections::HashMap;
use smithay_client_toolkit::{
    environment::Environment,
    output::OutputStatusListener,
    reexports::client::{
        protocol::{wl_seat::WlSeat, wl_surface::WlSurface, wl_output::WlOutput},
        ConnectError, Display, EventQueue,
    },
    seat::SeatListener,
    shm::{AutoMemPool, Format},
    window::FallbackFrame,
};

use keystroke_decoder::KeystrokeDecoder;


smithay_client_toolkit::default_environment!(WaylandEnv, desktop, fields = [], singles = []);


#[derive(Default)]
pub struct EventListeners {
    seat: Option<SeatListener>,
    output: Option<OutputStatusListener>,
}

pub struct DispatchContext<S> {
    pub events: Vec<Event<S>>,
    pub outputs: HashMap<OutputId, WlOutput>,
    pub seats: HashMap<SeatId, WlSeat>,
    pub surfaces: HashMap<SurfaceId, WlSurface>,
    pub keystroke_decoder: KeystrokeDecoder,
}

pub struct WaylandPlatform<S> {
    environment: Environment<WaylandEnv>,
    event_listeners: EventListeners,
    event_queue: EventQueue,

    shm_memory_pool: AutoMemPool,
    dispatch_context: DispatchContext<S>,
    display: Display,
}

impl<S: 'static> WaylandPlatform<S> {
    pub fn new() -> Result<Self, ConnectError> {
        let wayland_environment =
            smithay_client_toolkit::new_default_environment!(WaylandEnv, desktop);
        let (environment, display, event_queue) = match wayland_environment {
            Ok(wayland_environment) => wayland_environment,
            Err(err) => return Err(err),
        };

        let dispatch_context = DispatchContext {
            events: Vec::new(),
            outputs: HashMap::new(),
            seats: HashMap::new(),
            surfaces: HashMap::new(),
            keystroke_decoder: KeystrokeDecoder::new(),
        };

        let event_listeners = EventListeners::default();

        let shm_memory_pool = environment.create_auto_pool().unwrap();

        let mut backend = Self {
            environment,
            display,
            event_queue,
            event_listeners,
            shm_memory_pool,
            dispatch_context,
        };

        backend.init_seat_listener();
        backend.init_output_listener();

        Ok(backend)
    }

    fn init_seat_listener(&mut self) {
        if self.event_listeners.seat.is_none() {
            for seat in self.environment.get_all_seats() {
                let id = seat.as_ref().id().into();
                smithay_client_toolkit::seat::with_seat_data(&seat, |seat_data| {
                    let seat = seat.detach();
                    if !seat_data.defunct {
                        self.dispatch_context.seats.insert(id, seat);

                        let event = SeatEventType::Added((id, seat_data.clone()).into());
                        self.dispatch_context.events.push(Event::Seat { id, event });
                    }
                });
            }
            self.event_listeners.seat = Some(self.environment.listen_for_seats(
                |seat, data, mut dispatch_data| {
                    let dispatch_context = dispatch_data.get::<DispatchContext<S>>().unwrap();
                    let seat = seat.detach();
                    let id = seat.as_ref().id().into();

                    if data.defunct {
                        seat.release();
                        dispatch_context.seats.remove(&id);
                        let event = SeatEventType::Removed;
                        dispatch_context.events.push(Event::Seat { id, event });
                    } else if dispatch_context.seats.contains_key(&id) {
                        let current_data: SeatInfo =
                            smithay_client_toolkit::seat::with_seat_data(&seat, |seat_data| {
                                (id, seat_data.clone()).into()
                            })
                            .unwrap();
                        match (current_data.has_pointer, data.has_pointer) {
                            (false, true) => {
                                let event = SeatEventType::Changed(SeatCapability::PointerAdded);
                                dispatch_context.events.push(Event::Seat { id, event });
                            }
                            (true, false) => {
                                let event = SeatEventType::Changed(SeatCapability::PointerRemoved);
                                dispatch_context.events.push(Event::Seat { id, event });
                            }
                            _ => (),
                        }
                        match (current_data.has_keyboard, data.has_keyboard) {
                            (false, true) => {
                                let keyboard = seat.get_keyboard();
                                handle_keyboard::<S>(id, keyboard);
                                let event = SeatEventType::Changed(SeatCapability::KeyboardAdded);
                                dispatch_context.events.push(Event::Seat { id, event });
                            }
                            (true, false) => {
                                let event = SeatEventType::Changed(SeatCapability::KeyboardRemoved);
                                dispatch_context.events.push(Event::Seat { id, event });
                            }
                            _ => (),
                        }
                        match (current_data.has_touch, data.has_touch) {
                            (false, true) => {
                                let event = SeatEventType::Changed(SeatCapability::TouchAdded);
                                dispatch_context.events.push(Event::Seat { id, event });
                            }
                            (true, false) => {
                                let event = SeatEventType::Changed(SeatCapability::TouchRemoved);
                                dispatch_context.events.push(Event::Seat { id, event });
                            }
                            _ => (),
                        }
                    } else {
                        dispatch_context.seats.insert(id, seat);
                    }
                },
            ));
        }
    }

    fn init_output_listener(&mut self) {
        if self.event_listeners.output.is_none() {
            for output in self.environment.get_all_outputs() {
                smithay_client_toolkit::output::with_output_info(&output, |output_info| {
                    if !output_info.obsolete {
                        unimplemented!();
                        /*
                        let id = output_info.id.into();
                        let event = OutputEventType::Added {};
                        self.dispatch_context.events.push(Event::Output{id,event});
                        */
                    }
                });
            }
            self.event_listeners.output = Some(self.environment.listen_for_outputs(
                |_handle, _data, mut _dispatch_data| {
                    unimplemented!();
                    /*
                    let id = handle.as_ref().id().into();
                    let event = if data.obsolete {
                        handle.release();
                        OutputEventType::Removed{}
                    } else {
                        OutputEventType::Added{}
                    };
                    dispatch_data
                        .get::<super::DispatchContext>()
                        .unwrap()
                        .events
                        .push(Event::Output{id,event});
                    */
                },
            ));
        }
    }
}

impl<S> crate::definitions::InputBackend<S> for WaylandPlatform<S> {
    fn events(&mut self) -> Vec<Event<S>> {
        self.event_queue
            .events(&mut (), |raw_event, anonymus_object, _events_data| {
                panic!(
                    "Unhandled event received:\n{:#?}\n{:#?}",
                    raw_event, anonymus_object
                );
            })
            .expect("An error occurred during event dispatching!");
        self.dispatch_context.events.drain(..).collect()
    }

    fn set_keyboard_layout(&mut self, layout: String) {
        self.dispatch_context.keystroke_decoder.set_layout(layout);
    }
}

impl<S: 'static> crate::definitions::GraphicBackend for WaylandPlatform<S> {
    fn create_surface(
        &mut self,
        output_id: Option<crate::definitions::OutputId>,
    ) {
        let width = 800;
        let height = 600;

        let buffer = self
            .shm_memory_pool
            .try_draw::<_, std::io::Error>(width, height, width * 4, Format::Argb8888, |bytes| {
                Ok(bytes.iter_mut().for_each(|byte| *byte = 255))
            })
            .unwrap();
        let surface = self.environment.create_surface();
        let surface = surface.detach();
        let surface_id = surface.as_ref().id().into();


        let (x, y) = match output_id {
            Some(output_id)=>{
                let output_info = match self.dispatch_context.outputs.get(&output_id){
                    Some(output)=>smithay_client_toolkit::output::with_output_info(&output, |output_info|output_info.clone()),
                    None=>return
                };

                match output_info {
                    Some(output)=>output.location,
                    None=>(0,0)
                }
            }
            None=>(0,0)
        };

        surface.attach(Some(&buffer), x, y);

        let mut window = self
            .environment
            .create_window::<FallbackFrame, _>(
                surface,
                None,
                (800, 600),
                move |event, mut dispatch_data| {
                    match event {
                        smithay_client_toolkit::window::Event::Configure {
                            new_size,
                            states: _,
                        } => {
                            if let Some(size) = new_size {
                                let event = SurfaceEventType::Resized(Size::from((size.0,size.1)));
                                let id = surface_id;
                                dispatch_data
                                    .get::<DispatchContext<S>>()
                                    .unwrap()
                                    .events
                                    .push(Event::Surface { id, event })
                            }
                            /*
                                                states.iter().for_each(|state|{
                                                    match state {
                                                        smithay_client_toolkit::window::State::Maximized,
                                                        smithay_client_toolkit::window::State::Fullscreen,
                                                        smithay_client_toolkit::window::State::Resizing,
                                                        smithay_client_toolkit::window::State::Activated,
                                                        smithay_client_toolkit::window::State::TiledLeft=>,
                                                        smithay_client_toolkit::window::State::TiledRight,
                                                        smithay_client_toolkit::window::State::TiledTop,
                                                        smithay_client_toolkit::window::State::TiledBottom,
                                                    }
                                                    dispatch_data.get::<super::DispatchContext>().unwrap().events.push(SurfaceEventTypeInner::TilingChanged.into())
                                                })

                                                dispatch_data.get::<super::DispatchContext>().unwrap().events.push(SurfaceEventTypeInner::Destroyed.into())
                            */
                        }
                        smithay_client_toolkit::window::Event::Close => {
                            let id = surface_id;
                            let event = SurfaceEventType::Destroyed;
                            dispatch_data
                                .get::<DispatchContext<S>>()
                                .unwrap()
                                .events
                                .push(Event::Surface { id, event })
                        }
                        smithay_client_toolkit::window::Event::Refresh => (),
                    }
                },
            )
            .unwrap();
        window.surface().commit();
        window.refresh();
        unimplemented!();
/*
        let id = output_id;
        let event = OutputEventType::Added{};
        self.dispatch_context.events.push(Event::Output{id,event});
        */

/*
        let surface = super::WaylandSurface { id: surface_id };
        let linux_surface = super::LinuxSurface::from(surface);
        self.dispatch_context
            .surfaces
            .insert(surface_id, window.surface().clone());

        Ok(linux_surface.into())
        */
    }

/*
    fn output_info(&self, output_id: OutputId) -> Option<OutputInfo> {
        for output in self.environment.get_all_outputs() {
            let output_info =
                smithay_client_toolkit::output::with_output_info(&output, |output_info| {
                    if output_id == output_info.id.into() {
                        Some(output_info.clone().into())
                    } else {
                        None
                    }
                })
                .unwrap();
            if output_info.is_some() {
                return output_info;
            }
        }
        return None;
    }
    */
    /*
    fn list_outputs(&self) -> Vec<crate::Output> {
        self.environment
            .get_all_outputs()
            .into_iter()
            .map(|wayland_output| {
                smithay_client_toolkit::output::with_output_info(&wayland_output, |output_info| {

                    output_info.clone().into()
                })
                .unwrap()
            })
            .collect()
    }
    */
}
