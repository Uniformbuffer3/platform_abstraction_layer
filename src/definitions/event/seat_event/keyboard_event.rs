pub use keyboard_types::{Code as Key,KeyState as State};

#[derive(Clone,Debug,PartialEq)]
/// Possible keyboard events.
pub enum KeyboardEvent {
    Added(KeyboardInfo),
    Removed,
    Key{
        code: u32,
        key: Option<Key>,
        state: State,
        serial: u32,
        time: u32
    },
    AutoRepeat{
        rate: u32,
        delay: u32
    },
    LayoutModified{layout: String}
}

#[derive(Clone,Debug,PartialEq)]
/// Keyboard informations.
pub struct KeyboardInfo {
    pub layout: String,
    pub autorepeat: bool,
    pub encoding: KeyEncoding
}

#[derive(Clone,Debug,PartialEq)]
/// Keystroke encoding.
pub enum KeyEncoding {
    XkbV1
}

/*
#[derive(Debug,PartialEq)]
pub struct KeyboardEvent {
    pub key: Key,
    pub state: State,
}*/
