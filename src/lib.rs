#![allow(dead_code)]

pub mod definitions;
pub use definitions::{Event,ExternalContext};
mod backends;
pub use backends::Platform;

#[cfg(test)]
mod tests;
