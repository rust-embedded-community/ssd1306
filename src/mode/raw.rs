//! A raw display mode

use displayrotation::DisplayRotation;
use displaysize::DisplaySize;
use interface::DisplayInterface;

use mode::graphics::GraphicsMode;

/// A display mode without higher level mostly meant as a stepstone for changing into higher
/// abstracted modes
pub struct RawMode<DI> {
    iface: DI,
    display_size: DisplaySize,
    display_rotation: DisplayRotation,
}

impl<DI: DisplayInterface> RawMode<DI> {
    /// Create a new raw display mode
    pub fn new(iface: DI, display_size: DisplaySize, display_rotation: DisplayRotation) -> Self {
        RawMode {
            iface,
            display_size,
            display_rotation,
        }
    }

    /// Changed into graphics mode
    pub fn into_graphicsmode(self) -> GraphicsMode<DI> {
        GraphicsMode::new(self.iface, self.display_size, self.display_rotation)
    }
}
