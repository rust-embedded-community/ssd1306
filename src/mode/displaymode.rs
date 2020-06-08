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
