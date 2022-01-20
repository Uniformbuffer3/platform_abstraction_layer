/// Possible keyboard requests.
pub enum KeyboardRequest {
    ModifyLayout{layout: String},
    SetAutoRepeat{
        rate: u32,
        delay: u32
    },
}

