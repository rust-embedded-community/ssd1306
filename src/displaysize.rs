//! Display size

// TODO: Add to prelude
/// Display size enumeration
#[derive(Clone, Copy)]
pub enum DisplaySize {
    /// 128 by 64 pixels
    Display128x64,
    /// 128 by 32 pixels
    Display128x32,
    /// 96 by 16 pixels
    Display96x16,
    /// 70 by 42 pixels
    Display72x40,
    /// 64 by 48 pixels
    Display64x48,
}

impl DisplaySize {
    /// Get integral dimensions from DisplaySize
    // TODO: Use whatever vec2 impl I decide to use here
    pub fn dimensions(&self) -> (u8, u8) {
        match *self {
            DisplaySize::Display128x64 => (128, 64),
            DisplaySize::Display128x32 => (128, 32),
            DisplaySize::Display96x16 => (96, 16),
            DisplaySize::Display72x40 => (72, 40),
            DisplaySize::Display64x48 => (64, 48),
        }
    }
}
