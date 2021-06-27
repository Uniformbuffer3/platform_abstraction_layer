mod event;
pub use event::*;

pub trait PlatformBackend {
    fn dispatch(&mut self) -> Vec<Event>;
    fn set_keyboard_layout(&mut self, layout: String);
    fn set_key_repeat(&mut self, seat_id: SeatId, value: bool);
    fn set_cursor_mode(&mut self, seat_id: SeatId, mode: CursorMode);

    fn create_surface(&mut self, output: Option<OutputId>);
}

/*
pub trait SeatBackend {

}

pub trait OutputBackend {

}

pub trait SurfaceBackend {
    fn dispatch_surfaces(&mut self) -> Vec<SurfaceEvent> {Vec::new()}

}
*/
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
