use std::ops::{Add,Sub};
use std::cmp::PartialOrd;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum CursorMode {
    Absolute,
    Relative
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Position2D<T>{pub x: T,pub y: T}
impl<T> From<(T,T)> for Position2D<T> {
    fn from(position: (T,T))->Self {Self{x: position.0,y: position.1}}
}
impl<T> Into<(T,T)> for Position2D<T> {
    fn into(self)->(T,T) {(self.x,self.y)}
}
impl<T: Copy> From<[T; 2]> for Position2D<T> {
    fn from(position: [T; 2])->Self {Self{x: position[0],y: position[1]}}
}
impl<T> From<Position3D<T>> for Position2D<T> {
    fn from(position: Position3D<T>)->Self {Self{x: position.x,y: position.y}}
}
impl<T: Add<Output = T>> Add for Position2D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Sub<Output = T>> Sub for Position2D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Add<Output = T>> Add<Offset2D<T>> for Position2D<T> {
    type Output = Self;

    fn add(mut self, other: Offset2D<T>) -> Self::Output {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self
    }
}
impl<T: Sub<Output = T>> Sub<Offset2D<T>> for Position2D<T> {
    type Output = Self;

    fn sub(mut self, other: Offset2D<T>) -> Self::Output {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self
    }
}


#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Position3D<T>{pub x: T,pub y: T,pub z: T}
impl<T> From<(T,T,T)> for Position3D<T> {
    fn from(position: (T,T,T))->Self {Self{x: position.0,y: position.1,z: position.2}}
}
impl<T: Copy> From<[T; 3]> for Position3D<T> {
    fn from(position: [T; 3])->Self {Self{x: position[0],y: position[1],z: position[2]}}
}
impl<T: Copy> Into<[T; 3]> for Position3D<T> {
    fn into(self)->[T; 3] {[self.x,self.y,self.z]}
}
impl<T> From<(Position2D<T>,T)> for Position3D<T> {
    fn from(position: (Position2D<T>,T))->Self {Self{x: position.0.x,y: position.0.y,z: position.1}}
}

impl<T: Add<Output = T>> Add<Offset2D<T>> for Position3D<T> {
    type Output = Self;

    fn add(mut self, other: Offset2D<T>) -> Self::Output {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self
    }
}
impl<T: Sub<Output = T>> Sub<Offset2D<T>> for Position3D<T> {
    type Output = Self;

    fn sub(mut self, other: Offset2D<T>) -> Self::Output {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self
    }
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Offset2D<T>{pub x: T,pub y: T}
impl<T> From<(T,T)> for Offset2D<T> {
    fn from(offset: (T,T))->Self {Self{x: offset.0,y: offset.1}}
}
impl<T: Copy> From<[T; 2]> for Offset2D<T> {
    fn from(size: [T; 2])->Self {Self{x: size[0],y: size[1]}}
}

impl<T: Add<Output = T>> Add<Position2D<T>> for Offset2D<T> {
    type Output = Position2D<T>;

    fn add(self, mut other: Position2D<T>) -> Self::Output {
        other.x = other.x + self.x;
        other.y = other.y + self.y;
        other
    }
}
impl<T: Sub<Output = T>> Sub<Position2D<T>> for Offset2D<T> {
    type Output = Position2D<T>;

    fn sub(self, mut other: Position2D<T>) -> Self::Output {
        other.x = other.x - self.x;
        other.y = other.y - self.y;
        other
    }
}

impl<T: Add<Output = T>> Add<Position3D<T>> for Offset2D<T> {
    type Output = Position3D<T>;

    fn add(self, mut other: Position3D<T>) -> Self::Output {
        other.x = other.x + self.x;
        other.y = other.y + self.y;
        other
    }
}
impl<T: Sub<Output = T>> Sub<Position3D<T>> for Offset2D<T> {
    type Output = Position3D<T>;

    fn sub(self, mut other: Position3D<T>) -> Self::Output {
        other.x = other.x - self.x;
        other.y = other.y - self.y;
        other
    }
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Size2D<T>{pub width: T,pub height: T}
impl<T> From<(T,T)> for Size2D<T> {
    fn from(size: (T,T))->Self {Self{width: size.0,height: size.1}}
}
impl<T> From<Size2D<T>> for (T,T) {
    fn from(size: Size2D<T>)->Self {(size.width,size.height)}
}
impl<T: Copy> From<[T; 2]> for Size2D<T> {
    fn from(size: [T; 2])->Self {Self{width: size[0],height: size[1]}}
}
impl<T: Copy> Into<[T; 2]> for Size2D<T> {
    fn into(self)->[T; 2] {[self.width,self.height]}
}
impl<T: Add<Output = T>> Add<Self> for Size2D<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.width = self.width + other.width;
        self.height = self.height + other.height;
        self
    }
}
impl<T: Sub<Output = T>> Sub<Self> for Size2D<T> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self.width = self.width - other.width;
        self.height = self.height - other.height;
        self
    }
}
impl<T: Add<Output = T>> Add<Offset2D<T>> for Size2D<T> {
    type Output = Self;

