#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub struct SeatId(u32);
impl From<u32> for SeatId {
    fn from(hash: u32) -> Self {
        Self(hash)
    }
}
impl Eq for SeatId {}

#[derive(Debug,PartialEq)]
pub struct SeatInfo {
    pub id: SeatId,
    pub name: String,
    pub has_pointer: bool,
    pub has_keyboard: bool,
    pub has_touch: bool,
}
