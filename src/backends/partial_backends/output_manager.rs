use std::collections::HashMap;

use crate::definitions::{OutputEventType,Position,Size,Mode,Subpixel,OutputInfo,OutputId};

use parry2d::{
    shape::{Cuboid,Segment,Compound,SharedShape},
    math::{Real,Vector,Point,Isometry},
    query::{Ray,RayCast}
};


#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Output{
    pub position: Position,
    pub size: Size
}
impl Output {
    pub fn new(position: Position,size: Size)->Self {
        Self {position,size}
    }
}

pub struct OutputManager {
    outputs: HashMap<OutputId,Output>,
    output_stack: Vec<OutputId>,

    shape: Compound,
    max_dist: Real
}

impl OutputManager {
    pub fn new()->Self {
        let outputs = HashMap::new();
        let output_stack = Vec::new();
        let shape = Compound::new(vec![(
            Isometry::translation(0.0,0.0),
            SharedShape::new(Cuboid::new(Vector::new(0.0,0.0)))
        )]);
        let max_dist = 0.0;
        Self {outputs,output_stack,shape,max_dist}
    }

    pub fn add_output(&mut self,
        id: OutputId,
        selected_mode: Mode,
        available_modes: Vec<Mode>,
        physical_size: Size,
        subpixel: Subpixel
    )->Vec<(OutputId,OutputEventType)> {
        let position = if self.output_stack.is_empty() {Position::from((0,0))}
        else {
            let last_output = self.outputs.get(self.output_stack.last().unwrap()).unwrap();
            Position::from((last_output.position.x + last_output.size.width,0))
        };

        let output = Output{position,size: selected_mode.resolution};
        self.outputs.insert(id,output);
        self.output_stack.push(id);
        self.rebuild_shape();

        let output_info = OutputInfo {position,selected_mode,available_modes,physical_size,subpixel};
        vec![(id,OutputEventType::Added(output_info))]
    }

    pub fn del_output(&mut self, id: OutputId)->Vec<(OutputId,OutputEventType)>{
        let mut events = Vec::new();
        if let Some(removed_output) = self.outputs.remove(&id){
            events.push((id,OutputEventType::Removed));
            if let Some(index) = self.output_stack.iter().position(|output_id|id == *output_id){
                if index != self.output_stack.len()-1 {
                    events.append(&mut self.update_outputs_from((index+1,removed_output)));
                }
                self.output_stack.remove(index);
            }
            self.rebuild_shape();
        }
        else {println!("Warning: invalid index requested");}
        events
    }

    pub fn apply_limit(&self, old_position: Position,new_position: Position)->Position {
        let old_position = Point::new(old_position.x as f32,old_position.y as f32);
        let new_position = Point::new(new_position.x as f32, new_position.y as f32);
        let segment = Segment::new(old_position,new_position);

        let ray = Ray::new(old_position,*segment.direction().unwrap());
        let result = self.shape.cast_ray_and_get_normal(
            &Isometry::translation(0.0,0.0),
            &ray,
            self.max_dist,
            false
        ).unwrap();
        let point = ray.point_at(result.toi);

        Position{x: point.coords.x as u32,y: point.coords.y as u32}
    }

    fn rebuild_shape(&mut self){
        let mut cuboids = Vec::new();
        for output_id in &self.output_stack {
            let output = self.outputs.get(output_id).unwrap();
            let position = output.position;
            let size = output.size;

            let width = size.width as f32/2.0;
            let height = size.height as f32/2.0;
            let isometry = Isometry::translation(position.x as f32+width,position.y as f32+height);
            let cuboid = SharedShape::new(Cuboid::new(Vector::new(width,height)));

            cuboids.push((isometry,cuboid));
        }
        self.shape = Compound::new(cuboids);
        self.max_dist = self.shape.local_bounding_sphere().radius*2.0;
    }
    fn update_outputs_from(&mut self,output: (usize,Output))->Vec<(OutputId,OutputEventType)> {
        let (index,output) = output;
        let mut events = Vec::new();

        let mut offset = output.position.x;
        for output_index in index..self.output_stack.len() {
            let output_id = self.output_stack[output_index];
            let mut output = self.outputs.get_mut(&output_id).unwrap();
            output.position.x = offset;
            events.push((output_id,OutputEventType::PositionChanged(output.position)));
            offset += output.size.width;
        }

        events
    }
}
