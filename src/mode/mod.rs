//! Display modes.

mod buffered_graphics;
mod terminal;

use crate::{command::AddrMode, rotation::DisplayRotation, size::DisplaySize, Ssd1306};
pub use buffered_graphics::*;
use display_interface::{DisplayError, WriteOnlyDataCommand};
pub use terminal::*;

/// Common functions to all display modes.
pub trait DisplayConfig {
    /// Error.
    type Error;

    /// Set display rotation.
    fn set_rotation(&mut self, rotation: DisplayRotation) -> Result<(), Self::Error>;

    /// Initialise and configure the display for the given mode.
    fn init(&mut self) -> Result<(), Self::Error>;
}

/// A mode with no additional functionality beyond that provided by the base [`Ssd1306`] struct.
#[derive(Debug, Copy, Clone)]
pub struct BasicMode;

impl<DI, SIZE> Ssd1306<DI, SIZE, BasicMode>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    /// Clear the display.
    pub fn clear(&mut self) -> Result<(), DisplayError> {
        self.set_draw_area((0, 0), self.dimensions())?;

        // TODO: If const generics allows this, replace `1024` with computed W x H value for current
        // `SIZE`.
        self.draw(&[0u8; 1024])
    }
}

impl<DI, SIZE> DisplayConfig for Ssd1306<DI, SIZE, BasicMode>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Error = DisplayError;

    /// Set the display rotation.
    fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DisplayError> {
        self.set_rotation(rot)
    }

    /// Initialise in horizontal addressing mode.
    fn init(&mut self) -> Result<(), DisplayError> {
        self.init_with_addr_mode(AddrMode::Horizontal)
    }
}
