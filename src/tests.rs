
/*
#[test]
fn test_create(){
    let mut platform_backend = PlatformBackend::new(Vec::new());
}

*/
use std::collections::HashMap;

#[test]
fn create_window() {
    use crate::definitions::GraphicBackend;
    use crate::definitions::InputBackend;
    use crate::{Platform};
    use crate::definitions::{Event,SurfaceEvent,OutputEvent};

    let mut platform = Platform::new(Vec::new());

    let mut surfaces = HashMap::new();
    'main_loop: loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let events = platform.dispatch();

        for event in &events {
            match event {
                Event::Output{id,event: OutputEvent::Added {..}}=>{platform.create_surface(id.clone());},
                Event::Surface {id, event: SurfaceEvent::Added{
                    _x,_y,_width,_height
                }}=>{
                    surfaces.insert(id.clone(),());
                }
                Event::Surface {id, event: SurfaceEvent::Destroyed}=>{
                    surfaces.remove(id);
                    if surfaces.is_empty() {break 'main_loop;}
                }
                _=>{}
            }
        }
        println!("{:#?}", events);
    }

    //println!("Dispatch2: {:#?}",platform_backend.dispatch());
}
