use keyboard_types::KeyState as State;

use crate::definitions::{SurfaceId,Position2D,Offset2D,CursorMode};

#[derive(Clone,Debug,PartialEq)]
/// Possible cursor events.
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
        source: AxisSource,
        direction: AxisDirection,
        value: AxisValue,
    },
    ModeChanged(CursorMode),
    VisibilityChanged(bool)
}



#[derive(Clone,Copy,Debug,PartialEq)]
/// Source of the axis event.
pub enum AxisSource {
    Wheel
}

#[derive(Clone,Copy,Debug,PartialEq)]
/// Direction of the axis event.
pub enum AxisDirection {
    Horizontal,
    Vertical
}

#[derive(Clone,Copy,Debug,PartialEq)]
/// Value of the axis event.
pub enum AxisValue {
    Discrete(i32),
    Continuous(f32)
}

#[derive(Clone,Copy,Debug,PartialEq)]
/// Possible buttons for button events.
pub enum Button {
    Left,
    Right,
    Middle,
}

#[derive(Clone,Debug,PartialEq)]
/// Image kind for the cursor.
pub enum CursorImage {
    Custom(Vec<u8>),
    Default,
    Hidden
}

#[derive(Clone,Debug,PartialEq)]
/// Cursor informations.
pub struct CursorInfo {
    pub mode: CursorMode,
    pub theme: CursorImage,
    pub visible: bool
}
