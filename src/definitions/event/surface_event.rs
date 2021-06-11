/*
#[derive(Debug)]
pub struct SurfaceEvent {
    pub surface_id: SurfaceId,
    pub event: SurfaceEventInner,
}

impl From<(SurfaceId, SurfaceEventInner)> for SurfaceEvent {
    fn from(event: (SurfaceId, SurfaceEventInner)) -> Self {
        Self {
            surface_id: event.0,
            event: event.1,
        }
    }
}
*/
#[derive(Debug,PartialEq)]
pub enum SurfaceEvent {
    Added{
        x: u32,
        y: u32,
        width: u32,
        height: u32
    },
    Removed,
    Moved{
        x: u32,
        y: u32
    },
    Resized{
        width: u32,
        height: u32
    },
    Focused(bool),
    ChangedMode(Option<crate::definitions::SurfaceMode>),
    ChangedTiling(Option<crate::definitions::SurfaceTiling>),
    Destroyed,
}
