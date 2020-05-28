//! Display brightness

/// Struct that holds display brightness
#[derive(Copy, Clone)]
pub struct Brightness {
    pub(crate) precharge: u8,
    pub(crate) contrast: u8,
}

impl Brightness {
    /// The darkest predefined brightness level
    pub const DARKEST: Brightness   = Brightness::custom(0x1, 0x00);

    /// A dark predefined brightness level
    pub const DARK: Brightness      = Brightness::custom(0x2, 0x2F);

    /// A medium predefined brightness level
    pub const NORMAL: Brightness    = Brightness::custom(0x2, 0x5F);

    /// A bright predefined brightness level
    pub const BRIGHT: Brightness    = Brightness::custom(0x2, 0x9F);

    /// The brightest predefined brightness level
    pub const BRIGHTEST: Brightness = Brightness::custom(0x2, 0xFF);

    /// Create a Brightness object from a precharge period and contrast pair.
    ///
    /// `precharge` must be between 1 and 15
    const fn custom(precharge: u8, contrast: u8) -> Self {
        Self {
            precharge,
            contrast,
        }
    }
}
