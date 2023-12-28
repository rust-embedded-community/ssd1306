//! SSD1306 OLED display driver.
//!
//! This crate provides a driver interface to the popular SSD1306 monochrome OLED display driver. It
//! supports I2C and SPI via the [`display_interface`](https://docs.rs/display_interface) crate.
//!
//! The main driver is created using [`Ssd1306::new`] which accepts an interface instance, display
//! size, rotation and mode. The following display modes are supported:
//!
//! - [`BasicMode`](crate::mode::BasicMode) - A simple mode with lower level methods available.
//! - [`BufferedGraphicsMode`] - A framebuffered mode with additional methods and integration with
//!   [embedded-graphics](https://docs.rs/embedded-graphics).
//! - [`TerminalMode`] - A bufferless mode supporting drawing text to the display, as well as
//!   setting cursor positions like a simple terminal.
//!
//! # Examples
//!
//! Examples can be found in [the examples/
//! folder](https://github.com/jamwaffles/ssd1306/blob/master/examples)
//!
//! ## Draw some text to the display
//!
//! Uses [`BufferedGraphicsMode`] and [embedded_graphics](https://docs.rs/embedded-graphics). [See
//! the complete example
//! here](https://github.com/jamwaffles/ssd1306/blob/master/examples/text_i2c.rs).
//!
//! ```rust
//! # use ssd1306::test_helpers::I2cStub;
//! # let i2c = I2cStub;
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     text::{Baseline, Text},
//! };
//! use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
//!
//! let interface = I2CDisplayInterface::new(i2c);
//! let mut display = Ssd1306::new(
//!     interface,
//!     DisplaySize128x64,
//!     DisplayRotation::Rotate0,
//! ).into_buffered_graphics_mode();
//! display.init().unwrap();
//!
//! let text_style = MonoTextStyleBuilder::new()
//!     .font(&FONT_6X10)
//!     .text_color(BinaryColor::On)
//!     .build();
//!
//! Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
//!     .draw(&mut display)
//!     .unwrap();
//!
//! Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
//!     .draw(&mut display)
//!     .unwrap();
//!
//! display.flush().unwrap();
//! ```
//!
//! ## Write text to the display without a framebuffer
//!
//! Uses [`TerminalMode`]. [See the complete example
//! here](https://github.com/jamwaffles/ssd1306/blob/master/examples/terminal_i2c.rs).
//!
//! ```rust
//! # use ssd1306::test_helpers::I2cStub;
//! # let i2c = I2cStub;
//! use core::fmt::Write;
//! use ssd1306::{mode::TerminalMode, prelude::*, I2CDisplayInterface, Ssd1306};
//!
//! let interface = I2CDisplayInterface::new(i2c);
//!
//! let mut display = Ssd1306::new(
//!     interface,
//!     DisplaySize128x64,
//!     DisplayRotation::Rotate0,
//! ).into_terminal_mode();
//! display.init().unwrap();
//! display.clear().unwrap();
//!
//! // Spam some characters to the display
//! for c in 97..123 {
//!     let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
//! }
//! for c in 65..91 {
//!     let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
//! }
//!
//! // The `write!()` macro is also supported
//! write!(display, "Hello, {}", "world");
//! ```
//!
//! [featureset]: https://github.com/jamwaffles/embedded-graphics#features
//! [`BufferedGraphicsMode`]: crate::mode::BufferedGraphicsMode
//! [`TerminalMode`]: crate::mode::TerminalMode

#![no_std]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(async_fn_in_trait)]

mod brightness;
pub mod command;
mod error;
mod i2c_interface;
pub mod mode;
pub mod prelude;
pub mod rotation;
pub mod size;
#[doc(hidden)]
pub mod test_helpers;

pub use crate::i2c_interface::I2CDisplayInterface;
use crate::mode::BasicMode;
use brightness::Brightness;
use command::{AddrMode, Command, VcomhLevel};
use core::convert::Infallible;
use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use error::Error;
use mode::{BufferedGraphicsMode, TerminalMode};
use rotation::DisplayRotation;
use size::DisplaySize;

