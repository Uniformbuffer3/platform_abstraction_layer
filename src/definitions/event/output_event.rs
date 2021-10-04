use crate::definitions::{Position,Size,Mode,Subpixel};

#[derive(Clone,Debug,PartialEq)]
pub struct OutputEvent {
    pub id: OutputId,
    pub event_type: OutputEventType
}
impl From<(OutputId,OutputEventType)> for OutputEvent {
    fn from(tuple: (OutputId,OutputEventType))->Self {Self{id: tuple.0,event_type: tuple.1}}
}

#[derive(Clone,Debug,PartialEq)]
pub enum OutputEventType {
    Added(OutputInfo),
    ModeAdded(Mode),
    ModeChanged(Mode),
    PositionChanged(Position),
    Removed
}

#[derive(Debug, PartialEq, Hash, Copy, Clone,Eq,Ord,PartialOrd)]
pub struct OutputId(u32);
impl Into<u32> for OutputId {
    fn into(self) -> u32 {
        self.0
    }
}
impl From<u32> for OutputId {
    fn from(hash: u32) -> Self {
        Self(hash)
    }
}
impl Into<usize> for OutputId {
    fn into(self) -> usize {
        self.0 as usize
    }
}
impl From<usize> for OutputId {
    fn from(id: usize) -> Self {
        Self(id as u32)
    }
}
impl std::fmt::Display for OutputId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Clone,PartialEq)]
pub struct OutputInfo {
    pub position: Position,
    pub selected_mode: Mode,
    pub available_modes: Vec<Mode>,
    pub physical_size: Size,
    pub subpixel: Subpixel,
}
