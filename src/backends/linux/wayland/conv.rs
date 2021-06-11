/*
impl From<smithay_client_toolkit::output::OutputInfo> for crate::definitions::OutputInfo {
    fn from(output_info: smithay_client_toolkit::output::OutputInfo) -> Self {
        Self {
            id: output_info.id.into(),
            model: output_info.model.clone(),
            make: output_info.make.clone(),
            location: output_info.location.clone(),
            physical_size: output_info.physical_size.clone(),
            subpixel: output_info.subpixel.into(),
            scale_factor: output_info.scale_factor.clone(),
            modes: output_info
                .modes
                .into_iter()
                .map(|mode| mode.into())
                .collect(),
        }
    }
}*/
impl From<smithay_client_toolkit::output::Subpixel> for crate::definitions::Subpixel {
    fn from(subpixel: smithay_client_toolkit::output::Subpixel) -> Self {
        match subpixel {
            smithay_client_toolkit::output::Subpixel::Unknown => Self::Unknown,
            smithay_client_toolkit::output::Subpixel::None => Self::None,
            smithay_client_toolkit::output::Subpixel::HorizontalRgb => Self::HorizontalRgb,
            smithay_client_toolkit::output::Subpixel::HorizontalBgr => Self::HorizontalBgr,
            smithay_client_toolkit::output::Subpixel::VerticalRgb => Self::VerticalRgb,
            smithay_client_toolkit::output::Subpixel::VerticalBgr => Self::VerticalBgr,
            _ => panic!(),
        }
    }
}

impl From<smithay_client_toolkit::output::Transform> for crate::definitions::Transform {
    fn from(transform: smithay_client_toolkit::output::Transform) -> Self {
        match transform {
            smithay_client_toolkit::output::Transform::Normal => Self::Normal,
            smithay_client_toolkit::output::Transform::_90 => Self::_90,
            smithay_client_toolkit::output::Transform::_180 => Self::_180,
            smithay_client_toolkit::output::Transform::_270 => Self::_270,
            smithay_client_toolkit::output::Transform::Flipped => Self::Flipped,
            smithay_client_toolkit::output::Transform::Flipped90 => Self::Flipped90,
            smithay_client_toolkit::output::Transform::Flipped180 => Self::Flipped180,
            smithay_client_toolkit::output::Transform::Flipped270 => Self::Flipped270,
            _ => panic!(),
        }
    }
}
/*
impl From<smithay_client_toolkit::output::Mode> for crate::definitions::Mode {
    fn from(mode_info: smithay_client_toolkit::output::Mode) -> Self {
        Self {
            dimensions: mode_info.dimensions,
            refresh_rate: mode_info.refresh_rate,
            is_current: mode_info.is_current,
            is_preferred: mode_info.is_preferred,
        }
    }
}
*/
impl From<(crate::definitions::SeatId, smithay_client_toolkit::seat::SeatData)> for crate::definitions::SeatInfo {
    fn from(seat: (crate::definitions::SeatId, smithay_client_toolkit::seat::SeatData)) -> Self {
        Self {
            id: seat.0,
            name: seat.1.name,
            has_pointer: seat.1.has_pointer,
            has_keyboard: seat.1.has_keyboard,
            has_touch: seat.1.has_touch,
        }
    }
}

/*
use keyboard_types::Code;
pub fn keysym_to_code(keysym: xkbcommon::xkb::Keysym)->Option<Code>{
    match keysym {
        keysyms::XKB_KEY_A=>Some(Code::KeyA),
        _=>None
    }

}*/
