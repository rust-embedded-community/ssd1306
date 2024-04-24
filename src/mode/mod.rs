//! Display modes.

mod buffered_graphics;
mod terminal;

use crate::{command::AddrMode, rotation::DisplayRotation, size::DisplaySize, Ssd1306};
pub use buffered_graphics::*;
use display_interface::{AsyncWriteOnlyDataCommand, DisplayError};
pub use terminal::*;

/// Common functions to all display modes.
pub trait DisplayConfig {
    /// Error.
    type Error;

    /// Set display rotation.
    async fn set_rotation(&mut self, rotation: DisplayRotation) -> Result<(), Self::Error>;

    /// Initialise and configure the display for the given mode.
    async fn init(&mut self) -> Result<(), Self::Error>;
}

/// A mode with no additional functionality beyond that provided by the base [`Ssd1306`] struct.
#[derive(Debug, Copy, Clone)]
pub struct BasicMode;

impl<DI, SIZE> Ssd1306<DI, SIZE, BasicMode>
where
    DI: AsyncWriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    /// Clear the display.
    pub async fn clear(&mut self) -> Result<(), DisplayError> {
        let old_addr_mode = self.addr_mode;
        if old_addr_mode != AddrMode::Horizontal {
            self.set_addr_mode(AddrMode::Horizontal).await?;
        }

        let dim = self.dimensions();
        self.set_draw_area((0, 0), dim).await?;

        let num_pixels = dim.0 as u16 * dim.1 as u16;

        const BITS_PER_BYTE: u16 = 8;
        const BYTES_PER_BATCH: u16 = 64;
        const PIXELS_PER_BATCH: u16 = BITS_PER_BYTE * BYTES_PER_BATCH;

        // Not all screens have number of pixels divisible by 512, so add 1 to cover tail
        let num_batches = num_pixels / PIXELS_PER_BATCH + 1;

        for _ in 0..num_batches {
            self.draw(&[0; BYTES_PER_BATCH as usize]).await?;
        }

        if old_addr_mode != AddrMode::Horizontal {
            self.set_addr_mode(old_addr_mode).await?;
        }

        Ok(())
    }
}

impl<DI, SIZE> DisplayConfig for Ssd1306<DI, SIZE, BasicMode>
where
    DI: AsyncWriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Error = DisplayError;

    /// Set the display rotation.
    async fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DisplayError> {
        self.set_rotation(rot).await
    }

    /// Initialise in horizontal addressing mode.
    async fn init(&mut self) -> Result<(), DisplayError> {
        self.init_with_addr_mode(AddrMode::Horizontal).await
    }
}
