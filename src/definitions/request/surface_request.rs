use crate::definitions::{SurfaceId,OutputId};

pub enum SurfaceRequest {
    Create(Option<OutputId>),
    Destroy(SurfaceId),

    Commit(SurfaceId)
}
