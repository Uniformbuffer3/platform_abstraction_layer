use std::collections::HashMap;
use log::info;

use crate::definitions::{Position2D,Size2D,OutputId};

use parry2d::{
    shape::{Cuboid,Compound,SharedShape},
    math::{Real,Vector,Isometry},
    query::{closest_points::ClosestPoints}
};
use parry2d::query::{closest_points,contact,distance};

use petgraph::{
    Undirected,
    prelude::GraphMap,
    visit::Dfs,
    prelude::UnGraphMap
};

pub trait OutputManager {
    fn on_add(&mut self,graph: &GraphMap<OutputId,(),Undirected>,outputs: &HashMap<OutputId,PhysicalOutput>, id: OutputId, size: Size2D<u32>)->Vec<PhysicalOutputEvent>;
    fn on_move(&mut self,graph: &GraphMap<OutputId,(),Undirected>,outputs: &HashMap<OutputId,PhysicalOutput>, id: OutputId, position: Position2D<u32>)->Vec<PhysicalOutputEvent>;
    fn on_remove(&mut self,graph: &GraphMap<OutputId,(),Undirected>,outputs: &HashMap<OutputId,PhysicalOutput>, id: OutputId)->Vec<PhysicalOutputEvent>;
}

#[derive(Debug,Clone, PartialEq)]
pub enum PhysicalOutputEvent {
    Added{id: OutputId, output: PhysicalOutput},
    Moved{id: OutputId, position: Position2D<u32>},
    Removed{id: OutputId },
}

#[derive(Debug,Clone, PartialEq)]
pub struct PhysicalOutput {
    pub position: Position2D<u32>,
    pub size: Size2D<u32>
}
impl PhysicalOutput {
    pub fn new(position: Position2D<u32>,size: Size2D<u32>)->Self {
        Self {position,size}
    }
    pub fn get_shape(&self)->(Isometry<Real>,Cuboid){
        let width = (self.size.width-1) as f32/2.0;
        let height = (self.size.height-1) as f32/2.0;
        let isometry = Isometry::translation(self.position.x as f32+width,self.position.y as f32+height);
        let cuboid = Cuboid::new(Vector::new(width,height));
        (isometry,cuboid)
    }
    pub fn get_shared_shape(&self)->(Isometry<Real>,SharedShape){
        let (isometry,cuboid) = self.get_shape();
        (isometry,SharedShape::new(cuboid))
    }
    fn is_adjacent(&self, other: &PhysicalOutput)->bool{
        let (isometry1,shape1) = self.get_shape();
        let (isometry2,shape2) = other.get_shape();
        distance(&isometry1,&shape1,&isometry2,&shape2).unwrap() == 1.0
    }
}

pub struct OutputConstraintKeeper<T: OutputManager> {
    output_manager: T,
    outputs: HashMap<OutputId,PhysicalOutput>,
    graph: UnGraphMap<OutputId,()>,

    shape: Compound,
    max_dist: Real
}

impl<T: OutputManager> OutputConstraintKeeper<T> {
    pub fn new(output_manager: T)->Self {
        let outputs = HashMap::new();
        let graph = GraphMap::new();

        let shape = Compound::new(vec![(
            Isometry::translation(0.0,0.0),
            SharedShape::new(Cuboid::new(Vector::new(0.0,0.0)))
        )]);
        let max_dist = 0.0;

        //outputs.node_weight(NodeIndex::new(3));

        //let d: &dyn DataMap<EdgeId = EdgeIndex,NodeId = NodeIndex,NodeWeight = PhysicalOutput,EdgeWeight = ()> = &outputs;

        Self {
            output_manager,
            outputs,
            graph,

            shape,
            max_dist
        }
    }
    pub fn add_output(&mut self,id: OutputId, size: Size2D<u32>)->Vec<PhysicalOutputEvent> {
        let events = self.output_manager.on_add(&self.graph,&self.outputs,id,size);

        self.apply_events(events.clone());
        events
    }

    pub fn move_output(&mut self,id: OutputId, position: Position2D<u32>)->Vec<PhysicalOutputEvent> {
        let events = self.output_manager.on_move(&self.graph,&self.outputs,id,position);
        self.apply_events(events.clone());
        events
    }

    pub fn remove_output(&mut self,id: OutputId)->Vec<PhysicalOutputEvent> {
        let events = self.output_manager.on_remove(&self.graph,&self.outputs,id);
        self.apply_events(events.clone());
        events
    }

