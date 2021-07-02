
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum SurfaceError {
    TooManySurfaces,
    Unsupported
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum CursorModeError {
    Unsupported
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum KeyRepeatError {
    Unsupported
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum KeyboardLayoutError {
    Unsupported
}
