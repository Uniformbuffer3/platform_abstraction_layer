use std::collections::HashMap;
use crate::definitions::{Event,
    //SeatEvent,SeatId,SeatInfo,
    //OutputEvent,OutputId,OutputInfo,
    SurfaceEvent//,SurfaceId,SurfaceInfo
};

pub struct PostProcessing;
impl PostProcessing {
    pub fn new()->Self {
        Self {}
    }

    pub fn process(&self,events: Vec<Event>)->Vec<Event> {
        let mut surface_moved = HashMap::new();
        let mut surface_resized = HashMap::new();

        events.into_iter().filter(|event|{
            match &event {
                Event::Surface(event)=>{
                    match event.event {
                        SurfaceEvent::Moved{..}=>{
                            if !surface_moved.contains_key(&event.id) {surface_moved.insert(event.id,());true}
                            else {false}
                        },
                        SurfaceEvent::Resized{..}=>{
                            if !surface_resized.contains_key(&event.id) {surface_resized.insert(event.id,());true}
                            else {false}
                        }
                        _=>{true}
                    }
                }
                _=>{true}
            }
        }).collect()
    }
}
