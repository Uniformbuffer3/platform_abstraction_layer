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

pub trait PlatformBackend {
    fn platform_type(&self)->PlatformType;
    fn events(&mut self) -> Vec<Event>;
    fn request(&mut self, requests: Vec<Request>);
}

pub enum PlatformType {
    Compositor,
    Direct
}