    fn apply_events(&mut self, events: Vec<PhysicalOutputEvent>){
        for event in events {
            match event {
                PhysicalOutputEvent::Added{id,output}=>{
                    info!("Applying PhysicalOutputEvent::Added event");
                    self.outputs.insert(id,output);
                    self.graph.add_node(id);
                    self.rebuild_neighbors(id);
                }
                PhysicalOutputEvent::Moved{id,position}=>{
                    info!("Applying PhysicalOutputEvent::Moved event");
                    match self.outputs.get_mut(&id){
                        Some(output)=>{
                            //let neighbors: Vec<OutputId> = self.graph.neighbors(id).collect();
                            output.position = position;
                            self.rebuild_edges();
                            self.merge_groups();
                        },
                        None=>()
                    }


                }
                PhysicalOutputEvent::Removed{id}=>{
                    info!("Applying PhysicalOutputEvent::Removed event");
                    self.graph.remove_node(id);
                    self.outputs.remove(&id);
                    self.merge_groups();


                    //let neighbors: Vec<OutputId> = self.graph.neighbors(id).collect();
                    //let mut bfs = Bfs::new(&graph, a);
                }
            }
        }
    }

    fn rebuild_edges(&mut self){
        // Remove edges
        let edges: Vec<(OutputId,OutputId)> = self.graph.all_edges().map(|(output1,output2,_edge)|(output1,output2)).collect();
        edges.into_iter().for_each(|(output1,output2)|{self.graph.remove_edge(output1,output2);});

        //Recalculate edges
        for (id1,output1) in &self.outputs {
            for (id2,output2) in &self.outputs {
                if id1 != id2 && output1.is_adjacent(output2) {
                    self.graph.add_edge(*id1,*id2,());
                }
            }
        }
    }

    fn rebuild_neighbors(&mut self,id: OutputId){
        let updated_output = match self.outputs.get(&id){
            Some(output)=>output.clone(),
            None=>return
        };

        let current_neighbors: Vec<OutputId> = self.graph.neighbors(id).collect();
        for neighbor_id in current_neighbors {
            match self.outputs.get(&neighbor_id){
                Some(output)=>{
                    if !updated_output.is_adjacent(output){self.graph.remove_edge(id,neighbor_id);}
                }
                None=>()
            }
        }

        let all_nodes: Vec<OutputId> = self.graph.nodes().collect();
        for output_id in all_nodes {
            if output_id == id {continue}
            let other_output = match self.outputs.get(&output_id){
                Some(output)=>output,
                None=>continue
            };
            if updated_output.is_adjacent(other_output){self.graph.add_edge(id,output_id,());}
        }
    }

    /// Check if the ids are connected together
    pub fn connected(&self,ids: Vec<OutputId>)->bool {
        match ids.get(0) {
            Some(id)=>{
                let mut connected = true;
                let dfs = Dfs::new(&self.graph, *id);
                for id in ids {
                    if dfs.stack.iter().find(|dfs_id|*dfs_id == &id).is_none(){
                        connected = false;
                        break;
                    }
                }
                connected
            }
            None=>false
        }
    }
    /// Split outputs into groups: all the outputs adjacent each other will belong to the same group.
    pub fn groups(&self)->Vec<Vec<OutputId>>{
        let mut groups: Vec<Vec<OutputId>> = Vec::new();

        let mut output_ids: Vec<OutputId> = self.graph.nodes().collect();
        while !output_ids.is_empty(){
            let first_output = output_ids[0];
            let mut new_group = Vec::new();
            let mut dfs = Dfs::new(&self.graph, first_output);
            while let Some(id) = dfs.next(&self.graph){
                if let Some(index) = output_ids.iter().position(|output_id|output_id == &id){
                    let removed_output = output_ids.remove(index);
                    new_group.push(removed_output);
                }
            }
            groups.push(new_group);
        }

        groups
    }

    /// Find the nearest output. Return none if the id is invalid or if the id is the only output present.
    pub fn nearest(&self,id: OutputId)->Option<OutputId> {
        let (isometry,shape) = match self.outputs.get(&id){
            Some(output)=>output.get_shape(),
            None=>return None
        };

        let mut nearest_id = None;
        let mut nearest_distance = f32::MAX;
        for (output_id,output) in &self.outputs {
            if &id == output_id {continue}
            let (current_isometry,current_shape) = output.get_shape();
            let distance = distance(&isometry,&shape,&current_isometry,&current_shape).unwrap();

            if distance < nearest_distance {nearest_distance = distance;nearest_id = Some(*output_id);}
        }
        nearest_id
    }

