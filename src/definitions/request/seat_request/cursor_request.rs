use crate::definitions::CursorMode;

pub enum CursorRequest {
    ChangeMode(CursorMode),
}

