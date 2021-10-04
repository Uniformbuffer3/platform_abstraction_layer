
/*
#[test]
fn test_create(){
    let mut platform_backend = PlatformBackend::new(Vec::new());
}

*/
use std::collections::HashMap;

#[test]
fn create_window() {
    use crate::Platform;
    use crate::definitions::*;

    let mut platform = Platform::new(vec![RawContext]);

    match platform.platform_type(){
        PlatformType::Compositor=>{
            platform.request(vec![SurfaceRequest::Create(None).into()]);
        }
        PlatformType::Direct=>{
            let mut initial_requests = Vec::new();

            let initial_events = platform.events();
            for event in &initial_events {
                match event {
                    Event::Output(event)=>{
                        if let OutputEventType::Added(_) = event.event_type {
                            initial_requests.push(SurfaceRequest::Create(Some(event.id)).into());
                        }
                    },
                    _=>{}
                }

            }
            platform.request(initial_requests);
        }
    }

    let mut surfaces = HashMap::new();
    'main_loop: loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let events = platform.events();
        if !events.is_empty() {println!("{:#?}", events);}

        for event in &events {
            match event {
                Event::Surface(event)=>{
                    match event.event_type {
                        SurfaceEventType::Added(_)=>{
                            surfaces.insert(event.id,());
                        }
                        SurfaceEventType::Removed=>{
                            surfaces.remove(&event.id);
                            if surfaces.is_empty() {break 'main_loop;}
                        }
                        _=>{}
                    }
                }
                _=>{}
            }
        }

    }

    //println!("Dispatch2: {:#?}",platform_backend.events());
}


use crate::backends::partial_backends::output_manager::PhysicalOutput;
use crate::backends::partial_backends::output_manager::OutputManager;
use crate::backends::partial_backends::output_manager::PhysicalOutputEvent;
use crate::definitions::Position;
use petgraph::prelude::GraphMap;
use petgraph::Undirected;

pub struct SimpleOutputManager {
    stack: Vec<OutputId>
}
impl SimpleOutputManager {
    pub fn new()->Self {
        let stack = Vec::new();
        Self {stack}
    }
}
impl OutputManager for SimpleOutputManager {
    fn on_add(&mut self,graph: &GraphMap<OutputId,(),Undirected>,outputs: &HashMap<OutputId,PhysicalOutput>, id: OutputId, size: Size)->Vec<PhysicalOutputEvent>{
        let mut x = 0;
        for output_id in &self.stack {
            let output = match outputs.get(output_id){
                Some(output)=>output,
                None=>continue
            };
            x += output.size.width;
        }
        let y = 0;

        let position = Position{x,y};
        let output = PhysicalOutput{position,size};
        self.stack.push(id);
        vec![PhysicalOutputEvent::Added{id,output}]
    }
    fn on_move(&mut self,graph: &GraphMap<OutputId,(),Undirected>,outputs: &HashMap<OutputId,PhysicalOutput>, id: OutputId, position: Position)->Vec<PhysicalOutputEvent>{
        vec![PhysicalOutputEvent::Moved{id,position}]
    }
    fn on_remove(&mut self,graph: &GraphMap<OutputId,(),Undirected>,outputs: &HashMap<OutputId,PhysicalOutput>, id: OutputId)->Vec<PhysicalOutputEvent>{
        vec![PhysicalOutputEvent::Removed{id}]
    }
}
use crate::definitions::*;
#[test]
fn virtual_output_backend(){
    use crate::backends::partial_backends::output_manager::OutputConstraintKeeper;

    env_logger::init();

    let mut output_manager = OutputConstraintKeeper::new(SimpleOutputManager::new());

    let events = output_manager.add_output(
        1u32.into(),
        Size {width: 10,height: 10}
    );

    let events = output_manager.add_output(
        2u32.into(),
        Size {width: 10,height: 10}
    );

    let events = output_manager.add_output(
        3u32.into(),
        Size {width: 10,height: 10}
    );
    let events = output_manager.remove_output(2u32.into());
}
