#![allow(dead_code)]

pub mod definitions;
pub use definitions::{PlatformBackend,PlatformType,Event,OutputEventType,SurfaceEventType,SeatEventType,ExternalContext};
mod backends;
pub use backends::Platform;
#[cfg(test)]
mod tests;
