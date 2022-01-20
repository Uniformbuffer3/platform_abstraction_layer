
#[derive(Debug,Clone,Copy,PartialEq)]
/// Possible error while creating surface.
pub enum SurfaceError {
    TooManySurfaces,
    Unsupported
}

#[derive(Debug,Clone,Copy,PartialEq)]
/// Possible error while setting cursor mode.
pub enum CursorModeError {
    Unsupported
}

#[derive(Debug,Clone,Copy,PartialEq)]
/// Possible error while setting key repeat.
pub enum KeyRepeatError {
    Unsupported
}

#[derive(Debug,Clone,Copy,PartialEq)]
/// Possible error while setting keyboard layout.
pub enum KeyboardLayoutError {
    Unsupported
}
