pub fn keysym_to_w3c_keycode(keysym: u32) -> Option<keyboard_types::Code> {
    use keyboard_types::Code;
    use keystroke_decoder::keysyms;

    let code = match keysym {
        keysyms::KEY_A | keysyms::KEY_a => Code::KeyA,
        keysyms::KEY_B | keysyms::KEY_b => Code::KeyB,
        keysyms::KEY_C | keysyms::KEY_c => Code::KeyC,
        keysyms::KEY_D | keysyms::KEY_d => Code::KeyD,
        keysyms::KEY_E | keysyms::KEY_e => Code::KeyE,
        keysyms::KEY_F | keysyms::KEY_f => Code::KeyF,
        keysyms::KEY_G | keysyms::KEY_g => Code::KeyG,
        keysyms::KEY_H | keysyms::KEY_h => Code::KeyH,
        keysyms::KEY_I | keysyms::KEY_i => Code::KeyI,
        keysyms::KEY_J | keysyms::KEY_j => Code::KeyJ,
        keysyms::KEY_K | keysyms::KEY_k => Code::KeyK,
        keysyms::KEY_L | keysyms::KEY_l => Code::KeyL,
        keysyms::KEY_M | keysyms::KEY_m => Code::KeyM,
        keysyms::KEY_N | keysyms::KEY_n => Code::KeyN,
        keysyms::KEY_O | keysyms::KEY_o => Code::KeyO,
        keysyms::KEY_P | keysyms::KEY_p => Code::KeyP,
        keysyms::KEY_Q | keysyms::KEY_q => Code::KeyQ,
        keysyms::KEY_R | keysyms::KEY_r => Code::KeyR,
        keysyms::KEY_S | keysyms::KEY_s => Code::KeyS,
        keysyms::KEY_T | keysyms::KEY_t => Code::KeyT,
        keysyms::KEY_U | keysyms::KEY_u => Code::KeyU,
        keysyms::KEY_V | keysyms::KEY_v => Code::KeyV,
        keysyms::KEY_W | keysyms::KEY_w => Code::KeyW,
        keysyms::KEY_X | keysyms::KEY_x => Code::KeyX,
        keysyms::KEY_Y | keysyms::KEY_y => Code::KeyY,
        keysyms::KEY_Z | keysyms::KEY_z => Code::KeyZ,
        keysyms::KEY_0 => Code::Digit0,
        keysyms::KEY_1 => Code::Digit1,
        keysyms::KEY_2 => Code::Digit2,
        keysyms::KEY_3 => Code::Digit3,
        keysyms::KEY_4 => Code::Digit4,
        keysyms::KEY_5 => Code::Digit5,
        keysyms::KEY_6 => Code::Digit6,
        keysyms::KEY_7 => Code::Digit7,
        keysyms::KEY_8 => Code::Digit8,
        keysyms::KEY_9 => Code::Digit9,
        _ => return None,
    };
    Some(code)
}

use crate::definitions::Button;
pub fn keysym_to_button(keysym: u32) -> Option<Button> {
    use keystroke_decoder::keysyms;

    let code = match keysym {
        keysyms::KEY_Pointer_Button1 => Button::Left,
        keysyms::KEY_Pointer_Button2 => Button::Middle,
        keysyms::KEY_Pointer_Button3 => Button::Right,
        _ => return None,
    };
    Some(code)
}
