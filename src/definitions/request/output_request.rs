use crate::definitions::OutputId;

/// Possible output requests.
pub struct OutputRequest {
    pub id: OutputId,
    pub event: OutputRequestType,
}

/// Output request type.
pub enum OutputRequestType {

}
