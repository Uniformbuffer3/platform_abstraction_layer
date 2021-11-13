use crate::definitions::*;

pub enum OutputUpdate {
    PositionChanged{id: usize,position: Position2D<i32>}
}

pub struct Output {
    id: usize,
    geometry: Rectangle<i32,u32>,
}
impl Output {
    pub fn new(id: usize, geometry: impl Into<Rectangle<i32,u32>>)->Self {
        let geometry = geometry.into();
        Self {id,geometry}
    }
}

pub struct OutputManager {
    outputs: Vec<Output>
}
impl OutputManager {
    pub fn new()->Self {
        let outputs = Vec::new();
        Self {outputs}
    }

    pub fn add_output(&mut self, id: usize, size: Size2D<u32>){
        let x_offset = *self.outputs.last().map(|output|output.geometry.x_offset()).get_or_insert(0);
        let position = Position2D::from((x_offset,0));
        self.outputs.push(Output::new(id,(position,size)));
    }
    pub fn del_output(&mut self, id: usize)->Vec<OutputUpdate>{
        let indexes_to_update = self.outputs.iter()
        .position(|output|output.id == id)
        .map(|position|{self.outputs.remove(position);position..self.outputs.len()})
        .get_or_insert(0..0)
        .clone();

        self.update_offset(indexes_to_update)
    }

    pub fn output_ref<T>(&self,id: usize, callback: impl Fn(&Output)->T)->Option<T> {
        self.outputs.iter().find_map(|output|{
            if output.id == id {Some(callback(output))}
            else{None}
        })
    }
    pub fn surface_mut<T>(&mut self,id: usize, callback: impl Fn(&mut Output)->T)->Option<(T,Vec<OutputUpdate>)> {
        self.outputs.iter_mut().enumerate().find_map(|(index,output)|{
            if output.id == id {Some((index,callback(output)))}
            else{None}
        }).map(|(index,result)|(result,self.update_offset(index..self.outputs.len())))
    }

    fn update_offset<'a>(&'a mut self,range: impl std::ops::RangeBounds<usize>+Iterator<Item=usize> )->Vec<OutputUpdate> {
        let mut updates = Vec::new();
        for index in range {
            let x_offset = if index == 0{0}
            else{self.outputs[index-1].geometry.x_offset()};

            if self.outputs[index].geometry.position.x == x_offset {return updates;}
            else {
                self.outputs[index].geometry.position.x = x_offset;
                let update = OutputUpdate::PositionChanged{
                    id: self.outputs[index].id,
                    position: self.outputs[index].geometry.position
                };
                updates.push(update);
            }
        }
        updates
    }
}
