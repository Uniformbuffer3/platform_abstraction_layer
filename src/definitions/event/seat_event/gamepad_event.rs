#[derive(Clone,Debug,PartialEq)]
/// Possible gamepad events.
pub enum GamepadEvent {
    Added(GamepadInfo),
    Removed,
}

#[derive(Clone,Debug,PartialEq)]
/// Gamepad informations.
pub struct GamepadInfo {

}
