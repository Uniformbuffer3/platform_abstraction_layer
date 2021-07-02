use crate::definitions::OutputId;

pub struct OutputRequest {
    pub id: OutputId,
    pub event_type: OutputRequestType,
}

pub enum OutputRequestType {

}
