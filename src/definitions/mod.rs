//! Definitions of structures and enumerations used by the crate.

mod event;
pub use event::*;

mod request;
pub use request::*;

mod common;
pub use common::*;

mod errors;
pub use errors::*;

mod external_context;
pub use external_context::*;

/**
Trait required to be implemented to be a valid platform.
*/
pub trait PlatformBackend {
    /**
    Rquest the platform type.
    */
    fn platform_type(&self)->PlatformType;
    /**
    Retrieve the events.
    */
    fn events(&mut self) -> Vec<Event>;
    /**
    Send requests.
    */
    fn requests(&mut self, requests: Vec<Request>);
}

#[cfg(target_os = "linux")]
/**
Linux platforms are required to also implement AsRawFd to get the underlying polling fd.
*/
pub trait LinuxPlatformBackend: PlatformBackend + std::os::unix::io::AsRawFd {}

#[derive(Debug,Clone, Copy, PartialEq)]
/**
Enumeration representing the different types of platform.
*/
pub enum PlatformType {
    Compositor,
    Direct
}