/// SSD1306 driver.
///
/// Note that some methods are only available when the display is configured in a certain [`mode`].
#[derive(Copy, Clone, Debug)]
pub struct Ssd1306<DI, SIZE, MODE> {
    interface: DI,
    mode: MODE,
    size: SIZE,
    addr_mode: AddrMode,
    rotation: DisplayRotation,
}

impl<DI, SIZE> Ssd1306<DI, SIZE, BasicMode>
where
    SIZE: DisplaySize,
{
    /// Create a basic SSD1306 interface.
    ///
    /// Use the `into_*_mode` methods to enable more functionality.
    pub fn new(interface: DI, size: SIZE, rotation: DisplayRotation) -> Self {
        Self {
            interface,
            size,
            addr_mode: AddrMode::Page,
            mode: BasicMode,
            rotation,
        }
    }
}

impl<DI, SIZE, MODE> Ssd1306<DI, SIZE, MODE>
where
    SIZE: DisplaySize,
{
    /// Convert the display into another interface mode.
    fn into_mode<MODE2>(self, mode: MODE2) -> Ssd1306<DI, SIZE, MODE2> {
        Ssd1306 {
            mode,
            addr_mode: self.addr_mode,
            interface: self.interface,
            size: self.size,
            rotation: self.rotation,
        }
    }

    /// Convert the display into a buffered graphics mode, supporting
    /// [embedded-graphics](https://crates.io/crates/embedded-graphics).
    ///
    /// See [`BufferedGraphicsMode`] for more information.
    pub fn into_buffered_graphics_mode(self) -> Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>> {
        self.into_mode(BufferedGraphicsMode::new())
    }

    /// Convert the display into a text-only, terminal-like mode.
    ///
    /// See [`TerminalMode`] for more information.
    pub fn into_terminal_mode(self) -> Ssd1306<DI, SIZE, TerminalMode> {
        self.into_mode(TerminalMode::new())
    }

    fn rotation_commands(&self, rotation: DisplayRotation) -> [Command; 2] {
        let (remap, reverse) = match rotation {
            DisplayRotation::Rotate0 => (true, true),
            DisplayRotation::Rotate90 => (false, true),
            DisplayRotation::Rotate180 => (false, false),
            DisplayRotation::Rotate270 => (true, false),
        };

        [
            Command::SegmentRemap(remap),
            Command::ReverseComDir(reverse),
        ]
    }

    fn mirror_commands(&self, mirror: bool) -> [Command; 2] {
        if mirror {
            let (remap, reverse) = match self.rotation {
                DisplayRotation::Rotate0 => (false, true),
                DisplayRotation::Rotate90 => (false, false),
                DisplayRotation::Rotate180 => (true, false),
                DisplayRotation::Rotate270 => (true, true),
            };

            [
                Command::SegmentRemap(remap),
                Command::ReverseComDir(reverse),
            ]
        } else {
            self.rotation_commands(self.rotation)
        }
    }

    fn brightness_commands(&self, brightness: Brightness) -> [Command; 2] {
        [
            Command::PreChargePeriod(1, brightness.precharge),
            Command::Contrast(brightness.contrast),
        ]
    }

    fn init_commands(&self, mode: AddrMode) -> [Command; 18] {
        [
            Command::DisplayOn(false),
            Command::DisplayClockDiv(0x8, 0x0),
            Command::Multiplex(SIZE::HEIGHT - 1),
            Command::DisplayOffset(0),
            Command::StartLine(0),
            // TODO: Ability to turn charge pump on/off
            Command::ChargePump(true),
            Command::AddressMode(mode),
            self.size.commands()[0],
            self.size.commands()[1],
            self.rotation_commands(self.rotation)[0],
            self.rotation_commands(self.rotation)[1],
            self.brightness_commands(Brightness::default())[0],
            self.brightness_commands(Brightness::default())[1],
            Command::VcomhDeselect(VcomhLevel::Auto),
            Command::AllOn(false),
            Command::Invert(false),
            Command::EnableScroll(false),
            Command::DisplayOn(true),
        ]
    }

    fn draw_area_commands(&self, start: (u8, u8), end: (u8, u8)) -> [Command; 2] {
        [
            Command::ColumnAddress(start.0, end.0.saturating_sub(1)),
            if self.addr_mode != AddrMode::Page {
                Command::PageAddress(start.1.into(), (end.1.saturating_sub(1)).into())
            } else {
                Command::FastNoop
            },
        ]
    }

    fn buffer_chunks(
        buffer: &[u8],
        disp_width: usize,
        upper_left: (u8, u8),
        lower_right: (u8, u8),
    ) -> impl Iterator<Item = &[u8]> {
        // Divide by 8 since each row is actually 8 pixels tall
        let num_pages = ((lower_right.1 - upper_left.1) / 8) as usize + 1;

        // Each page is 8 bits tall, so calculate which page number to start at (rounded down) from
        // the top of the display
        let starting_page = (upper_left.1 / 8) as usize;

        // Calculate start and end X coordinates for each page
        let page_lower = upper_left.0 as usize;
        let page_upper = lower_right.0 as usize;

        buffer
            .chunks(disp_width)
            .skip(starting_page)
            .take(num_pages)
            .map(move |s| &s[page_lower..page_upper])
    }

    /// Get display dimensions, taking into account the current rotation of the display
    ///
    /// ```rust
    /// # use ssd1306::test_helpers::StubInterface;
    /// # let interface = StubInterface;
    /// use ssd1306::{mode::TerminalMode, prelude::*, Ssd1306};
    ///
    /// let mut display = Ssd1306::new(
    ///     interface,
    ///     DisplaySize128x64,
    ///     DisplayRotation::Rotate0,
    /// ).into_terminal_mode();
    /// assert_eq!(display.dimensions(), (128, 64));
    ///
    /// # let interface = StubInterface;
    /// let mut rotated_display = Ssd1306::new(
    ///     interface,
    ///     DisplaySize128x64,
    ///     DisplayRotation::Rotate90,
    /// ).into_terminal_mode();
    /// assert_eq!(rotated_display.dimensions(), (64, 128));
    /// ```
    pub fn dimensions(&self) -> (u8, u8) {
        match self.rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (SIZE::WIDTH, SIZE::HEIGHT),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (SIZE::HEIGHT, SIZE::WIDTH),
        }
    }

    /// Get the display rotation.
    pub fn rotation(&self) -> DisplayRotation {
        self.rotation
    }
}

