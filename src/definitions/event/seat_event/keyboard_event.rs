pub use keyboard_types::{Code as Key,KeyState as State};

#[derive(Clone,Debug,PartialEq)]
pub enum KeyboardEvent {
    Added(KeyboardInfo),
    Removed,
    Key{
        code: u32,
        key: Option<Key>,
        state: State
    },
    AutoRepeat(bool),
    LayoutModified{layout: String}
}

#[derive(Clone,Debug,PartialEq)]
pub struct KeyboardInfo {
    pub layout: String,
    pub autorepeat: bool,
    pub encoding: KeyEncoding
}

#[derive(Clone,Debug,PartialEq)]
pub enum KeyEncoding {
    XkbV1
}

/*
#[derive(Debug,PartialEq)]
pub struct KeyboardEvent {
    pub key: Key,
    pub state: State,
}*/
