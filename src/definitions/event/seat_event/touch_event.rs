#[derive(Clone,Debug,PartialEq)]
pub enum TouchEvent {
    Added(TouchInfo),
    Removed,
}

#[derive(Clone,Debug,PartialEq)]
pub struct TouchInfo {

}
