//! Abstraction of different operating modes for the SSD1306

use crate::properties::DisplayProperties;

/// Trait with core functionality for display mode switching
pub trait DisplayModeTrait<DI> {
    /// Allocate all required data and initialise display for mode
    fn new(properties: DisplayProperties<DI>) -> Self;

    /// Release resources for reuse with different mode
    fn release(self) -> DisplayProperties<DI>;
}
