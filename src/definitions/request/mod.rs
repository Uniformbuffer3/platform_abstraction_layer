mod seat_request;
pub use seat_request::*;
mod output_request;
pub use output_request::*;
mod surface_request;
pub use surface_request::*;

pub enum Request {
    Seat(SeatRequest),
    Output(OutputRequest),
    Surface(SurfaceRequest),
}

impl From<SeatRequest> for Request {
    fn from(request: SeatRequest)->Self {Self::Seat(request)}
}
impl From<OutputRequest> for Request {
    fn from(request: OutputRequest)->Self {Self::Output(request)}
}
impl From<SurfaceRequest> for Request {
    fn from(request: SurfaceRequest)->Self {Self::Surface(request)}
}
