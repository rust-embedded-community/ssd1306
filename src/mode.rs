//! Abstraction of different operating modes for the SSD1306

use displayrotation::DisplayRotation;
use displaysize::DisplaySize;
use interface::DisplayInterface;
use egfx::EgfxMode;

/// A mode that keeps the display uninitialised for conversion into other modes
pub struct NoMode<DI> {
    iface: DI,
    display_size: DisplaySize,
    display_rotation: DisplayRotation,
}

impl<DI: DisplayInterface> NoMode<DI> {
    /// Create a new dummy interface
    pub fn new(iface: DI, display_size: DisplaySize, display_rotation: DisplayRotation) -> Self {
        NoMode {
            iface,
            display_size,
            display_rotation,
        }
    }

    /// Changed into the embedded graphics mode
    pub fn into_egfx(self) -> EgfxMode<DI> {
        EgfxMode::new(self.iface, self.display_size, self.display_rotation)
    }
}
