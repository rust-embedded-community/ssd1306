//! Raw mode for coercion into richer driver types
//!
//! A display driver instance without high level functionality used as a return type from the
//! builder. Used as a source to coerce the driver into richer modes like
//! [`GraphicsMode`](../graphics/index.html) and [`TerminalMode`](../terminal/index.html).

use crate::{mode::displaymode::DisplayModeTrait, properties::DisplayProperties};
use display_interface::WriteOnlyDataCommand;

/// Raw display mode
pub struct RawMode<DI>
where
    DI: WriteOnlyDataCommand<u8>,
{
    properties: DisplayProperties<DI>,
}

impl<DI> DisplayModeTrait<DI> for RawMode<DI>
where
    DI: WriteOnlyDataCommand<u8>,
{
    /// Create new RawMode instance
    fn new(properties: DisplayProperties<DI>) -> Self {
        RawMode { properties }
    }

    /// Release all resources used by RawMode
    fn release(self) -> DisplayProperties<DI> {
        self.properties
    }
}

impl<DI: WriteOnlyDataCommand<u8>> RawMode<DI> {
    /// Create a new raw display mode
    pub fn new(properties: DisplayProperties<DI>) -> Self {
        RawMode { properties }
    }
}
