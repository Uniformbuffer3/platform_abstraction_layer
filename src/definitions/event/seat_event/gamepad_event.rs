#[derive(Clone,Debug,PartialEq)]
pub enum GamepadEvent {
    Added(GamepadInfo),
    Removed,
}

#[derive(Clone,Debug,PartialEq)]
pub struct GamepadInfo {

}
