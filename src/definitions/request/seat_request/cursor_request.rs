use crate::definitions::CursorMode;
use crate::definitions::CursorImage;

#[derive(Debug, Clone)]
pub enum CursorRequest {
    ChangeMode(CursorMode),
    ChangeImage(CursorImage)
}


