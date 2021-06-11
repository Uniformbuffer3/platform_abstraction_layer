
#[derive(Debug,PartialEq)]
pub enum OutputEvent {
    Added {
        x: u32,
        y: u32,
        selected_mode: Mode,
        available_modes: Vec<Mode>,
        physical_width: u32,
        physical_height: u32,
        subpixel: Subpixel,
    },
    ModeAdded{mode: Mode},
    ModeChanged{mode: Mode},
    Removed
}

#[derive(Debug, Clone,PartialEq)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub is_preferred: bool,
}

#[derive(Debug, Clone,PartialEq)]
pub enum Subpixel {
    Unknown,
    None,
    HorizontalRgb,
    HorizontalBgr,
    VerticalRgb,
    VerticalBgr,
}
