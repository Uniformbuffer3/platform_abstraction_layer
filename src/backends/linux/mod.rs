mod graphic_backends;
mod input_backends;

#[cfg(feature = "wayland_platform")]
mod wayland;
#[cfg(feature = "wayland_platform")]
use wayland::WaylandPlatform;

#[cfg(feature = "xcb_platform")]
mod xcb;
#[cfg(feature = "xcb_platform")]
use self::xcb::XcbPlatform;

/*
mod libinput;
use libinput::LibinputPlatform;
*/

mod common;
pub use common::*;

pub enum LinuxPlatform {
    #[cfg(feature = "wayland_platform")]
    Wayland(WaylandPlatform),
    #[cfg(feature = "xcb_platform")]
    Xcb(XcbPlatform),
}
impl LinuxPlatform {
    pub fn new(contexts: Vec<crate::definitions::ExternalContext>) -> Option<Self> {
        #[cfg(feature = "wayland_platform")]
        match WaylandPlatform::new() {
            Ok(platform) => return Some(Self::Wayland(platform)),
            Err(_err) => (), //info!("Failed to init wayland platform: {:#?}",err)
        }

        #[cfg(feature = "xcb_platform")]
        match XcbPlatform::new(contexts) {
            Ok(platform) => return Some(Self::Xcb(platform)),
            Err(_err) => (), //info!("Failed to init wayland platform: {:#?}",err)
        }
        None
    }
}
impl crate::definitions::InputBackend for LinuxPlatform {
    fn dispatch(&mut self) -> Vec<crate::definitions::Event> {
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.dispatch(),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(platform) => platform.dispatch(),
        }
    }
    fn keyboard_layout(&self) -> String {
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.keyboard_layout(),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(_platform) => unimplemented!(),
        }
    }
    fn set_keyboard_layout(&mut self, layout: String) {
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.set_keyboard_layout(layout),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(_platform) => unimplemented!(),
        }
    }
}

impl crate::definitions::GraphicBackend for LinuxPlatform {
    fn create_surface(
        &mut self,
        output: Option<crate::definitions::OutputId>,
    ){
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.create_surface(output),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(platform) => platform.create_surface(output),
        }
    }
    fn raw_surface_handle(&self, surface: crate::definitions::SurfaceId) -> crate::definitions::RawSurfaceHandle {
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.raw_surface_handle(surface),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(platform) => platform.raw_surface_handle(surface),
        }
    }
    /*
    fn list_outputs(&self) -> Vec<crate::Output> {
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.list_outputs(),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(platform) => platform.list_outputs(),
        }
    }
    */
}

