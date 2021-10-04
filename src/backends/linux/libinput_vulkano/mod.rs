use crate::definitions::*;
use crate::backends::linux::input_backends::libinput::LibinputBackend;
use crate::backends::graphic_backends::vulkano::VulkanoBackend;

pub struct Window {}

pub struct LibinputVulkanoPlatform<S> {
    libinput_backend: LibinputBackend<S>,
    vulkano_backend: VulkanoBackend<S>
}


impl<S> GraphicBackend for LibinputVulkanoPlatform<S> {
    fn create_surface(
        &mut self,
        output: Option<crate::definitions::OutputId>,
    ) {
    }
}

impl<S> InputBackend<S> for LibinputVulkanoPlatform<S> {
    fn events(&mut self) -> Vec<crate::definitions::Event<S>> {
        Vec::new()
    }
    fn set_keyboard_layout(&mut self, layout: String) {

    }
}