impl<DI, SIZE, MODE> Ssd1306<DI, SIZE, MODE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    fn send_commands(&mut self, commands: &[Command]) -> Result<(), DisplayError> {
        for command in commands {
            command.send(&mut self.interface)?;
        }

        Ok(())
    }

    /// Initialise the display in one of the available addressing modes.
    pub fn init_with_addr_mode(&mut self, mode: AddrMode) -> Result<(), DisplayError> {
        self.send_commands(&self.init_commands(mode))?;
        self.addr_mode = mode;

        Ok(())
    }

    /// Change the addressing mode
    pub fn set_addr_mode(&mut self, mode: AddrMode) -> Result<(), DisplayError> {
        self.send_commands(&[Command::AddressMode(mode)])?;
        self.addr_mode = mode;

        Ok(())
    }

    /// Send the data to the display for drawing at the current position in the framebuffer
    /// and advance the position accordingly. Cf. `set_draw_area` to modify the affected area by
    /// this method.
    ///
    /// This method takes advantage of a bounding box for faster writes.
    pub fn bounded_draw(
        &mut self,
        buffer: &[u8],
        disp_width: usize,
        upper_left: (u8, u8),
        lower_right: (u8, u8),
    ) -> Result<(), DisplayError> {
        Self::flush_buffer_chunks(
            &mut self.interface,
            buffer,
            disp_width,
            upper_left,
            lower_right,
        )
    }

    /// Send a raw buffer to the display.
    pub fn draw(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.interface.send_data(U8(buffer))
    }

    /// Set the display rotation.
    pub fn set_rotation(&mut self, rotation: DisplayRotation) -> Result<(), DisplayError> {
        self.send_commands(&self.rotation_commands(rotation))?;
        self.rotation = rotation;

        Ok(())
    }

    /// Set mirror enabled/disabled.
    pub fn set_mirror(&mut self, mirror: bool) -> Result<(), DisplayError> {
        self.send_commands(&self.mirror_commands(mirror))
    }

    /// Change the display brightness.
    pub fn set_brightness(&mut self, brightness: Brightness) -> Result<(), DisplayError> {
        self.send_commands(&self.brightness_commands(brightness))
    }

    /// Turn the display on or off. The display can be drawn to and retains all
    /// of its memory even while off.
    pub fn set_display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_commands(&[Command::DisplayOn(on)])
    }

    /// Set the position in the framebuffer of the display limiting where any sent data should be
    /// drawn. This method can be used for changing the affected area on the screen as well
    /// as (re-)setting the start point of the next `draw` call.
    pub fn set_draw_area(&mut self, start: (u8, u8), end: (u8, u8)) -> Result<(), DisplayError> {
        self.send_commands(&self.draw_area_commands(start, end))
    }

    /// Set the column address in the framebuffer of the display where any sent data should be
    /// drawn.
    pub fn set_column(&mut self, column: u8) -> Result<(), DisplayError> {
        self.send_commands(&[Command::ColStart(column)])
    }

    /// Set the page address (row 8px high) in the framebuffer of the display where any sent data
    /// should be drawn.
    ///
    /// Note that the parameter is in pixels, but the page will be set to the start of the 8px
    /// row which contains the passed-in row.
    pub fn set_row(&mut self, row: u8) -> Result<(), DisplayError> {
        self.send_commands(&[Command::PageStart(row.into())])
    }

    /// Set the screen pixel on/off inversion
    pub fn set_invert(&mut self, invert: bool) -> Result<(), DisplayError> {
        Command::Invert(invert).send(&mut self.interface)
    }

    fn flush_buffer_chunks(
        interface: &mut DI,
        buffer: &[u8],
        disp_width: usize,
        upper_left: (u8, u8),
        lower_right: (u8, u8),
    ) -> Result<(), DisplayError> {
        Self::buffer_chunks(buffer, disp_width, upper_left, lower_right)
            .try_for_each(|c| interface.send_data(U8(c)))
    }
}

