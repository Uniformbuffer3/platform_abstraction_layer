
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

#[derive(Debug, PartialEq, Hash, Copy, Clone)]
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
impl Eq for OutputId {}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Position{pub x: u32,pub y: u32}
impl From<(u32,u32)> for Position {
    fn from(position: (u32,u32))->Self {Self{x: position.0,y: position.1}}
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Offset{pub x: f32,pub y: f32}
impl From<(f32,f32)> for Offset {
    fn from(offset: (f32,f32))->Self {Self{x: offset.0,y: offset.1}}
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Size{pub width: u32,pub height: u32}
impl From<(u32,u32)> for Size {
    fn from(size: (u32,u32))->Self {Self{width: size.0,height: size.1}}
}

#[derive(Debug, Clone,PartialEq)]
pub enum Transform {
    Normal,
    _90,
    _180,
    _270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}


#[derive(Debug, Clone,PartialEq)]
pub struct OutputInfo {
    pub position: Position,
    pub selected_mode: Mode,
    pub available_modes: Vec<Mode>,
    pub physical_size: Size,
    pub subpixel: Subpixel,
}

#[derive(Debug, Clone,PartialEq)]
pub struct Mode {
    pub resolution: Size,
    pub refresh_rate: u32,
    pub is_preferred: bool,
}

#[derive(Debug, Clone,PartialEq)]
pub enum Subpixel {
    Unknown,
    None,
    HorizontalRgb,
    HorizontalBgr,
    VerticalRgb,
    VerticalBgr,
}
