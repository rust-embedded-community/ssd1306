//! A raw display mode

use interface::DisplayInterface;
use properties::DisplayProperties;

use mode::graphics::GraphicsMode;

/// A display mode without higher level mostly meant as a stepstone for changing into higher
/// abstracted modes
pub struct RawMode<DI>
where
    DI: DisplayInterface,
{
    properties: DisplayProperties<DI>,
}

impl<DI: DisplayInterface> RawMode<DI> {
    /// Create a new raw display mode
    pub fn new(properties: DisplayProperties<DI>) -> Self {
        RawMode { properties }
    }

    /// Changed into graphics mode
    pub fn into_graphicsmode(self) -> GraphicsMode<DI> {
        GraphicsMode::new(self.properties)
    }
}
