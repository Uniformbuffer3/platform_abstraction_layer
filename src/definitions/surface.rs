#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub struct SurfaceId(u32);
impl SurfaceId {
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }
    pub fn id(&self) -> u32 {
        self.0
    }
}
impl From<u32> for SurfaceId {
    fn from(hash: u32) -> Self {
        Self(hash)
    }
}
impl Eq for SurfaceId {}


bitflags::bitflags! {
    pub struct SurfaceMode: u32 {
        const MINIMIZED = (1 << 0);
        const MAXIMIZED = (1 << 1);
        const FULLSCREEN = (1 << 2);
    }
}

#[derive(Debug,PartialEq)]
pub enum SurfaceTiling {
    TiledLeft,
    TiledRight,
    TiledTop,
    TiledBottom,
}

pub struct SurfaceCapabilities {
    supported_modes: Vec<SurfaceMode>,
    can_resize: bool,
    can_move: bool,
    is_monitor_indipendent: bool,
}
