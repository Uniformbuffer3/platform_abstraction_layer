use crate::definitions::CursorMode;
use crate::definitions::CursorImage;

#[derive(Debug, Clone)]
/// Possible cursor requests.
pub enum CursorRequest {
    ChangeMode(CursorMode),
    ChangeImage(CursorImage)
}


