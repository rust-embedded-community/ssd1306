//! Display brightness

/// Struct that holds display brightness
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Brightness {
    pub(crate) precharge: u8,
    pub(crate) contrast: u8,
}

impl Default for Brightness {
    fn default() -> Self {
        Brightness::NORMAL
    }
}

impl Brightness {
    /// The dimmest predefined brightness level
    pub const DIMMEST: Brightness = Brightness::custom(0x1, 0x00);

    /// A dim predefined brightness level
    pub const DIM: Brightness = Brightness::custom(0x2, 0x2F);

    /// A medium predefined brightness level
    pub const NORMAL: Brightness = Brightness::custom(0x2, 0x5F);

    /// A bright predefined brightness level
    pub const BRIGHT: Brightness = Brightness::custom(0x2, 0x9F);

    /// The brightest predefined brightness level
    pub const BRIGHTEST: Brightness = Brightness::custom(0x2, 0xFF);

    /// Create a Brightness object from a precharge period and contrast pair.
    ///
    /// `precharge` sets the `phase 1` argument of the `0xD9 Set Pre-Charge Period` command and must
    /// be must be between 1 and 15. See section 10.1.17 of the SSD1306 datasheet for more
    /// information.
    ///
    /// `contrast` sets the value used in the `0x81 Set Contrast Control` command and must be
    /// between 0 and 255. See section 10.1.7 of the SSD1306 datasheet for more information.
    const fn custom(precharge: u8, contrast: u8) -> Self {
        Self {
            precharge,
            contrast,
        }
    }
}
