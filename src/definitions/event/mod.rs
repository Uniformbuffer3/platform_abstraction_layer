mod seat_event;
pub use seat_event::*;

mod surface_event;
pub use surface_event::*;

mod output_event;
pub use output_event::*;

#[derive(Debug,PartialEq)]
pub enum Event {
    Seat {
        id: crate::definitions::SeatId,
        event: SeatEvent,
    },
    Surface {
        id: crate::definitions::SurfaceId,
        event: SurfaceEvent,
    },
    Output{
        id: crate::definitions::OutputId,
        event: OutputEvent
    }
}
/*
impl From<SeatEvent> for Event {
    fn from(event: SeatEvent) -> Self {
        Self::Seat(event)
    }
}
*/
/*
impl From<SurfaceEvent> for Event {
    fn from(event: SurfaceEvent) -> Self {
        Self::Surface(event)
    }
}

impl From<OutputEvent> for Event {
    fn from(event: OutputEvent) -> Self {
        Self::Output(event)
    }
}*/
