use crate::definitions::OutputId;

pub struct OutputRequest {
    pub id: OutputId,
    pub event: OutputRequestType,
}

pub enum OutputRequestType {

}
