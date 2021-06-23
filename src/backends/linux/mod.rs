pub mod graphic_backends;
pub mod input_backends;

#[cfg(feature = "wayland_platform")]
mod wayland;
#[cfg(feature = "wayland_platform")]
use wayland::WaylandPlatform;

#[cfg(feature = "xcb_platform")]
mod xcb;
#[cfg(feature = "xcb_platform")]
use xcb::XcbPlatform;

#[cfg(feature = "libinput_vulkano_platform")]
mod libinput_vulkano;
//#[cfg(feature = "libinput_vulkano_platform")]
//use libinput_vulkano::LibinputVulkanoPlatform;

mod common;
pub use common::*;

use crate::definitions::ExternalContext;
use log::*;

pub enum LinuxPlatform {
    #[cfg(feature = "wayland_platform")]
    Wayland(WaylandPlatform),
    #[cfg(feature = "xcb_platform")]
    Xcb(XcbPlatform),
}
impl LinuxPlatform {
    pub fn new(context: ExternalContext) -> Option<Self> {
        #[cfg(feature = "wayland_platform")]
        match WaylandPlatform::new() {
            Ok(platform) => return Some(Self::Wayland(platform)),
            Err(err) => info!("Failed to init wayland platform: {:#?}",err),
        }

        #[cfg(feature = "xcb_platform")]
        match XcbPlatform::new() {
            Ok(platform) => return Some(Self::Xcb(platform)),
            Err(err) => info!("Failed to init wayland platform: {:#?}",err),
        }
        None
    }
}

impl crate::definitions::SeatBackend for LinuxPlatform {
    fn set_keyboard_layout(&mut self, layout: String){
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.set_keyboard_layout(layout),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(_platform) => unimplemented!(),
        }
    }
}

impl crate::definitions::OutputBackend for LinuxPlatform {
}

impl crate::definitions::SurfaceBackend for LinuxPlatform {
    fn create_surface(&mut self, output: Option<crate::definitions::OutputId>){
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.create_surface(output),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(platform) => platform.create_surface(output),
        }
    }
}

impl crate::definitions::PlatformBackend for LinuxPlatform {
    fn dispatch(&mut self) -> Vec<crate::definitions::Event> {
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.dispatch(),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(platform) => platform.dispatch(),
        }
    }
}

