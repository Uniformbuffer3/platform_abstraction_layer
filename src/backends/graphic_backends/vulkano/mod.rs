use vulkano::swapchain::Surface;
use crate::definitions::GraphicBackend;
use crate::contexts::{SurfaceContext,ExternalContext};

pub struct VulkanoBackend<S> {
    context: Box<dyn ExternalContext<S>>
}
impl<S> VulkanoBackend<S> {
    pub fn new(context: impl ExternalContext<S> + 'static) -> Result<Self, ()> {
        let context = Box::new(context);
        Ok(Self {context})
    }
}


impl<S> GraphicBackend for VulkanoBackend<S> {
    fn create_surface(
        &mut self,
        output: Option<crate::definitions::OutputId>,
    ) {
    }
}

/*
impl SurfaceBackend for VulkanoBackend {
    type Surface = Surface<Window>;

    fn create_surface(&self)->Self::Surface {

    }
}*/
