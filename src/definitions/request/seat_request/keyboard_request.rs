pub enum KeyboardRequest {
    ModifyLayout{layout: String},
    SetAutoRepeat(bool),
}

