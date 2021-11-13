use crate::definitions::CursorMode;
use crate::definitions::CursorImage;

pub enum CursorRequest {
    ChangeMode(CursorMode),
    ChangeImage(CursorImage)
}