impl<DI, SIZE> Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: display_interface::AsyncWriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    async fn send_commands_async(&mut self, commands: &[Command]) -> Result<(), DisplayError> {
        for command in commands {
            command.send_async(&mut self.interface).await?;
        }

        Ok(())
    }

    /// Initialise the display in one of the available addressing modes.
    pub async fn init_with_addr_mode_async(&mut self, mode: AddrMode) -> Result<(), DisplayError> {
        self.send_commands_async(&self.init_commands(mode)).await?;
        self.addr_mode = mode;

        Ok(())
    }

    /// Change the addressing mode
    pub async fn set_addr_mode_async(&mut self, mode: AddrMode) -> Result<(), DisplayError> {
        self.send_commands_async(&[Command::AddressMode(mode)])
            .await?;
        self.addr_mode = mode;

        Ok(())
    }

    /// Send the data to the display for drawing at the current position in the framebuffer
    /// and advance the position accordingly. Cf. `set_draw_area` to modify the affected area by
    /// this method.
    ///
    /// This method takes advantage of a bounding box for faster writes.
    pub async fn bounded_draw_async(
        &mut self,
        buffer: &[u8],
        disp_width: usize,
        upper_left: (u8, u8),
        lower_right: (u8, u8),
    ) -> Result<(), DisplayError> {
        Self::flush_buffer_chunks_async(
            &mut self.interface,
            buffer,
            disp_width,
            upper_left,
            lower_right,
        )
        .await
    }

    /// Send a raw buffer to the display.
    pub async fn draw_async(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.interface.send_data(U8(buffer)).await
    }

    /// Set the display rotation.
    pub async fn set_rotation_async(
        &mut self,
        rotation: DisplayRotation,
    ) -> Result<(), DisplayError> {
        self.send_commands_async(&self.rotation_commands(rotation))
            .await?;
        self.rotation = rotation;

        Ok(())
    }

    /// Set mirror enabled/disabled.
    pub async fn set_mirror_async(&mut self, mirror: bool) -> Result<(), DisplayError> {
        self.send_commands_async(&self.mirror_commands(mirror))
            .await
    }

    /// Change the display brightness.
    pub async fn set_brightness_async(
        &mut self,
        brightness: Brightness,
    ) -> Result<(), DisplayError> {
        self.send_commands_async(&self.brightness_commands(brightness))
            .await
    }

    /// Turn the display on or off. The display can be drawn to and retains all
    /// of its memory even while off.
    pub async fn set_display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_commands_async(&[Command::DisplayOn(on)]).await
    }

    /// Set the position in the framebuffer of the display limiting where any sent data should be
    /// drawn. This method can be used for changing the affected area on the screen as well
    /// as (re-)setting the start point of the next `draw` call.
    pub async fn set_draw_area_async(
        &mut self,
        start: (u8, u8),
        end: (u8, u8),
    ) -> Result<(), DisplayError> {
        self.send_commands_async(&self.draw_area_commands(start, end))
            .await
    }

    /// Set the column address in the framebuffer of the display where any sent data should be
    /// drawn.
    pub async fn set_column_async(&mut self, column: u8) -> Result<(), DisplayError> {
        self.send_commands_async(&[Command::ColStart(column)]).await
    }

    /// Set the page address (row 8px high) in the framebuffer of the display where any sent data
    /// should be drawn.
    ///
    /// Note that the parameter is in pixels, but the page will be set to the start of the 8px
    /// row which contains the passed-in row.
    pub async fn set_row_async(&mut self, row: u8) -> Result<(), DisplayError> {
        self.send_commands_async(&[Command::PageStart(row.into())])
            .await
    }

    async fn flush_buffer_chunks_async(
        interface: &mut DI,
        buffer: &[u8],
        disp_width: usize,
        upper_left: (u8, u8),
        lower_right: (u8, u8),
    ) -> Result<(), DisplayError> {
        for chunk in Self::buffer_chunks(buffer, disp_width, upper_left, lower_right) {
            interface.send_data(U8(chunk)).await?;
        }

        Ok(())
    }
}

