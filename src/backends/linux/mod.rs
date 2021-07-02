pub mod partial_backends;

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

mod libinput_wgpu;

mod common;
pub use common::*;

use crate::definitions::*;
use log::*;

pub enum LinuxPlatform {
    #[cfg(feature = "wayland_platform")]
    Wayland(WaylandPlatform),
    #[cfg(feature = "xcb_platform")]
    Xcb(XcbPlatform),
}
impl LinuxPlatform {
    pub fn new(_context: ExternalContext) -> Option<Self> {
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

impl crate::definitions::PlatformBackend for LinuxPlatform {
    fn platform_type(&self)->PlatformType {PlatformType::Compositor}
    fn dispatch(&mut self) -> Vec<crate::definitions::Event> {
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.dispatch(),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(platform) => platform.dispatch(),
        }
    }
    fn request(&mut self, requests: Vec<Request>) {
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.request(requests),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(platform) => platform.request(requests),
        }
    }
}


