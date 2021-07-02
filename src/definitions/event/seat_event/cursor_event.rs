use keyboard_types::KeyState as State;

use crate::definitions::{SurfaceId,Position,Offset,CursorMode};

#[derive(Clone,Debug,PartialEq)]
pub enum CursorEvent {
    Added(CursorInfo),
    Removed,
    Button {
        key: Button,
        state: State
    },
    AbsoluteMovement{
        position: Position,
    },
    RelativeMovement {
        offset: Offset
    },
    Entered {
        surface_id: SurfaceId,
        position: Position,
    },
    Left {
        surface_id: SurfaceId,
    },
    Axis {
        value: Offset,
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

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct CursorInfo {
    pub mode: CursorMode,
    pub visible: bool
}
