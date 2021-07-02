
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
    let mut platform = Platform::new(ExternalContext::Raw);

    let mut initial_requests = Vec::new();

    let initial_events = platform.dispatch();
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
    println!("{:#?}", initial_events);
    platform.request(initial_requests);

    let mut surfaces = HashMap::new();
    'main_loop: loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let events = platform.dispatch();

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
        println!("{:#?}", events);
    }

    //println!("Dispatch2: {:#?}",platform_backend.dispatch());
}

#[test]
fn virtual_output_backend(){
    use crate::definitions::*;
    use crate::backends::partial_backends::output_manager::OutputManager;
    let mut output_manager = OutputManager::new();

    println!("Adding output 1");
    let events = output_manager.add_output(
        1u32.into(),
        Mode{
            resolution: Size {width: 1920, height: 1080},
            refresh_rate: 60,
            is_preferred: true
        },
        vec![Mode{
                resolution: Size {width: 1920, height: 1080},
                refresh_rate: 60,
                is_preferred: true
        }],
        Size {width: 0,height: 0},
        Subpixel::Unknown
    );
    println!("{:#?}",events);

    println!("Adding output 2");
    let events = output_manager.add_output(
        2u32.into(),
        Mode{
            resolution: Size {width: 1920, height: 1080},
            refresh_rate: 60,
            is_preferred: true
        },
        vec![Mode{
                resolution: Size {width: 1920, height: 1080},
                refresh_rate: 60,
                is_preferred: true
        }],
        Size {width: 0,height: 0},
        Subpixel::Unknown
    );
    println!("{:#?}",events);

    println!("Adding output 3");
    let events = output_manager.add_output(
        3u32.into(),
        Mode{
            resolution: Size {width: 1920, height: 1080},
            refresh_rate: 60,
            is_preferred: true
        },
        vec![Mode{
                resolution: Size {width: 1920, height: 1080},
                refresh_rate: 60,
                is_preferred: true
        }],
        Size {width: 0,height: 0},
        Subpixel::Unknown
    );
    println!("{:#?}",events);

    println!("Removing output 1");
    let events = output_manager.del_output(1u32.into());
    println!("{:#?}",events);

}
