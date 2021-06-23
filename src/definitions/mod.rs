mod event;
pub use event::*;

pub trait PlatformBackend: SeatBackend + OutputBackend + SurfaceBackend {
    fn dispatch(&mut self) -> Vec<Event> {
        let mut events: Vec<Event> = self.dispatch_seats().into_iter().map(|event|Event::from(event)).collect();
        events.append(&mut self.dispatch_outputs().into_iter().map(|event|Event::from(event)).collect());
        events.append(&mut self.dispatch_surfaces().into_iter().map(|event|Event::from(event)).collect());
        events
    }
}

pub trait SeatBackend {
    fn dispatch_seats(&mut self) -> Vec<SeatEvent> {Vec::new()}
    fn set_keyboard_layout(&mut self, layout: String);
}

pub trait OutputBackend {
    fn dispatch_outputs(&mut self) -> Vec<OutputEvent> {Vec::new()}
}

pub trait SurfaceBackend {
    fn dispatch_surfaces(&mut self) -> Vec<SurfaceEvent> {Vec::new()}
    fn create_surface(&mut self, output: Option<OutputId>);
}

#[derive(Debug)]
pub enum ExternalContext {
    Raw
}

#[derive(Debug)]
pub enum CreateSurfaceError {
    TooManySurfaces,
    InvalidOutputId,
}

#[derive(Debug)]
pub enum RawSurfaceHandle {
    #[cfg(unix)]
    Xcb(raw_window_handle::unix::XcbHandle),
    #[cfg(unix)]
    Wayland(raw_window_handle::unix::WaylandHandle),
    Vulkan
}
impl std::convert::TryInto<raw_window_handle::RawWindowHandle> for RawSurfaceHandle {
    type Error = ();
    fn try_into(self) -> Result<raw_window_handle::RawWindowHandle, Self::Error> {
        match self {
            #[cfg(unix)]
            Self::Xcb(handle) => Ok(raw_window_handle::RawWindowHandle::Xcb(handle)),
            #[cfg(unix)]
            Self::Wayland(handle) => Ok(raw_window_handle::RawWindowHandle::Wayland(handle)),
            Self::Vulkan => Err(())
        }
    }
}
/*
unsafe impl raw_window_handle::HasRawWindowHandle for RawSurfaceHandle {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        match self {
            #[cfg(unix)]
            Self::Xcb(handle) => raw_window_handle::RawWindowHandle::Xcb(*handle),
            #[cfg(unix)]
            Self::Wayland(handle) => raw_window_handle::RawWindowHandle::Wayland(*handle),
        }
    }
}*/