    fn add(mut self, other: Offset2D<T>) -> Self::Output {
        self.width = self.width + other.x;
        self.height = self.height + other.y;
        self
    }
}
impl<T: Sub<Output = T>> Sub<Offset2D<T>> for Size2D<T> {
    type Output = Self;

    fn sub(mut self, other: Offset2D<T>) -> Self::Output {
        self.width = self.width - other.x;
        self.height = self.height - other.y;
        self
    }
}

#[derive(Debug,Clone, Copy, PartialEq)]
pub struct Rectangle<P,S> {
    pub position: Position2D<P>,
    pub size: Size2D<S>
}
impl<P: Ord,S: Ord> Rectangle<P,S> {
    pub fn bounding_box(rectangles: impl Iterator<Item=Rectangle<P,S>>)->Rectangle<P,S> {
        let mut x = None;
        let mut y = None;
        let mut width = None;
        let mut height = None;

        for rectangle in rectangles {
            match x {
                None=>{x = Some(rectangle.position.x);}
                Some(value)=>{x = Some(std::cmp::min(value,rectangle.position.x));}
            }
            match y {
                None=>{y = Some(rectangle.position.y);}
                Some(value)=>{y = Some(std::cmp::min(value,rectangle.position.y));}
            }

            match width {
                None=>{width = Some(rectangle.size.width);}
                Some(value)=>{width = Some(std::cmp::max(value,rectangle.size.width));}
            }
            match height {
                None=>{height = Some(rectangle.size.height);}
                Some(value)=>{height = Some(std::cmp::max(value,rectangle.size.height));}
            }
/*
            if y.is_none() {y = Some(rectangle.position.y);}
            else{y = Some(std::cmp::max(y,rectangle.position.y));}

            y = Some(std::cmp::max(y,rectangle.position.y));
            width = Some(std::cmp::min(width,rectangle.size.width));
            height = Some(std::cmp::min(height,rectangle.size.height));
            */
        }

        match (x,y,width,height) {
            (Some(x),Some(y),Some(width),Some(height))=>Rectangle::from((Position2D::from((x,y)),Size2D::from((width,height)))),
            _=>panic!()
        }

    }
}
impl<
    P: Copy + Add<Output=P>  + PartialOrd ,
    S: Copy+ std::convert::TryInto<P,Error = E>,
    E: std::fmt::Debug
> Rectangle<P,S> {
    pub fn contains(&self,position: Position2D<P>)->bool{
        position.x > self.position.x && position.x < self.position.x + self.size.width.try_into().unwrap() &&
        position.y > self.position.y && position.y < self.position.y + self.size.height.try_into().unwrap()
    }
    pub fn x_offset(&self)->P {self.position.x + self.size.width.try_into().unwrap()}
    pub fn y_offset(&self)->P {self.position.y + self.size.height.try_into().unwrap()}
}
impl<
    P: Copy + Add<Output=P> + Sub<Output=P> + PartialOrd ,
    S: Copy+ std::convert::TryInto<P,Error = E>,
    E: std::fmt::Debug
> Rectangle<P,S> {
    pub fn relative_to(&self, position: Position2D<P>)->Option<Position2D<P>>{
        if self.contains(position) {Some(position - self.position)}
        else{None}
    }
}
impl<P,S> From<(Position2D<P>,Size2D<S>)> for Rectangle<P,S> {
    fn from(tuple: (Position2D<P>,Size2D<S>)) -> Self {
        let position = tuple.0;
        let size = tuple.1;
        Self {position,size}
    }
}
impl<P,S> From<(Size2D<S>,Position2D<P>)> for Rectangle<P,S> {
    fn from(tuple: (Size2D<S>,Position2D<P>)) -> Self {
        Self::from((tuple.1,tuple.0))
    }
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
    pub resolution: Size2D<u32>,
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
