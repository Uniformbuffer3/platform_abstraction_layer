#![allow(dead_code)]

pub mod definitions;
pub use definitions::*;
mod backends;
pub use backends::Platform;


#[cfg(feature = "wgpu_custom_backend")]
pub use wgpu_custom as wgpu;
#[cfg(feature = "wgpu_standard_backend")]
pub use wgpu_standard as wgpu;

#[cfg(test)]
mod tests;
