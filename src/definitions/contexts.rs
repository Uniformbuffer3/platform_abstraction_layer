
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
