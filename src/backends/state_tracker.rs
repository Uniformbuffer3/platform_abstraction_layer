use std::collections::HashMap;
use crate::definitions::{Event,
    SeatEventType,SeatId,SeatInfo,
    OutputEventType,OutputId,OutputInfo,
    SurfaceEventType,SurfaceId,SurfaceInfo
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
                    match seat_event.event_type {
                        SeatEventType::Added(ref info)=>{
                            self.seats.insert(seat_event.id,info.clone());
                        }
                        SeatEventType::Removed=>{
                            self.seats.remove(&seat_event.id);
                        }
                        _=>{}
                    }
                }
                Event::Output(ref output_event)=>{
                    match output_event.event_type {
                        OutputEventType::Added(ref info)=>{
                            self.outputs.insert(output_event.id,info.clone());
                        }
                        OutputEventType::Removed=>{
                            self.outputs.remove(&output_event.id);
                        }
                        _=>{}
                    }
                }
                Event::Surface(ref surface_event)=>{
                    match surface_event.event_type {
                        SurfaceEventType::Added(ref info)=>{
                            self.surfaces.insert(surface_event.id,info.clone());
                        }
                        SurfaceEventType::Moved(ref position)=>{
                            if let Some(surface) = self.surfaces.get_mut(&surface_event.id){
                                surface.position = *position;
                            }
                        }
                        SurfaceEventType::Resized(ref size)=>{
                            if let Some(surface) = self.surfaces.get_mut(&surface_event.id){
                                surface.size = *size;
                            }
                        }
                        SurfaceEventType::Removed=>{
                            self.surfaces.remove(&surface_event.id);
                        }
                        _=>{}
                    }
                }
            }
        })
    }
}

