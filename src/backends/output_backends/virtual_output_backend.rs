use std::collections::HashMap;
use petgraph::stable_graph::{StableGraph,NodeIndex};
use simple_matrix::Matrix;

use crate::definitions::{OutputEventType,Position,Size,Mode,Subpixel,OutputInfo,OutputId,SeatId,SurfaceId};

#[derive(Debug,Copy,Clone)]
struct Output {
    pub position: Position,
    pub size: Size
}

pub struct VirtualOutputBackend {
    outputs: HashMap<OutputId,Output>,
    output_stack: Vec<OutputId>
}

impl VirtualOutputBackend {
    pub fn new()->Self {
        let outputs = HashMap::new();
        let output_stack = Vec::new();
        Self {outputs,output_stack}
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

        self.outputs.insert(id,Output{position,size: selected_mode.resolution});
        self.output_stack.push(id);

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
        }
        else {println!("Warning: invalid index requested");}
        events
    }


    //pub fn virtual_size(&self){}

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
