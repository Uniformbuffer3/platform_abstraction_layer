pub mod output;
pub use output::*;

pub mod seat;
pub use seat::*;

pub mod surface;
pub use surface::*;

mod event;
pub use event::*;

pub trait GraphicInputBackend: GraphicBackend + InputBackend {}

pub trait InputBackend {
    fn dispatch(&mut self) -> Vec<Event>;
    fn keyboard_layout(&self) -> String;
    fn set_keyboard_layout(&mut self, layout: String);
}

pub trait GraphicBackend {
    fn create_surface(&mut self, output: Option<OutputId>);
    fn raw_surface_handle(&self, surface: SurfaceId) -> RawSurfaceHandle;

}

#[derive(Debug)]
pub enum CreateSurfaceError {
    TooManySurfaces,
    InvalidOutputId,
}

pub enum ExternalContext<'a> {
    #[cfg(feature = "wgpu_interaction")]
    WGpu { instance: &'a wgpu::Instance },
    #[cfg(not(feature = "external_context"))]
    None(std::marker::PhantomData<&'a u32>),
}

pub enum RawSurfaceHandle {
    #[cfg(unix)]
    Xcb(raw_window_handle::unix::XcbHandle),
    #[cfg(unix)]
    Wayland(raw_window_handle::unix::WaylandHandle),
}
impl std::convert::TryInto<raw_window_handle::RawWindowHandle> for RawSurfaceHandle {
    type Error = ();
    fn try_into(self) -> Result<raw_window_handle::RawWindowHandle, Self::Error> {
        match self {
            #[cfg(unix)]
            Self::Xcb(handle) => Ok(raw_window_handle::RawWindowHandle::Xcb(handle)),
            #[cfg(unix)]
            Self::Wayland(handle) => Ok(raw_window_handle::RawWindowHandle::Wayland(handle)),
        }
    }
}

unsafe impl raw_window_handle::HasRawWindowHandle for RawSurfaceHandle {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        match self {
            #[cfg(unix)]
            Self::Xcb(handle) => raw_window_handle::RawWindowHandle::Xcb(*handle),
            #[cfg(unix)]
            Self::Wayland(handle) => raw_window_handle::RawWindowHandle::Wayland(*handle),
        }
    }
}
