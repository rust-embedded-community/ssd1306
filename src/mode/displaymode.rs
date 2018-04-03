//! Abstraction of different operating modes for the SSD1306

use super::graphics::GraphicsMode;
use super::raw::RawMode;

use interface::DisplayInterface;
use properties::DisplayProperties;

/// Display mode abstraction
pub struct DisplayMode<MODE>(MODE);

/// Trait with core functionality for display mode switching
pub trait DisplayTrait<DI> {
    /// Allocate all required data and initialise display for mode
    fn new(properties: DisplayProperties<DI>) -> Self;

    /// Release resources for reuse with different mode
    fn release(self) -> DisplayProperties<DI>;
}

impl<MODE> DisplayMode<MODE> {
    /// Setup display to run in Raw mode
    pub fn new<DI>(properties: DisplayProperties<DI>) -> DisplayMode<RawMode<DI>>
    where
        DI: DisplayInterface,
    {
        DisplayMode(RawMode::new(properties))
    }

    /// Change display mode into graphics mode
    // TODO: Figure out how to stay as generic DisplayMode but act as particular mode
    // TODO: Figure out how to get rid of explicit mode switching functions
    pub fn into_graphicsmode<DI>(self) -> GraphicsMode<DI>
    where
        DI: DisplayInterface,
        MODE: DisplayTrait<DI>,
    {
        let properties = self.0.release();
        GraphicsMode::new(properties)
    }
}
