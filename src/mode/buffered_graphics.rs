//! Buffered graphics mode.

use crate::{
    command::AddrMode,
    rotation::DisplayRotation,
    size::{DisplaySize, NewZeroed},
    Ssd1306,
};
#[cfg(feature = "async")]
use crate::{size::DisplaySizeAsync, Ssd1306Async};
#[cfg(feature = "async")]
use display_interface::AsyncWriteOnlyDataCommand;
use display_interface::{DisplayError, WriteOnlyDataCommand};

/// Buffered graphics mode.
///
/// This mode keeps a pixel buffer in system memory, up to 1024 bytes for 128x64px displays. This
/// buffer is drawn to by [`set_pixel`](Ssd1306::set_pixel) commands or
/// [`embedded-graphics`](https://docs.rs/embedded-graphics) commands. The display can then be
/// updated using the [`flush`](Ssd1306::flush) method.
#[maybe_async_cfg::maybe(
    sync(keep_self),
    async(feature = "async", idents(DisplaySize(async = "DisplaySizeAsync")))
)]
#[derive(Clone, Debug)]
pub struct BufferedGraphicsMode<SIZE>
where
    SIZE: DisplaySize,
{
    buffer: SIZE::Buffer,
    min_x: u8,
    max_x: u8,
    min_y: u8,
    max_y: u8,
}

#[maybe_async_cfg::maybe(
    sync(keep_self),
    async(feature = "async", idents(DisplaySize(async = "DisplaySizeAsync")))
)]
impl<SIZE> BufferedGraphicsMode<SIZE>
where
    SIZE: DisplaySize,
{
    /// Create a new buffered graphics mode instance.
    pub(crate) fn new() -> Self {
        Self {
            buffer: NewZeroed::new_zeroed(),
            min_x: 255,
            max_x: 0,
            min_y: 255,
            max_y: 0,
        }
    }
}

#[maybe_async_cfg::maybe(
    sync(keep_self),
    async(
        feature = "async",
        idents(
            DisplaySize(async = "DisplaySizeAsync"),
            DisplayConfig(async = "DisplayConfigAsync"),
            WriteOnlyDataCommand(async = "AsyncWriteOnlyDataCommand"),
            BufferedGraphicsMode(async = "BufferedGraphicsModeAsync"),
        )
    )
)]
impl<DI, SIZE> DisplayConfig for Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Error = DisplayError;

    /// Set the display rotation
    ///
    /// This method resets the cursor but does not clear the screen.
    async fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DisplayError> {
        self.set_rotation(rot).await
    }

    /// Initialise and clear the display in graphics mode.
    async fn init(&mut self) -> Result<(), DisplayError> {
        self.clear_impl(false);
        self.init_with_addr_mode(AddrMode::Horizontal).await
    }
}

#[maybe_async_cfg::maybe(
    sync(keep_self),
    async(
        feature = "async",
        idents(
            DisplaySize(async = "DisplaySizeAsync"),
            WriteOnlyDataCommand(async = "AsyncWriteOnlyDataCommand"),
            BufferedGraphicsMode(async = "BufferedGraphicsModeAsync")
        )
    )
)]
impl<DI, SIZE> Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    fn clear_impl(&mut self, value: bool) {
        self.mode.buffer.as_mut().fill(if value { 0xff } else { 0 });

        let (width, height) = self.dimensions();
        self.mode.min_x = 0;
        self.mode.max_x = width - 1;
        self.mode.min_y = 0;
        self.mode.max_y = height - 1;
    }

    /// Clear the underlying framebuffer. You need to call `disp.flush()` for any effect on the screen.
    pub fn clear_buffer(&mut self) {
        self.clear_impl(false);
    }

    /// Write out data to a display.
    ///
    /// This only updates the parts of the display that have changed since the last flush.
    pub async fn flush(&mut self) -> Result<(), DisplayError> {
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
                )
                .await?;

                Self::flush_buffer_chunks(
                    &mut self.interface,
                    self.mode.buffer.as_mut(),
                    width as usize,
                    (disp_min_x, disp_min_y),
                    (disp_max_x, disp_max_y),
                )
                .await
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                self.set_draw_area(
                    (disp_min_y + offset_x, disp_min_x + SIZE::OFFSETY),
                    (disp_max_y + offset_x, disp_max_x + SIZE::OFFSETY),
                )
                .await?;

                Self::flush_buffer_chunks(
                    &mut self.interface,
                    self.mode.buffer.as_mut(),
                    height as usize,
                    (disp_min_y, disp_min_x),
                    (disp_max_y, disp_max_x),
                )
                .await
            }
        }
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: bool) {
        let value = value as u8;
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

        if let Some(byte) = self.mode.buffer.as_mut().get_mut(idx) {
            // Keep track of max and min values
            self.mode.min_x = self.mode.min_x.min(x as u8);
            self.mode.max_x = self.mode.max_x.max(x as u8);

            self.mode.min_y = self.mode.min_y.min(y as u8);
            self.mode.max_y = self.mode.max_y.max(y as u8);

            // Set pixel value in byte
            // Ref this comment https://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit#comment46654671_47990
            *byte = *byte & !(1 << bit) | (value << bit);
        }
    }
}

#[cfg(feature = "graphics")]
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::Size,
    geometry::{Dimensions, OriginDimensions},
    pixelcolor::BinaryColor,
    Pixel,
};

use super::DisplayConfig;
#[cfg(feature = "async")]
use super::DisplayConfigAsync;

#[cfg(feature = "graphics")]
#[maybe_async_cfg::maybe(
    sync(keep_self),
    async(
        feature = "async",
        idents(
            DisplaySize(async = "DisplaySizeAsync"),
            BufferedGraphicsMode(async = "BufferedGraphicsModeAsync"),
            WriteOnlyDataCommand(async = "AsyncWriteOnlyDataCommand")
        )
    )
)]
impl<DI, SIZE> DrawTarget for Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Color = BinaryColor;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();

        pixels
            .into_iter()
            .filter(|Pixel(pos, _color)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| {
                self.set_pixel(pos.x as u32, pos.y as u32, color.is_on());
            });

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.clear_impl(color.is_on());
        Ok(())
    }
}

#[cfg(feature = "graphics")]
#[maybe_async_cfg::maybe(
    sync(keep_self,),
    async(
        feature = "async",
        idents(
            DisplaySize(async = "DisplaySizeAsync"),
            WriteOnlyDataCommand(async = "AsyncWriteOnlyDataCommand"),
            BufferedGraphicsMode(async = "BufferedGraphicsModeAsync")
        )
    )
)]
impl<DI, SIZE> OriginDimensions for Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    fn size(&self) -> Size {
        let (w, h) = self.dimensions();

        Size::new(w.into(), h.into())
    }
}
