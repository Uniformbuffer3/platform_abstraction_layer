use keyboard_types::{Code as Key, KeyState as State};

#[derive(Debug,PartialEq)]
pub struct KeyboardEvent {
    pub key: Key,
    pub state: State,
}
