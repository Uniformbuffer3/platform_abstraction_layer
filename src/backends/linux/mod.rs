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

#[cfg(feature = "libinput_wgpu_platform")]
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
    pub fn new(external_contexts: Vec<Box<dyn ExternalContext>>) -> Option<Self> {
        #[cfg(feature = "wayland_platform")]
        match WaylandPlatform::new() {
            Ok(platform) => return Some(Self::Wayland(platform)),
            Err(err) => info!("Failed to init wayland platform: {:#?}",err),
        }

        #[cfg(feature = "xcb_platform")]
        match XcbPlatform::new(external_contexts) {
            Ok(platform) => return Some(Self::Xcb(platform)),
            Err(err) => info!("Failed to init wayland platform: {:#?}",err),
        }
        None
    }
}

#[cfg(target_os = "linux")]
impl std::os::unix::io::AsRawFd for LinuxPlatform {
    fn as_raw_fd(&self)->std::os::unix::io::RawFd {
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.as_raw_fd(),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(platform) => platform.as_raw_fd(),
        }
    }
}

impl crate::definitions::PlatformBackend for LinuxPlatform {
    fn platform_type(&self)->PlatformType {PlatformType::Compositor}
    fn events(&mut self) -> Vec<crate::definitions::Event> {
        match self {
            #[cfg(feature = "wayland_platform")]
            Self::Wayland(platform) => platform.events(),
            #[cfg(feature = "xcb_platform")]
            Self::Xcb(platform) => platform.events(),
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


