use keyboard_types::KeyState as State;

#[derive(Clone,Debug,PartialEq)]
pub enum Button {
    Left,
    Right,
    Middle,
}

#[derive(Clone,Debug,PartialEq)]
pub enum CursorEvent {
    CursorButton {
        key: Button,
        state: State
    },
    CursorMoved {
        position: (f64, f64),
    },
    CursorEntered {
        surface_id: crate::definitions::SurfaceId,
        position: (f64, f64),
    },
    CursorLeft {
        surface_id: crate::definitions::SurfaceId,
    },
    CursorAxis {
        value: (f64, f64),
    },
}
