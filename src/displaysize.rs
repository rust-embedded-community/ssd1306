//! Display size

// TODO: Add to prelude
/// Display size enumeration
#[derive(Clone, Copy)]
pub enum DisplaySize {
    /// 128 by 64 pixels
    Display128x64,
    /// 132 by 64 pixels, try this for ssd1305 chips that are 128x64.
    Display132x64,
    /// 128 by 32 pixels
    Display128x32,
    /// 96 by 16 pixels
    Display96x16,
}

impl DisplaySize {
    /// Get integral dimensions from DisplaySize
    // TODO: Use whatever vec2 impl I decide to use here
    pub fn dimensions(&self) -> (u8, u8) {
        match *self {
            DisplaySize::Display128x64 => (128, 64),
            DisplaySize::Display132x64 => (134, 64),
            DisplaySize::Display128x32 => (128, 32),
            DisplaySize::Display96x16 => (96, 16),
        }
    }

    /// Get the number of characters this display can show. based on a 8x8 font.
    pub fn numchars(&self) -> u8 {
        match *self {
            DisplaySize::Display128x64 => 128,
            DisplaySize::Display132x64 => 64,
            DisplaySize::Display128x32 => 64,
            DisplaySize::Display96x16 => 24,
        }
    }

    /// Get the area within the display that characters will be drawn.
    /// This is primaritly implemented for the framebuffer 132x64 hack
    /// because the buffer is larger than the draw area.
    pub fn draw_area(&self) -> ((u8, u8), (u8, u8)) {
        match *self {
            DisplaySize::Display128x64 => ((0, 0), (128, 64)),
            DisplaySize::Display132x64 => ((4, 32), (132, 64)),
            DisplaySize::Display128x32 => ((0, 0), (128, 32)),
            DisplaySize::Display96x16 => ((0, 0), (96, 16)),
        }
    }
}
