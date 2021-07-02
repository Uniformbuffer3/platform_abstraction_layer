mod event;
pub use event::*;

mod request;
pub use request::*;

mod common;
pub use common::*;

mod errors;
pub use errors::*;

pub trait PlatformBackend {
    fn platform_type(&self)->PlatformType;
    fn dispatch(&mut self) -> Vec<Event>;
    fn request(&mut self, requests: Vec<Request>);
}

pub enum PlatformType {
    Compositor,
    Direct
}

#[derive(Debug)]
pub enum ExternalContext {
    Raw
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
