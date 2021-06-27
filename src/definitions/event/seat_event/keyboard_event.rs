pub use keyboard_types::{Code as Key,KeyState as State};

#[derive(Clone,Debug,PartialEq)]
pub enum KeyboardEvent {
    Added(KeyboardInfo),
    Removed,
    Key{
        key: Key,
        state: State
    },
    AutoRepeat(bool),
    LayoutModified{layout: String}
}

#[derive(Clone,Debug,PartialEq)]
pub struct KeyboardInfo {
    layout: String,
    autorepeat: bool
}

/*
#[derive(Debug,PartialEq)]
pub struct KeyboardEvent {
    pub key: Key,
    pub state: State,
}*/
