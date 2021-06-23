mod seat_event;
pub use seat_event::*;

mod surface_event;
pub use surface_event::*;

mod output_event;
pub use output_event::*;

#[derive(Clone, Debug,PartialEq)]
pub enum Event {
    Seat(SeatEvent),
    Output(OutputEvent),
    Surface(SurfaceEvent),
}

impl From<SeatEvent> for Event {
    fn from(event: SeatEvent)->Self {Self::Seat(event)}
}

impl From<OutputEvent> for Event {
    fn from(event: OutputEvent)->Self {Self::Output(event)}
}

impl From<SurfaceEvent> for Event {
    fn from(event: SurfaceEvent)->Self {Self::Surface(event)}
}
