use crate::definitions::*;
use std::cmp::Ordering;
use std::collections::VecDeque;

/*
use parry2d::{
    shape::{Cuboid,Segment,Compound,SharedShape},
    math::{Real,Vector,Point,Isometry},
    query::{Ray,RayCast}
};
*/

pub enum SurfaceKind {

}

pub enum SurfaceUpdate {
    PositionChanged{id: usize,position: Position3D<u32>}
}


#[derive(Debug)]
pub struct Surface {
    pub id: usize,
    pub position: Position3D<u32>,
    pub size: Size2D<u32>,
}
impl Surface {
    pub fn contains(&self,position: Position2D<u32>)->bool{
        position.x > self.position.x && position.x < self.position.x + self.size.width &&
        position.y > self.position.y && position.y < self.position.y + self.size.height
    }
}
impl Ord for Surface {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.z.cmp(&other.position.z)
    }
}
impl PartialOrd for Surface {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.position.z.cmp(&other.position.z))
    }
}

impl PartialEq for Surface {
    fn eq(&self, other: &Self) -> bool {
        self.position.z == other.position.z
    }
}
impl Eq for Surface {}

#[derive(Debug)]
pub struct SurfaceManager{
    surfaces: VecDeque<Surface>
}
impl SurfaceManager {
    pub fn new()->Self {
        let surfaces = VecDeque::new();
        Self{surfaces}
    }
    pub fn add_surface(&mut self, id: usize, size: Size2D<u32>)->Vec<SurfaceUpdate>{
        self.add_surface_inner(id, Position2D::from((0,0)), size)
    }
    pub fn add_surface_inner(&mut self, id: usize, position: Position2D<u32>, size: Size2D<u32>)->Vec<SurfaceUpdate>{
        let position = Position3D::from((position,0));
        self.surfaces.push_front(Surface {id,position,size});
        self.update_depth(1..)
    }
    pub fn del_surface(&mut self,id: usize)->Vec<SurfaceUpdate>{
        if let Some(position) = self.surfaces.iter().position(|surface|surface.id == id){
            self.surfaces.remove(position);
            self.update_depth(position..)
        }
        else{Vec::new()}
    }

    pub fn surface_ref<T>(&self,id: usize, callback: impl Fn(&Surface)->T)->Option<T> {
        self.surfaces.iter().find_map(|surface|{
            if surface.id == id {Some(callback(surface))}
            else{None}
        })
    }
    pub fn surface_mut<T>(&mut self,id: usize, callback: impl Fn(&mut Surface)->T)->Option<T> {
        self.surfaces.iter_mut().find_map(|surface|{
            if surface.id == id {Some(callback(surface))}
            else{None}
        })
    }

    fn update_depth<'a>(&'a mut self,range: impl std::ops::RangeBounds<usize>+Iterator<Item=usize> )->Vec<SurfaceUpdate> {
        range.map(|index|{
            self.surfaces[index].position.z = index as u32;
            SurfaceUpdate::PositionChanged{
                id: self.surfaces[index].id,
                position: self.surfaces[index].position
            }
        }).collect()
    }
}



/*
pub struct GeometryManager {
    outputs: HashMap<usize,Rectangle<u32>>,


    //size: Size2D<u32>,
    //window_space: Rectangle<u32>
}
impl GeometryManager {
    pub fn new()->Self {
        let outputs = HashMap::new();
        let surfaces = HashMap::new();
        let surface_stack = BTreeSet::new();
        //let window_space = Rectangle {};
        Self {outputs,surfaces,surface_stack}
    }


    pub fn add_surface(&mut self, id: usize, size: Size2D<u32>, reserve_space: bool){
        let surface = Surface {position,size,reserve_space};

        if reserve_space {
            if position.y + size.height > self.window_space.position.y {
            }
        }

        self.surfaces.insert(id,surface);
        self.surface_stack.push(id);
    }
    pub fn del_surface(&mut self, id: usize){
        self.surfaces.remove(&id);
        if let Some(index) = self.surface_stack.iter().position(|current_id|current_id == &id){
            self.surface_stack.remove(index);
        }
    }

    pub fn put_on_top(&mut self,id: usize){
        if let Some(index) = self.surface_stack.iter().position(|current_id|current_id == &id){
            let id = self.surface_stack.remove(index);
            self.surface_stack.push(id);
        }
    }

    pub fn cursor_movement(&self, id: SeatId, old_position: Position2D<u32>, new_position: Position2D<u32>)->Vec<Event> {
        let mut events = Vec::new();
        let surface_old = self.surface(old_position);
        let surface_new = self.surface(old_position);

        match (surface_old,surface_new){
            (Some(id1),Some(id2))=>{
                if id1 != id2 {
                    let surface_id = id1.into();
                    let event = SeatEvent::Cursor(CursorEvent::Left{surface_id});
                    events.push(Event::Seat{id,event});

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

                    //Position2D{x: point.coords.x as u32,y: point.coords.y as u32}
*/


                    let surface_id = id2.into();
                    let position = new_position;
                    let event = SeatEvent::Cursor(CursorEvent::Entered{surface_id,position});
                    events.push(Event::Seat{id,event});
                }

                let position = new_position;
                let event = SeatEvent::Cursor(CursorEvent::AbsoluteMovement{position});
                events.push(Event::Seat{id,event});
            }
            _=>{}
        }

        events
    }

    pub fn surface(&self,position: Position2D<u32>)->Option<usize> {
        for surface_id in &self.surface_stack {
            if let Some(surface) = self.surfaces.get(surface_id){
                if surface.contains(position){return Some(*surface_id);}
            }
        }
        None
    }

}
*/
