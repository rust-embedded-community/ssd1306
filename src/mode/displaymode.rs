//! Abstraction of different operating modes for the SSD1306

use crate::properties::DisplayProperties;

/// Trait with core functionality for display mode switching
pub trait DisplayModeTrait<DI>: Sized {
    /// Allocate all required data and initialise display for mode
    fn new(properties: DisplayProperties<DI>) -> Self;

    /// Deconstruct object and retrieve DisplayProperties
    fn into_properties(self) -> DisplayProperties<DI>;

    /// Release display interface
    fn release(self) -> DI {
        self.into_properties().release()
    }
}

impl<MODE> DisplayMode<MODE> {
    /// Setup display to run in requested mode
    pub fn new<DI>(properties: DisplayProperties<DI>) -> Self
    where
        DI: WriteOnlyDataCommand,
        MODE: DisplayModeTrait<DI>,
    {
        DisplayMode(MODE::new(properties))
    }

    /// Change into any mode implementing DisplayModeTrait
    // TODO: Figure out how to stay as generic DisplayMode but act as particular mode
    pub fn into<DI, NMODE: DisplayModeTrait<DI>>(self) -> NMODE
    where
        DI: WriteOnlyDataCommand,
        MODE: DisplayModeTrait<DI>,
    {
        let properties = self.0.into_properties();
        NMODE::new(properties)
    }
}
