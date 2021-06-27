use crate::definitions::*;

/*
pub struct VirtualPlatform<SeatB: SeatBackend,OutputB: OutputBackend, SurfaceB: SurfaceBackend> {
    seat_backend: SeatB,
    output_backend: OutputB,
    surface_backend: SurfaceB,
}
impl<SeatB: SeatBackend,OutputB: OutputBackend, SurfaceB: SurfaceBackend> VirtualPlatform<SeatB,OutputB,SurfaceB> {
    pub fn new(seat_backend: SeatB,output_backend: OutputB,surface_backend: SurfaceB)->Self {
        Self {seat_backend,output_backend,surface_backend}
    }
}

impl<SeatB: SeatBackend,OutputB: OutputBackend, SurfaceB: SurfaceBackend> SeatBackend for VirtualPlatform<SeatB,OutputB,SurfaceB> {
    fn dispatch_seats(&mut self) -> Vec<SeatEvent> {self.seat_backend.dispatch_seats()}
    fn set_keyboard_layout(&mut self, layout: String) {self.seat_backend.set_keyboard_layout(layout)}
    fn set_cursor_mode(&mut self, seat_id: SeatId, mode: CursorMode){

    }
    fn set_key_repeat(&mut self, seat_id: SeatId, value: bool){

    }
}*/