    fn align(&mut self,ids: &Vec<OutputId>,other_ids: &Vec<OutputId>)->Vec<PhysicalOutputEvent>{
        let compound = self.build_compound(&ids);
        let other_compound = self.build_compound(&other_ids);
        let origin = Isometry::translation(0.0,0.0);

        let closest_points = closest_points(&origin,&compound,&origin,&other_compound,f32::MAX).unwrap();
        let (x_offset,y_offset) = match closest_points{
            ClosestPoints::Intersecting=>{
                let contact = contact(&origin,&compound,&origin,&other_compound,f32::MAX).unwrap().unwrap();
                let x_offset = contact.point2.x - contact.point1.x;
                let y_offset = contact.point2.y - contact.point1.y;
                (x_offset,y_offset)
                /*
                info!("Contact: {:#?}",contact);
                info!("Offset2D<f32>: {},{}",x_offset,y_offset);

                other_ids.iter().filter_map(|other_id|{
                    match self.outputs.get_mut(other_id){
                        Some(other_output)=>{
                            info!("Current position: {:#?} - {:#?}",other_id,other_output);
                            other_output.position.x = (other_output.position.x as i32 + x_offset as i32) as u32 +1;
                            other_output.position.y = (other_output.position.y as i32 + y_offset as i32) as u32 +1;
                            Some(PhysicalOutputEvent::Moved{id: *other_id,position: other_output.position})
                        }
                        None=>None
                    }
                }).collect()*/


            }
            ClosestPoints::WithinMargin(point1, point2)=>{
                let x_offset = point1.x - point2.x;
                let y_offset = point1.y - point2.y;
                (x_offset,y_offset)
            },
            ClosestPoints::Disjoint=>{unreachable!()}
        };

        let x_adjacent = -1.0 <= x_offset && x_offset <= 1.0;
        let y_adjacent = -1.0 <= y_offset && y_offset <= 1.0;

        match (x_adjacent,y_adjacent) {
            (true,true)=>Vec::new(),
            (true,false)=>{
                other_ids.iter().filter_map(|other_id|{
                    match self.outputs.get_mut(other_id){
                        Some(other_output)=>{
                            other_output.position.y = (other_output.position.y as i32 + y_offset as i32) as u32 +1;
                            Some(PhysicalOutputEvent::Moved{id: *other_id,position: other_output.position})
                        }
                        None=>None
                    }
                }).collect()
            }
            (false,true)=>{
                other_ids.iter().filter_map(|other_id|{
                    match self.outputs.get_mut(other_id){
                        Some(other_output)=>{
                            other_output.position.x = (other_output.position.x as i32 + x_offset as i32) as u32 +1;
                            Some(PhysicalOutputEvent::Moved{id: *other_id,position: other_output.position})
                        }
                        None=>None
                    }
                }).collect()
            }
            (false,false)=>{
                other_ids.iter().filter_map(|other_id|{
                    match self.outputs.get_mut(other_id){
                        Some(other_output)=>{
                            other_output.position.x = (other_output.position.x as i32 + x_offset as i32) as u32 +1;
                            other_output.position.y = (other_output.position.y as i32 + y_offset as i32) as u32 +1;
                            Some(PhysicalOutputEvent::Moved{id: *other_id,position: other_output.position})
                        }
                        None=>None
                    }
                }).collect()
            }
        }
    }

    fn build_compound(&self, ids: &Vec<OutputId>)->Compound {
        let shapes: Vec<(Isometry<Real>,SharedShape)> = ids.iter().map(|id|self.outputs.get(id).unwrap().get_shared_shape()).collect();
        Compound::new(shapes)
    }

    fn merge_groups(&mut self){
        let mut groups = self.groups();
        if groups.len() <= 1 {}
        else {
            info!("Detected {} separate groups of outputs: merging together",groups.len());
            let main_group = groups.remove(0);

            let new_events: Vec<PhysicalOutputEvent> = groups.iter().map(|group|{
                info!("Aligning outputs {:?} with {:?}",main_group,group);
                self.align(&main_group,&group)
            }).flatten().collect();
            self.apply_events(new_events);
        }
    }
}



