mod seat_event;
pub use seat_event::*;

mod surface_event;
pub use surface_event::*;

mod output_event;
pub use output_event::*;

#[derive(Clone, Debug,PartialEq)]
pub enum Event {
    Seat{
        time: u32,
        id: SeatId,
        event: SeatEvent,
    },
    Output{
        time: u32,
        id: OutputId,
        event: OutputEvent
    },
    Surface{
        time: u32,
        id: SurfaceId,
        event: SurfaceEvent,
    },
}
