use raw_window_handle::RawWindowHandle;

use crate::definitions::{Position2D,Size2D,OutputId};
use std::sync::Arc;

#[derive(Clone,Debug,PartialEq)]
pub enum SurfaceEvent {
    Added(SurfaceInfo),
    Removed,

    Entered(OutputId),
    Left(OutputId),

    //Moved(Position2D<u32>),
    Resized(Size2D<u32>),
    Focused(bool),
    ModeChanged(SurfaceMode)
}

#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub struct SurfaceId(usize);
impl Into<usize> for SurfaceId {
    fn into(self) -> usize {
        self.0
    }
}
impl From<usize> for SurfaceId {
    fn from(hash: usize) -> Self {
        Self(hash)
    }
}
impl From<u32> for SurfaceId {
    fn from(id: u32) -> Self {
        Self(id as usize)
    }
}
impl From<i32> for SurfaceId {
    fn from(id: i32) -> Self {
        Self(id as usize)
    }
}
impl Eq for SurfaceId {}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceInfo {
    pub position: Position2D<u32>,
    pub size: Size2D<u32>,
    pub surface: Surface
}

#[derive(Debug, Clone)]
pub enum Surface {
    Raw(RawWindowHandle),
    #[cfg(feature="wgpu_backend")]
    WGpu(Arc<crate::wgpu::Surface>)
}
impl PartialEq for Surface {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Self::Raw(raw1),Self::Raw(raw2))=>raw1 == raw2,
            #[cfg(feature="wgpu_backend")]
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