/*

    pub fn add_output(&mut self,size: Size2D<u32>)->Vec<PhysicalOutputEvent> {

        let id = self.outputs.add_node(PhysicalOutputEvent{position: Position2D{x:0,y:0},size: Size2D<u32>{width: 0,height: 0}}).index();
        let events = self.output_manager.on_add(&self.outputs,&size);

        self.apply_events(events);


/*
        let position = if self.output_stack.is_empty() {Position2D::from((0,0))}
        else {

            contact(&Isometry::translation(0.0,0.0),&self.shape,&monitor3.0,&monitor3.1,0.0)
            //let last_output = self.outputs.get(self.output_stack.last().unwrap()).unwrap();
            //Position2D::from((last_output.position.x + last_output.size.width,0))
        };

*/
        //self.output_stack.push(id);
        self.rebuild_shape();

        events
    }

    pub fn move_output(&mut self, id: OutputId, position: Position2D)->Option<PhysicalOutputEvent>{
        //Clone the outputs.
        let mut outputs = self.outputs.clone();
        // Remove the target output, leaving all the others.
        let mut target_output = outputs.remove(&id);

        match target_output {
            Some(mut output)=>{
                //Updating position of the target output.
                output.position = position;

                //Building compound from other outputs.
                let other_outputs: Vec<(Isometry<Real>,SharedShape)> = outputs.values().cloned().map(|physical_output|{
                    let (isometry,cuboid) = physical_output.get_shape();
                    (isometry,SharedShape::new(cuboid))
                }).collect();
                let compound = Compound::new(other_outputs);

                //Align the target output with the compound builded from the other outputs.
                Self::align_output(&compound,id,&mut output);

                //Apply the aligned position to the actual target output.
                self.outputs.get_mut(&id).unwrap().position = output.position;
                Some(PhysicalOutputEvent::Moved{id,position: output.position})
            }
            None=>None
        }






/*

        match self.outputs.remove(&id){
            Some(output)=>{
                self.rebuild_shape();
                output.position = position;
                Self::align_output(&self.shape,id,&mut output);
                self.outputs.insert(id,output);
            },
            None=>()
        }
        */
    }

    pub fn del_output(&mut self, id: OutputId)->Vec<PhysicalOutputEvent>{
        match self.outputs.remove(&id) {
            Some(removed_output)=>{
                self.output_manager.on_del(&mut self.outputs);
                let shapes: Vec<(Isometry<Real>,Cuboid)> = self.outputs.values().map(|output|output.get_shape()).collect();
            }
            None=>{

            }
        }
        /*
        let mut events = Vec::new();
        if let Some(removed_output) = self.outputs.remove(&id){
            events.push(PhysicalOutputEvent::Removed{id});
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
        */
    }

    pub fn apply_limit(&self, old_position: Position2D,new_position: Position2D)->Position2D {
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

        Position2D{x: point.coords.x as u32,y: point.coords.y as u32}
    }

    fn rebuild_shape(&mut self){
        let mut cuboids = Vec::new();
        for (output_id,output) in self.outputs.iter() {
            let position = output.position;
            let size = output.size;

            let width = (size.width-1) as f32/2.0;
            let height = (size.height-1) as f32/2.0;
            let isometry = Isometry::translation(position.x as f32+width,position.y as f32+height);
            let cuboid = SharedShape::new(Cuboid::new(Vector::new(width,height)));

            cuboids.push((isometry,cuboid));
        }
        self.shape = Compound::new(cuboids);
        self.max_dist = self.shape.local_bounding_sphere().radius*2.0;
    }
    fn update_outputs_from(&mut self,output: (usize,PhysicalOutput))->Vec<PhysicalOutputEvent> {
        let (index,output) = output;
        let mut events = Vec::new();

        let mut offset = output.position.x;
        for output_index in index..self.output_stack.len() {
            let id = self.output_stack[output_index];
            let mut output = self.outputs.get_mut(&id).unwrap();
            output.position.x = offset;
            let position = output.position;
            events.push(PhysicalOutputEvent::Moved{id,position});
            offset += output.size.width;
        }

        events
    }




    fn rebuild_adjacency_matrix(&mut self){

    }

    fn apply_events(&mut self, events: Vec<PhysicalOutputEvent>){
        for event in events {
            match event {
                PhysicalOutputEvent::Added{id, position, size}=>{
                    let mut output = PhysicalOutput{position,size};



                    Self::align_output(&self.shape,id,&mut output);
                    self.outputs.insert(id,output);
                },
                PhysicalOutputEvent::Removed{id}=>{
                    self.outputs.remove(&id);
                },
                PhysicalOutputEvent::Moved{id, position}=>{
                    match self.outputs.get(&id){
                        Some(output)=>{
                            let mut adjacent_outputs = HashMap::new();
                            self.outputs.iter().for_each(|(output_id,output)|{

                            });
                        }
                        None=>()
                    }




                    //Clone the outputs.
                    let mut outputs = self.outputs.clone();
                    // Remove the target output, leaving all the others.
                    let mut target_output = outputs.remove(&id);

                    match target_output {
                        Some(mut output)=>{
                            //Updating position of the target output.
                            output.position = position;

                            //Building compound from other outputs.
                            let other_outputs: Vec<(Isometry<Real>,SharedShape)> = outputs.values().cloned().map(|physical_output|{
                                let (isometry,cuboid) = physical_output.get_shape();
                                (isometry,SharedShape::new(cuboid))
                            }).collect();
                            let compound = Compound::new(other_outputs);

                            //Align the target output with the compound builded from the other outputs.
                            Self::align_output(&compound,id,&mut output);

                            //Apply the aligned position to the actual target output.
                            self.outputs.get_mut(&id).unwrap().position = output.position;
                        }
                        None=>None
                    }
                }
            }

        }
    }
*/
