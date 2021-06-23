use keyboard_types::{Code as Key};

#[derive(Clone,Debug,PartialEq)]
pub enum KeyboardEvent {
    KeyPress{key: Key},
    KeyRelease{key: Key},
    LayoutModified{layout: String}
}
/*
#[derive(Debug,PartialEq)]
pub struct KeyboardEvent {
    pub key: Key,
    pub state: State,
}*/
