use raw_window_handle::RawWindowHandle;

use crate::definitions::{Position,Size,OutputId};
use std::sync::Arc;

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
    Removed,

    Entered(OutputId),
    Left(OutputId),

    Moved(Position),
    Resized(Size),
    Focused(bool),
    ModeChanged(SurfaceMode)
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
impl From<SurfaceId> for u32 {
    fn from(id: SurfaceId) -> Self {
        id.0
    }
}
impl Eq for SurfaceId {}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceInfo {
    pub position: Position,
    pub size: Size,
    pub surface: Surface
}

#[derive(Debug, Clone)]
pub enum Surface {
    Raw(RawWindowHandle),
    #[cfg(feature="wgpu_backend")]
    WGpu(Arc<wgpu::Surface>)
}
impl PartialEq for Surface {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Self::Raw(raw1),Self::Raw(raw2))=>raw1 == raw2,
            (Self::WGpu(raw1),Self::WGpu(raw2))=>Arc::ptr_eq(raw1,raw2),
            _=>false
        }
    }
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
        const TILING_LEFT = (1 << 3);
        const TILING_RIGHT = (1 << 4);
        const TILING_TOP = (1 << 5);
        const TILING_BOTTOM = (1 << 6);
    }
}
