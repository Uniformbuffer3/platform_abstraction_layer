use std::collections::HashMap;
use crate::definitions::{Event,
    SeatEvent,SeatId,SeatInfo,
    OutputEvent,OutputId,OutputInfo,
    SurfaceEvent,SurfaceId,SurfaceInfo
};

pub struct StateTracker {
    seats: HashMap<SeatId,SeatInfo>,
    outputs: HashMap<OutputId,OutputInfo>,
    surfaces: HashMap<SurfaceId,SurfaceInfo>
}

impl StateTracker {
    pub fn new()->Self {
        let seats = HashMap::new();
        let outputs = HashMap::new();
        let surfaces = HashMap::new();
        Self{seats,outputs,surfaces}
    }
    pub fn update(&mut self,events: &Vec<Event>){
        events.iter().for_each(|event|{
            match event {
                Event::Seat(ref seat_event)=>{
                    match seat_event.event {
                        SeatEvent::Added(ref info)=>{
                            self.seats.insert(seat_event.id,info.clone());
                        }
                        SeatEvent::Removed=>{
                            self.seats.remove(&seat_event.id);
                        }
                        _=>{}
                    }
                }
                Event::Output(ref output_event)=>{
                    match output_event.event {
                        OutputEvent::Added(ref info)=>{
                            self.outputs.insert(output_event.id,info.clone());
                        }
                        OutputEvent::Removed=>{
                            self.outputs.remove(&output_event.id);
                        }
                        _=>{}
                    }
                }
                Event::Surface(ref surface_event)=>{
                    match surface_event.event {
                        SurfaceEvent::Added(ref info)=>{
                            self.surfaces.insert(surface_event.id,info.clone());
                        }
                        SurfaceEvent::Moved(ref position)=>{
                            if let Some(surface) = self.surfaces.get_mut(&surface_event.id){
                                surface.position = *position;
                            }
                        }
                        SurfaceEvent::Resized(ref size)=>{
                            if let Some(surface) = self.surfaces.get_mut(&surface_event.id){
                                surface.size = *size;
                            }
                        }
                        SurfaceEvent::Removed=>{
                            self.surfaces.remove(&surface_event.id);
                        }
                        _=>{}
                    }
                }
            }
        })
    }
}

