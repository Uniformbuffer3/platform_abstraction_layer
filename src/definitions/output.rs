#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub struct OutputId(u32);
impl Into<u32> for OutputId {
    fn into(self) -> u32 {
        self.0
    }
}
impl From<u32> for OutputId {
    fn from(hash: u32) -> Self {
        Self(hash)
    }
}
impl Into<usize> for OutputId {
    fn into(self) -> usize {
        self.0 as usize
    }
}
impl From<usize> for OutputId {
    fn from(id: usize) -> Self {
        Self(id as u32)
    }
}
impl Eq for OutputId {}

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

