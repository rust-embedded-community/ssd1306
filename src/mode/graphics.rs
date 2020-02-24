//! Buffered display module for use with the [embedded_graphics] crate
//!
//! ```rust
//! # use ssd1306::test_helpers::I2cStub;
//! # let i2c = I2cStub;
//! use embedded_graphics::{
//!     fonts::Font6x8,
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     primitives::{Circle, Line, Rectangle, Triangle},
//!     style::PrimitiveStyleBuilder,
//! };
//! use ssd1306::{mode::GraphicsMode, prelude::*, Builder};
//!
//! let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
//!
//! display.init().unwrap();
//!
//! let yoffset = 20;
//!
//! let style = PrimitiveStyleBuilder::new()
//!     .stroke_width(1)
//!     .stroke_color(BinaryColor::On)
//!     .build();
//!
//! // screen outline
//! // default display size is 128x64 if you don't pass a _DisplaySize_
//! // enum to the _Builder_ struct
//! Rectangle::new(Point::new(0, 0), Point::new(127, 63))
//!     .into_styled(style)
//!     .draw(&mut display);
//!
//! // triangle
//! Triangle::new(
//!     Point::new(16, 16 + yoffset),
//!     Point::new(16 + 16, 16 + yoffset),
//!     Point::new(16 + 8, yoffset),
//! )
//! .into_styled(style)
//! .draw(&mut display);
//!
//! // square
//! Rectangle::new(Point::new(52, yoffset), Point::new(52 + 16, 16 + yoffset))
//!     .into_styled(style)
//!     .draw(&mut display);
//!
//! // circle
//! Circle::new(Point::new(96, yoffset + 8), 8)
//!     .into_styled(style)
//!     .draw(&mut display);
//!
//! display.flush().unwrap();
//! ```
//!
//! [embedded_graphics]: https://crates.io/crates/embedded_graphics

use hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use crate::{
    displayrotation::DisplayRotation, interface::DisplayInterface,
    mode::displaymode::DisplayModeTrait, properties::DisplayProperties, Error,
};

// TODO: Add to prelude
/// Graphics mode handler
pub struct GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    properties: DisplayProperties<DI>,
    buffer: [u8; 1024],
    min_x: u8,
    max_x: u8,
    min_y: u8,
    max_y: u8,
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
            min_x: 255,
            max_x: 0,
            min_y: 255,
            max_y: 0,
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

        let (width, height) = self.get_dimensions();
        self.min_x = 0;
        self.max_x = width - 1;
        self.min_y = 0;
        self.max_y = height - 1;
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

    /// Write out data to a display.
    ///
    /// This only updates the parts of the display that have changed since the last flush.
    pub fn flush(&mut self) -> Result<(), DI::Error> {
        // Nothing to do if no pixels have changed since the last update
        if self.max_x < self.min_x || self.max_y < self.min_y {
            return Ok(());
        }

        let (width, height) = self.get_dimensions();

        // Determine which bytes need to be sent
        let disp_min_x = self.min_x;
        let disp_min_y = self.min_y;

        let (disp_max_x, disp_max_y) = match self.properties.get_rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                ((self.max_x + 1).min(width), (self.max_y | 7).min(height))
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                ((self.max_x | 7).min(width), (self.max_y + 1).min(height))
            }
        };

        self.min_x = width - 1;
        self.max_x = 0;
        self.min_y = width - 1;
        self.max_y = 0;

        // Compensate for any offset in the physical display. For example, the 72x40 display has an
        // offset of (28, 0) pixels.
        let offs = self.properties.display_offset;

        // Tell the display to update only the part that has changed
        match self.properties.get_rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                self.properties.set_draw_area(
                    (disp_min_x + offs.0, disp_min_y + offs.1),
                    (disp_max_x + offs.0, disp_max_y + offs.1),
                )?;

                self.properties.bounded_draw(
                    &self.buffer,
                    width as usize,
                    (disp_min_x, disp_min_y),
                    (disp_max_x, disp_max_y),
                )
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                self.properties.set_draw_area(
                    (disp_min_y + offs.1, disp_min_x + offs.0),
                    (disp_max_y + offs.1, disp_max_x + offs.0),
                )?;

                self.properties.bounded_draw(
                    &self.buffer,
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
        let (display_width, _) = self.properties.get_size().dimensions();
        let display_rotation = self.properties.get_rotation();

        let (idx, bit) = match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                let idx = ((y as usize) / 8 * display_width as usize) + (x as usize);
                let bit = y % 8;

                (idx, bit)
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                let idx = ((x as usize) / 8 * display_width as usize) + (y as usize);
                let bit = x % 8;

                (idx, bit)
            }
        };

        if let Some(byte) = self.buffer.get_mut(idx) {
            // Keep track of max and min values
            self.min_x = self.min_x.min(x as u8);
            self.max_x = self.max_x.max(x as u8);

            self.min_y = self.min_y.min(y as u8);
            self.max_y = self.max_y.max(y as u8);

            // Set pixel value in byte
            // Ref this comment https://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit#comment46654671_47990
            *byte = *byte & !(1 << bit) | (value << bit)
        }
    }

    /// Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from
    /// column 0 on the left, to column _n_ on the right
    pub fn init(&mut self) -> Result<(), DI::Error> {
        self.clear();
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
use embedded_graphics::{
    drawable,
    geometry::Size,
    pixelcolor::{
        raw::{RawData, RawU1},
        BinaryColor,
    },
    DrawTarget,
};

#[cfg(feature = "graphics")]
impl<DI> DrawTarget<BinaryColor> for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    type Error = DI::Error;

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
        let (w, h) = self.get_dimensions();

        Size::new(w as u32, h as u32)
    }
}
