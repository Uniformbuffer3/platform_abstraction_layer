use keyboard_types::KeyState as State;

use crate::definitions::{SurfaceId,Position2D,Offset2D,CursorMode};

#[derive(Clone,Debug,PartialEq)]
pub enum CursorEvent {
    Added(CursorInfo),
    Removed,
    Button {
        code: u32,
        key: Option<Button>,
        state: State
    },
    AbsoluteMovement{
        position: Position2D<i32>,
    },
    RelativeMovement {
        offset: Offset2D<f32>
    },
    Entered {
        surface_id: SurfaceId,
        position: Position2D<i32>,
    },
    Left {
        surface_id: SurfaceId,
    },
    Axis {
        value: Offset2D<f32>,
    },
    ModeChanged(CursorMode),
    VisibilityChanged(bool)
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Button {
    Left,
    Right,
    Middle,
}

#[derive(Clone,Debug,PartialEq)]
pub enum CursorImage {
    Custom(Vec<u8>),
    Default,
    Hidden
}

#[derive(Clone,Debug,PartialEq)]
pub struct CursorInfo {
    pub mode: CursorMode,
    pub theme: CursorImage,
    pub visible: bool
}