// SPI-only reset
impl<DI, SIZE, MODE> Ssd1306<DI, SIZE, MODE> {
    /// Reset the display.
    pub fn reset<RST, DELAY>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), Error<Infallible, RST::Error>>
    where
        RST: OutputPin,
        DELAY: DelayNs,
    {
        fn inner_reset<RST, DELAY>(rst: &mut RST, delay: &mut DELAY) -> Result<(), RST::Error>
        where
            RST: OutputPin,
            DELAY: DelayNs,
        {
            rst.set_high()?;
            delay.delay_ms(1);
            rst.set_low()?;
            delay.delay_ms(10);
            rst.set_high()
        }

        inner_reset(rst, delay).map_err(Error::Pin)
    }
}

// SPI-only reset
impl<DI, SIZE, MODE> Ssd1306<DI, SIZE, MODE> {
    /// Reset the display.
    pub async fn reset_async<RST, DELAY>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), Error<Infallible, RST::Error>>
    where
        RST: OutputPin,
        DELAY: embedded_hal_async::delay::DelayNs,
    {
        async fn inner_reset_async<RST, DELAY>(
            rst: &mut RST,
            delay: &mut DELAY,
        ) -> Result<(), RST::Error>
        where
            RST: OutputPin,
            DELAY: embedded_hal_async::delay::DelayNs,
        {
            rst.set_high()?;
            delay.delay_ms(1).await;
            rst.set_low()?;
            delay.delay_ms(10).await;
            rst.set_high()
        }

        inner_reset_async(rst, delay).await.map_err(Error::Pin)
    }
}
