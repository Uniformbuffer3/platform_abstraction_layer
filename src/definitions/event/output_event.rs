use crate::definitions::{Position2D,Size2D,Mode,Subpixel};

#[derive(Clone,Debug,PartialEq)]
pub enum OutputEvent {
    Added(OutputInfo),
    ModeAdded(Mode),
    ModeChanged(Mode),
    Moved(Position2D<u32>),
    Removed
}

#[derive(Debug, PartialEq, Hash, Copy, Clone,Eq,Ord,PartialOrd)]
pub struct OutputId(usize);
impl Into<usize> for OutputId {
    fn into(self) -> usize {
        self.0
    }
}
impl From<usize> for OutputId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}
impl From<u32> for OutputId {
    fn from(id: u32) -> Self {
        Self(id as usize)
    }
}
impl From<i32> for OutputId {
    fn from(id: i32) -> Self {
        Self(id as usize)
    }
}
impl std::fmt::Display for OutputId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Clone,PartialEq)]
pub struct OutputInfo {
    pub position: Position2D<u32>,
    pub selected_mode: Mode,
    pub available_modes: Vec<Mode>,
    pub physical_size: Size2D<u32>,
    pub subpixel: Subpixel,
}
