//! A raw display mode

use interface::DisplayInterface;
use properties::DisplayProperties;

use mode::displaymode::DisplayTrait;

/// A display mode without higher level mostly meant as a stepstone for changing into higher
/// abstracted modes
pub struct RawMode<DI>
where
    DI: DisplayInterface,
{
    properties: DisplayProperties<DI>,
}

impl<DI> DisplayTrait<DI> for RawMode<DI>
where
    DI: DisplayInterface,
{
    /// Create new RawMode instance
    fn new(properties: DisplayProperties<DI>) -> Self {
        RawMode {
            properties,
        }
    }

    /// Release all resources used by RawMode
    fn release(self) -> DisplayProperties<DI> {
        self.properties
    }
}

impl<DI: DisplayInterface> RawMode<DI> {
    /// Create a new raw display mode
    pub fn new(properties: DisplayProperties<DI>) -> Self {
        RawMode { properties }
    }
}
