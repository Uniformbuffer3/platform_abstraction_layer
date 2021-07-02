use std::collections::HashMap;
use crate::definitions::*;
/*
use parry2d::{
    shape::{Cuboid,Segment,Compound,SharedShape},
    math::{Real,Vector,Point,Isometry},
    query::{Ray,RayCast}
};
*/
struct Surface {
    position: Position,
    size: Size
}
impl Surface {
    pub fn contains(&self,position: Position)->bool{
        position.x > self.position.x && position.x < self.position.x + self.size.width &&
        position.y > self.position.y && position.y < self.position.y + self.size.height
    }
}

pub struct SurfaceManager {
    surfaces: HashMap<SurfaceId,Surface>,
    surface_stack: Vec<SurfaceId>
}
impl SurfaceManager {
    pub fn new()->Self {
        let surfaces = HashMap::new();
        let surface_stack = Vec::new();
        Self {surfaces,surface_stack}
    }
    pub fn add_surface(&mut self, surface_id: SurfaceId,position: Position, size: Size){
        let surface = Surface {position,size};
        self.surfaces.insert(surface_id,surface);
        self.surface_stack.push(surface_id);
    }
    pub fn del_surface(&mut self, surface_id: SurfaceId){
        self.surfaces.remove(&surface_id);
        if let Some(index) = self.surface_stack.iter().position(|id|id == &surface_id){
            self.surface_stack.remove(index);
        }
    }

    pub fn put_on_top(&mut self,surface_id: SurfaceId){
        if let Some(index) = self.surface_stack.iter().position(|id|id == &surface_id){
            let surface_id = self.surface_stack.remove(index);
            self.surface_stack.push(surface_id);
        }
    }

    pub fn cursor_movement(&self, id: SeatId, old_position: Position, new_position: Position)->Vec<Event> {
        let mut events = Vec::new();
        let surface_old = self.surface(old_position);
        let surface_new = self.surface(old_position);

        match (surface_old,surface_new){
            (Some(id1),Some(id2))=>{
                if id1 != id2 {
                    let surface_id = id1;
                    let event_type = SeatEventType::Cursor(CursorEvent::Left{surface_id});
                    let event = SeatEvent::from((id,event_type));
                    events.push(Event::Seat(event));

/*
                    let old_position = Point::new(old_position.x as f32,old_position.y as f32);
                    let new_position = Point::new(new_position.x as f32, new_position.y as f32);
                    let segment = Segment::new(old_position,new_position);

                    let ray = Ray::new(old_position,*segment.direction().unwrap());
                    let result = self.shape.cast_ray_and_get_normal(
                        &Isometry::translation(0.0,0.0),
                        &ray,
                        1000000.0,
                        false
                    ).unwrap();
                    let point = ray.point_at(result.toi);

                    //Position{x: point.coords.x as u32,y: point.coords.y as u32}
*/


                    let surface_id = id2;
                    let position = new_position;
                    let event_type = SeatEventType::Cursor(CursorEvent::Entered{surface_id,position});
                    let event = SeatEvent::from((id,event_type));
                    events.push(Event::Seat(event));
                }

                let position = new_position;
                let event_type = SeatEventType::Cursor(CursorEvent::AbsoluteMovement{position});
                let event = SeatEvent::from((id,event_type));
                events.push(Event::Seat(event));
            }
            _=>{}
        }

        events
    }

    pub fn surface(&self,position: Position)->Option<SurfaceId> {
        for surface_id in &self.surface_stack {
            if let Some(surface) = self.surfaces.get(surface_id){
                if surface.contains(position){return Some(*surface_id);}
            }
        }
        None
    }

}
