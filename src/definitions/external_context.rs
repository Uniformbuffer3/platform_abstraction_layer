
use raw_window_handle::RawWindowHandle;


use crate::definitions::Surface;
use std::sync::Arc;

/*
#[derive(Debug)]
pub enum ExternalContext {
    Raw(RawContext)
}*/

/// Trait providing the capability to create surfaces.
pub trait ExternalContext {
    fn create_surface(&self,raw_surface: &RawSurfaceHandle)->Result<Surface,()>;
}

#[derive(Debug)]
/// Raw context to create surfaces.
pub struct RawContext;
impl ExternalContext for RawContext {
    fn create_surface(&self,raw_surface: &RawSurfaceHandle)->Result<Surface,()> {
        match raw_surface {
            RawSurfaceHandle::Xcb(xcb_handle)=>Ok(Surface::Raw(RawWindowHandle::Xcb(*xcb_handle))),
            RawSurfaceHandle::Wayland(wayland_handle)=>Ok(Surface::Raw(RawWindowHandle::Wayland(*wayland_handle))),
            #[cfg(feature="wgpu")]
            RawSurfaceHandle::WGpuDisplay(_display)=>Err(())
        }
    }
}


#[cfg(feature="wgpu_backend")]
#[derive(Debug)]
/// Context to create WGpu surfaces.
pub struct WgpuContext{
    pub instance: Arc<crate::wgpu::Instance>,
    pub devices: Vec<Arc<(crate::wgpu::Adapter,crate::wgpu::Device, crate::wgpu::Queue)>>
}

#[cfg(feature="wgpu_backend")]
impl ExternalContext for WgpuContext {
    fn create_surface(&self,raw_surface: &RawSurfaceHandle)->Result<Surface,()> {
        match raw_surface {
            RawSurfaceHandle::Xcb(xcb_handle)=>{
                let handle: Window = RawWindowHandle::Xcb(*xcb_handle).into();
                let surface = unsafe{self.instance.create_surface(&handle)};
                return Ok(Surface::WGpu(Arc::new(surface)));
            },
            RawSurfaceHandle::Wayland(wayland_handle)=>{
                let handle: Window = RawWindowHandle::Wayland(*wayland_handle).into();
                let surface = unsafe{self.instance.create_surface(&handle)};
                return Ok(Surface::WGpu(Arc::new(surface)));
            },
            #[cfg(feature="wgpu")]
            RawSurfaceHandle::WGpuDisplay(_display)=>return Err(())
        }
    }
}


/// Display identifier.
pub type Display = u32;
#[derive(Debug)]
/// Supported surface handles.
pub enum RawSurfaceHandle {
    #[cfg(unix)]
    Xcb(raw_window_handle::unix::XcbHandle),
    #[cfg(unix)]
    Wayland(raw_window_handle::unix::WaylandHandle),
    #[cfg(feature="wgpu")]
    WGpuDisplay(Display)
}



struct Window(RawWindowHandle);
unsafe impl raw_window_handle::HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {self.0}
}
impl From<RawWindowHandle> for Window {
    fn from(raw: RawWindowHandle)->Self {Self(raw)}
}







/*
pub enum SurfaceContext {
    #[cfg(feature = "vulkano")]
    Vulkano{devices: Vec<vulkano::device::Device>},
    Raw
}

pub trait ExternalContext<S> {
    fn create_surface(&self,raw_surface_handle: RawSurfaceHandle)->S;
    fn surface_context(&self)->&SurfaceContext;
}

pub struct Raw {
    context: SurfaceContext
}
impl Raw {
    pub fn new()->Self {
        let context = SurfaceContext::Raw;
        Self{context}
    }
}

impl ExternalContext<RawSurfaceHandle> for Raw {
    fn create_surface(&self,raw_surface_handle: RawSurfaceHandle)->RawSurfaceHandle{raw_surface_handle}
    fn surface_context(&self)->&SurfaceContext {&self.context}
}*/
/*
pub struct Vulkano {
    context: SurfaceContext<'a>
}*/
