//! Buffered graphics mode.

use crate::{command::AddrMode, rotation::DisplayRotation, size::DisplaySize, Ssd1306};
use display_interface::{DisplayError, WriteOnlyDataCommand};
use generic_array::GenericArray;

/// Buffered graphics mode.
///
/// This mode keeps a pixel buffer in system memory, up to 1024 bytes for 128x64px displays. This
/// buffer is drawn to by [`set_pixel`](Ssd1306::set_pixel) commands or
/// [`embedded-graphics`](https://docs.rs/embedded-graphics) commands. The display can then be
/// updated using the [`flush`](Ssd1306::flush) method.
#[derive(Clone, Debug)]
pub struct BufferedGraphicsMode<SIZE>
where
    SIZE: DisplaySize,
{
    buffer: GenericArray<u8, SIZE::BufferSize>,
    min_x: u8,
    max_x: u8,
    min_y: u8,
    max_y: u8,
}

impl<SIZE> BufferedGraphicsMode<SIZE>
where
    SIZE: DisplaySize,
{
    /// Create a new buffered graphics mode instance.
    pub(crate) fn new() -> Self {
        Self {
            buffer: GenericArray::default(),
            min_x: 255,
            max_x: 0,
            min_y: 255,
            max_y: 0,
        }
    }
}

impl<DI, SIZE> DisplayConfig for Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Error = DisplayError;

    /// Set the display rotation
    ///
    /// This method resets the cursor but does not clear the screen.
    fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DisplayError> {
        self.set_rotation(rot)
    }

    /// Initialise and clear the display in graphics mode.
    fn init(&mut self) -> Result<(), DisplayError> {
        self.clear();
        self.init_with_addr_mode(AddrMode::Horizontal)
    }
}

impl<DI, SIZE> Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    /// Clear the display buffer. You need to call `disp.flush()` for any effect on the screen
    pub fn clear(&mut self) {
        self.mode.buffer = GenericArray::default();

        let (width, height) = self.dimensions();
        self.mode.min_x = 0;
        self.mode.max_x = width - 1;
        self.mode.min_y = 0;
        self.mode.max_y = height - 1;
    }

    /// Write out data to a display.
    ///
    /// This only updates the parts of the display that have changed since the last flush.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        // Nothing to do if no pixels have changed since the last update
        if self.mode.max_x < self.mode.min_x || self.mode.max_y < self.mode.min_y {
            return Ok(());
        }

        let (width, height) = self.dimensions();

        // Determine which bytes need to be sent
        let disp_min_x = self.mode.min_x;
        let disp_min_y = self.mode.min_y;

        let (disp_max_x, disp_max_y) = match self.rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (
                (self.mode.max_x + 1).min(width),
                (self.mode.max_y | 7).min(height),
            ),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (
                (self.mode.max_x | 7).min(width),
                (self.mode.max_y + 1).min(height),
            ),
        };

        self.mode.min_x = 255;
        self.mode.max_x = 0;
        self.mode.min_y = 255;
        self.mode.max_y = 0;

        // Tell the display to update only the part that has changed
        let offset_x = match self.rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate270 => SIZE::OFFSETX,
            DisplayRotation::Rotate180 | DisplayRotation::Rotate90 => {
                // If segment remapping is flipped, we need to calculate
                // the offset from the other edge of the display.
                SIZE::DRIVER_COLS - SIZE::WIDTH - SIZE::OFFSETX
            }
        };

        match self.rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                self.set_draw_area(
                    (disp_min_x + offset_x, disp_min_y + SIZE::OFFSETY),
                    (disp_max_x + offset_x, disp_max_y + SIZE::OFFSETY),
                )?;

                Self::flush_buffer_chunks(
                    &mut self.interface,
                    &self.mode.buffer,
                    width as usize,
                    (disp_min_x, disp_min_y),
                    (disp_max_x, disp_max_y),
                )
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                self.set_draw_area(
                    (disp_min_y + offset_x, disp_min_x + SIZE::OFFSETY),
                    (disp_max_y + offset_x, disp_max_x + SIZE::OFFSETY),
                )?;

                Self::flush_buffer_chunks(
                    &mut self.interface,
                    &self.mode.buffer,
                    height as usize,
                    (disp_min_y, disp_min_x),
                    (disp_max_y, disp_max_x),
                )
            }
        }
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let rotation = self.rotation;

        let (idx, bit) = match rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                let idx = ((y as usize) / 8 * SIZE::WIDTH as usize) + (x as usize);
                let bit = y % 8;

                (idx, bit)
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                let idx = ((x as usize) / 8 * SIZE::WIDTH as usize) + (y as usize);
                let bit = x % 8;

                (idx, bit)
            }
        };

        if let Some(byte) = self.mode.buffer.get_mut(idx) {
            // Keep track of max and min values
            self.mode.min_x = self.mode.min_x.min(x as u8);
            self.mode.max_x = self.mode.max_x.max(x as u8);

            self.mode.min_y = self.mode.min_y.min(y as u8);
            self.mode.max_y = self.mode.max_y.max(y as u8);

            // Set pixel value in byte
            // Ref this comment https://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit#comment46654671_47990
            *byte = *byte & !(1 << bit) | (value << bit)
        }
    }
}

#[cfg(feature = "graphics")]
use embedded_graphics::{
    drawable,
    geometry::Size,
    pixelcolor::{
        raw::{RawData, RawU1},
        BinaryColor,
    },
    DrawTarget,
};

use super::DisplayConfig;

#[cfg(feature = "graphics")]
impl<DI, SIZE> DrawTarget<BinaryColor> for Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Error = DisplayError;

    fn draw_pixel(&mut self, pixel: drawable::Pixel<BinaryColor>) -> Result<(), Self::Error> {
        let drawable::Pixel(pos, color) = pixel;

        // Guard against negative values. All positive i32 values from `pos` can be represented in
        // the `u32`s that `set_pixel()` accepts...
        if pos.x < 0 || pos.y < 0 {
            return Ok(());
        }

        // ... which makes the `as` coercions here safe.
        self.set_pixel(pos.x as u32, pos.y as u32, RawU1::from(color).into_inner());

        Ok(())
    }

    fn size(&self) -> Size {
        let (w, h) = self.dimensions();

        Size::new(w.into(), h.into())
    }
}
