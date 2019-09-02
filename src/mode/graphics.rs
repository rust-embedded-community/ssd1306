//! Buffered display module for use with the [embedded_graphics] crate
//!
//! ```rust
//! # use ssd1306::test_helpers::I2cStub;
//! # let i2c = I2cStub;
//! use ssd1306::{prelude::*, mode::GraphicsMode, Builder};
//! use embedded_graphics::{
//!     fonts::Font6x8,
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     primitives::{Circle, Line, Rectangle},
//! };
//!
//! let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
//!
//! display.init().unwrap();
//! display.flush().unwrap();
//! display.draw(
//!     Line::new(Point::new(0, 0), Point::new(16, 16))
//!         .stroke(Some(BinaryColor::On))
//!         .into_iter(),
//! );
//! display.draw(
//!     Rectangle::new(Point::new(24, 0), Point::new(40, 16))
//!         .stroke(Some(BinaryColor::On))
//!         .into_iter(),
//! );
//! display.draw(
//!     Circle::new(Point::new(64, 8), 8)
//!         .stroke(Some(BinaryColor::On))
//!         .into_iter(),
//! );
//! display.draw(
//!     Font6x8::render_str("Hello Rust!")
//!         .stroke(Some(BinaryColor::On))
//!         .translate(Point::new(24, 24))
//!         .into_iter(),
//! );
//! display.flush().unwrap();
//! ```
//!
//! [embedded_graphics]: https://crates.io/crates/embedded_graphics

use hal::blocking::delay::DelayMs;
use hal::digital::v2::OutputPin;

use crate::displayrotation::DisplayRotation;
use crate::displaysize::DisplaySize;
use crate::interface::DisplayInterface;
use crate::mode::displaymode::DisplayModeTrait;
use crate::properties::DisplayProperties;
use crate::Error;

// TODO: Add to prelude
/// Graphics mode handler
pub struct GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    properties: DisplayProperties<DI>,
    buffer: [u8; 1024],
}

impl<DI> DisplayModeTrait<DI> for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    /// Create new GraphicsMode instance
    fn new(properties: DisplayProperties<DI>) -> Self {
        GraphicsMode {
            properties,
            buffer: [0; 1024],
        }
    }

    /// Release all resources used by GraphicsMode
    fn release(self) -> DisplayProperties<DI> {
        self.properties
    }
}

impl<DI> GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    /// Clear the display buffer. You need to call `disp.flush()` for any effect on the screen
    pub fn clear(&mut self) {
        self.buffer = [0; 1024];
    }

    /// Reset display
    // TODO: Move to a more appropriate place
    pub fn reset<RST, DELAY, PinE>(
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

    /// Write out data to display
    pub fn flush(&mut self) -> Result<(), DI::Error> {
        let display_size = self.properties.get_size();

        // Ensure the display buffer is at the origin of the display before we send the full frame
        // to prevent accidental offsets
        let (display_width, display_height) = display_size.dimensions();
        self.properties
            .set_draw_area((0, 0), (display_width, display_height))?;

        match display_size {
            DisplaySize::Display128x64 => self.properties.draw(&self.buffer),
            DisplaySize::Display128x32 => self.properties.draw(&self.buffer[0..512]),
            DisplaySize::Display96x16 => self.properties.draw(&self.buffer[0..192]),
        }
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let (display_width, _) = self.properties.get_size().dimensions();
        let display_rotation = self.properties.get_rotation();

        let idx = match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                if x >= display_width as u32 {
                    return;
                }
                ((y as usize) / 8 * display_width as usize) + (x as usize)
            }

            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                if y >= display_width as u32 {
                    return;
                }
                ((x as usize) / 8 * display_width as usize) + (y as usize)
            }
        };

        if idx >= self.buffer.len() {
            return;
        }

        let (byte, bit) = match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                let byte =
                    &mut self.buffer[((y as usize) / 8 * display_width as usize) + (x as usize)];
                let bit = 1 << (y % 8);

                (byte, bit)
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                let byte =
                    &mut self.buffer[((x as usize) / 8 * display_width as usize) + (y as usize)];
                let bit = 1 << (x % 8);

                (byte, bit)
            }
        };

        if value == 0 {
            *byte &= !bit;
        } else {
            *byte |= bit;
        }
    }

    /// Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from
    /// column 0 on the left, to column _n_ on the right
    pub fn init(&mut self) -> Result<(), DI::Error> {
        self.properties.init_column_mode()
    }

    /// Get display dimensions, taking into account the current rotation of the display
    pub fn get_dimensions(&self) -> (u8, u8) {
        self.properties.get_dimensions()
    }

    /// Set the display rotation
    pub fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DI::Error> {
        self.properties.set_rotation(rot)
    }

    /// Turn the display on or off. The display can be drawn to and retains all
    /// of its memory even while off.
    pub fn display_on(&mut self, on: bool) -> Result<(), DI::Error> {
        self.properties.display_on(on)
    }
}

#[cfg(feature = "graphics")]
extern crate embedded_graphics;
#[cfg(feature = "graphics")]
use self::embedded_graphics::{drawable, pixelcolor::BinaryColor, Drawing};
#[cfg(feature = "graphics")]
use core::convert::TryInto;

#[cfg(feature = "graphics")]
impl<DI> Drawing<BinaryColor> for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: IntoIterator<Item = drawable::Pixel<BinaryColor>>,
    {
        // Filter out pixels that are off the top left of the screen
        let on_screen_pixels = item_pixels
            .into_iter()
            .filter(|drawable::Pixel(point, _)| point.x >= 0 && point.y >= 0);

        for drawable::Pixel(point, color) in on_screen_pixels {
            // NOTE: The filter above means the coordinate conversions should never panic
            self.set_pixel(
                point.x.try_into().expect("Point X coordinate is negative"),
                point.y.try_into().expect("Point Y coordinate is negative"),
                color as u8,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO lol
}
