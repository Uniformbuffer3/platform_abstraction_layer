#![allow(dead_code)]

pub mod definitions;
pub use definitions::{Event,OutputEventType,SurfaceEventType,SeatEventType};
mod backends;
pub use backends::Platform;
#[cfg(test)]
mod tests;
