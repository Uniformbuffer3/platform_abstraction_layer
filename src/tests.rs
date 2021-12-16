
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

    let mut platform = Platform::new(vec![Box::new(RawContext)]);

    match platform.platform_type(){
        PlatformType::Compositor=>{
            platform.request(vec![Request::Surface{request: SurfaceRequest::Create(None)}]);
        }
        PlatformType::Direct=>{
            let initial_requests: Vec<_> = platform.events().into_iter().filter_map(|event|{
                match event {
                    Event::Output{time: _, id, event: OutputEvent::Added(_)}=>{
                        Some(Request::Surface{request: SurfaceRequest::Create(Some(id))})
                    },
                    _=>None
                }
            }).collect();

            platform.request(initial_requests);
        }
    }

    use std::os::unix::io::AsRawFd;
    let mut event_loop = calloop::EventLoop::try_new().unwrap();
    let interest = calloop::Interest {
        readable: true,
        writable: false,
    };
    event_loop.handle().insert_source(calloop::generic::Generic::new(platform.as_raw_fd(),interest,calloop::Mode::Edge),move|_event,_metadata,_data|Ok(calloop::PostAction::Continue)).unwrap();

    let mut surfaces = HashMap::new();
    'main_loop: loop {
        match event_loop.dispatch(None,&mut ()){
            Ok(_)=>(),
            Err(_)=>()
        }
        let events = platform.events();
        println!("{:#?}", events);
        //if !events.is_empty() {println!("{:#?}", events);}

        for event in events {
            match event {
                Event::Surface{ time: _, id, event: SurfaceEvent::Added(_) }=>{
                    surfaces.insert(id,());
                }
                Event::Surface{ time: _, id, event: SurfaceEvent::Removed }=>{
                    surfaces.remove(&id);
                    if surfaces.is_empty() {break 'main_loop;}
                }
                _=>{}
            }
        }

    }
}


#[test]
fn cursor_test() {
    use crate::Platform;
    use crate::definitions::*;

    let mut platform = Platform::new(vec![Box::new(RawContext)]);

    match platform.platform_type(){
        PlatformType::Compositor=>{
            platform.request(vec![Request::Surface{request: SurfaceRequest::Create(None)}]);
        }
        PlatformType::Direct=>{
            let initial_requests: Vec<_> = platform.events().into_iter().filter_map(|event|{
                match event {
                    Event::Output{time: _, id, event: OutputEvent::Added(_)}=>{
                        Some(Request::Surface{request: SurfaceRequest::Create(Some(id))})
                    },
                    _=>None
                }
            }).collect();

            platform.request(initial_requests);
        }
    }

    use std::os::unix::io::AsRawFd;
    let mut event_loop = calloop::EventLoop::try_new().unwrap();
    let interest = calloop::Interest {
        readable: true,
        writable: false,
    };
    event_loop.handle().insert_source(calloop::generic::Generic::new(platform.as_raw_fd(),interest,calloop::Mode::Edge),move|_event,_metadata,_data|Ok(calloop::PostAction::Continue)).unwrap();

    let mut surfaces = HashMap::new();
    'main_loop: loop {
        match event_loop.dispatch(None,&mut ()){
            Ok(_)=>(),
            Err(_)=>()
        }
        let events = platform.events();
        println!("{:#?}", events);
        //if !events.is_empty() {println!("{:#?}", events);}

        for event in events {
            match event {
                Event::Surface{ time: _, id, event: SurfaceEvent::Added(_) }=>{
                    surfaces.insert(id,());
                }
                Event::Surface{ time: _, id, event: SurfaceEvent::Removed }=>{
                    surfaces.remove(&id);
                    if surfaces.is_empty() {break 'main_loop;}
                }
                Event::Seat{ time: _, id, event: SeatEvent::Cursor(CursorEvent::Entered{..}) }=>{
                    platform.request(vec![Request::Seat {
                        request: SeatRequest::Cursor(CursorRequest::ChangeImage(CursorImage::Hidden))
                    }]);
                }
                Event::Seat{ time: _, id, event: SeatEvent::Cursor(CursorEvent::Left{..}) }=>{
                    platform.request(vec![Request::Seat {
                        request: SeatRequest::Cursor(CursorRequest::ChangeImage(CursorImage::Default))
                    }]);
                }
                _=>{}
            }
        }

    }
}
