//! Abstraction of different operating modes for the SSD1306

use crate::properties::DisplayProperties;
use crate::Error;
use hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

/// Trait with core functionality for display mode switching
pub trait DisplayModeTrait<DI, DSIZE, DROTATION>: Sized {
    /// Allocate all required data and initialise display for mode
    fn new(properties: DisplayProperties<DI, DSIZE, DROTATION>) -> Self;

    /// Deconstruct object and retrieve DisplayProperties
    fn into_properties(self) -> DisplayProperties<DI, DSIZE, DROTATION>;

    /// Release display interface
    fn release(self) -> DI {
        self.into_properties().release()
    }

    /// Reset the display
    fn reset<RST, DELAY, PinE>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), Error<(), PinE>>
    where
        RST: OutputPin<Error = PinE>,
        DELAY: DelayMs<u8>,
    {
        rst.set_high().map_err(Error::Pin)?;
        delay.delay_ms(1);
        rst.set_low().map_err(Error::Pin)?;
        delay.delay_ms(10);
        rst.set_high().map_err(Error::Pin)
    }
}
