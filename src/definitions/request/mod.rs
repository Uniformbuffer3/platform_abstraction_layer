mod seat_request;
pub use seat_request::*;
mod output_request;
pub use output_request::*;
mod surface_request;
pub use surface_request::*;

//use crate::definitions::{SeatId,OutputId,SurfaceId};
/// Possible requests.
pub enum Request {
    Seat{
        request: SeatRequest
    },
    Output{
        request: OutputRequest
    },
    Surface{
        request: SurfaceRequest
    },
}

/*
impl From<SeatRequest> for Request {
    fn from(request: SeatRequest)->Self {Self::Seat(request)}
}
impl From<OutputRequest> for Request {
    fn from(request: OutputRequest)->Self {Self::Output(request)}
}
impl From<SurfaceRequest> for Request {
    fn from(request: SurfaceRequest)->Self {Self::Surface(request)}
}
*/
