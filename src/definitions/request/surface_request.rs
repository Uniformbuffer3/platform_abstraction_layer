use crate::definitions::{SurfaceId,OutputId};

/// Possible surface requests.
pub enum SurfaceRequest {
    Create(Option<OutputId>),
    Destroy(SurfaceId),

    Commit(SurfaceId)
}
