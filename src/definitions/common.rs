
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum CursorMode {
    Absolute,
    Relative
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Position{pub x: u32,pub y: u32}
impl From<(u32,u32)> for Position {
    fn from(position: (u32,u32))->Self {Self{x: position.0,y: position.1}}
}
impl From<[u32; 2]> for Position {
    fn from(size: [u32; 2])->Self {Self{x: size[0],y: size[1]}}
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Offset{pub x: f32,pub y: f32}
impl From<(f32,f32)> for Offset {
    fn from(offset: (f32,f32))->Self {Self{x: offset.0,y: offset.1}}
}
impl From<[f32; 2]> for Offset {
    fn from(size: [f32; 2])->Self {Self{x: size[0],y: size[1]}}
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Size{pub width: u32,pub height: u32}
impl From<(u32,u32)> for Size {
    fn from(size: (u32,u32))->Self {Self{width: size.0,height: size.1}}
}
impl From<[u32; 2]> for Size {
    fn from(size: [u32; 2])->Self {Self{width: size[0],height: size[1]}}
}


#[derive(Debug, Clone,PartialEq)]
pub enum Transform {
    Normal,
    _90,
    _180,
    _270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}


#[derive(Debug, Clone,PartialEq)]
pub struct Mode {
    pub resolution: Size,
    pub refresh_rate: u32,
    pub is_preferred: bool,
}

#[derive(Debug, Clone,PartialEq)]
pub enum Subpixel {
    Unknown,
    None,
    HorizontalRgb,
    HorizontalBgr,
    VerticalRgb,
    VerticalBgr,
}
