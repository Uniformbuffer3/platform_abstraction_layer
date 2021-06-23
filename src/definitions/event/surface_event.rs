use raw_window_handle::RawWindowHandle;
use super::{Position,Size};

#[derive(Clone,Debug,PartialEq)]
pub struct SurfaceEvent {
    pub id: SurfaceId,
    pub event_type: SurfaceEventType,
}
impl From<(SurfaceId,SurfaceEventType)> for SurfaceEvent {
    fn from(tuple: (SurfaceId,SurfaceEventType))->Self {Self{id: tuple.0,event_type: tuple.1}}
}

#[derive(Clone,Debug,PartialEq)]
pub enum SurfaceEventType {
    Added(SurfaceInfo),
    Moved(Position),
    Resized(Size),
    Focused(bool),
    ModeChanged(Option<SurfaceMode>),
    TilingChanged(Option<SurfaceTiling>),
    Removed,
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceInfo {
    pub position: Position,
    pub size: Size,
    pub surface: Surface
}

#[derive(Debug, Clone, PartialEq)]
pub enum Surface {
    Raw(RawWindowHandle)
}

/*
impl<S> std::fmt::Debug for SurfaceInfo<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SurfaceInfo")
         .field("x", &self.x)
         .field("y", &self.y)
         .field("width", &self.width)
         .field("height", &self.height)
         .field("surface", &"Surface")
         .finish()
    }
}
*/

bitflags::bitflags! {
    pub struct SurfaceMode: u32 {
        const MINIMIZED = (1 << 0);
        const MAXIMIZED = (1 << 1);
        const FULLSCREEN = (1 << 2);
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum SurfaceTiling {
    TiledLeft,
    TiledRight,
    TiledTop,
    TiledBottom,
}

#[derive(Clone,Debug,PartialEq)]
pub struct SurfaceCapabilities {
    supported_modes: Vec<SurfaceMode>,
    can_resize: bool,
    can_move: bool,
    is_monitor_indipendent: bool,
}
