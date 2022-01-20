#[derive(Clone,Debug,PartialEq)]
/// Possible touch events.
pub enum TouchEvent {
    Added(TouchInfo),
    Removed,
}

#[derive(Clone,Debug,PartialEq)]
/// Touch informations.
pub struct TouchInfo {

}
