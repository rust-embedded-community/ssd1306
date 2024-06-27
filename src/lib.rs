//! SSD1306 OLED display driver.
//!
//! This crate provides a driver interface to the popular SSD1306 monochrome OLED display driver. It
//! supports I2C and SPI via the [`display_interface`](https://docs.rs/display_interface) crate.
//!
//! The main driver is created using [`Ssd1306::new`] which accepts an interface instance, display
//! size, rotation and mode. The following display modes are supported:
//!
//! - [`BasicMode`] - A simple mode with lower level methods available.
//! - [`BufferedGraphicsMode`] - A framebuffered mode with additional methods and integration with
//!   [embedded-graphics](https://docs.rs/embedded-graphics).
//! - [`TerminalMode`] - A bufferless mode supporting drawing text to the display, as well as
//!   setting cursor positions like a simple terminal.
//!
//! # Examples
//!
//! Examples can be found in [the examples/
//! folder](https://github.com/rust-embedded-community/ssd1306/blob/master/examples)
//!
//! ## Draw some text to the display
//!
//! Uses [`BufferedGraphicsMode`] and [embedded_graphics](https://docs.rs/embedded-graphics). [See
//! the complete example
//! here](https://github.com/rust-embedded-community/ssd1306/blob/master/examples/text_i2c.rs).
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
//! here](https://github.com/rust-embedded-community/ssd1306/blob/master/examples/terminal_i2c.rs).
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
//! [featureset]: https://github.com/rust-embedded-community/embedded-graphics#features
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

use core::convert::Infallible;

pub use crate::i2c_interface::I2CDisplayInterface;
use crate::mode::BasicMode;
use brightness::Brightness;
#[cfg(feature = "async")]
use command::CommandAsync;
use command::{AddrMode, Command, VcomhLevel};
#[cfg(feature = "async")]
use display_interface::AsyncWriteOnlyDataCommand;
use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use embedded_hal::{delay::DelayNs, digital::OutputPin};
#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs as DelayNsAsync;
use error::Error;
use mode::{BufferedGraphicsMode, TerminalMode};
#[cfg(feature = "async")]
use mode::{BufferedGraphicsModeAsync, TerminalModeAsync};
use rotation::DisplayRotation;
use size::DisplaySize;
#[cfg(feature = "async")]
use size::DisplaySizeAsync;

/// SSD1306 driver.
///
/// Note that some methods are only available when the display is configured in a certain [`mode`].
#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async"))]
#[derive(Copy, Clone, Debug)]
pub struct Ssd1306<DI, SIZE, MODE> {
    interface: DI,
    mode: MODE,
    size: SIZE,
    addr_mode: AddrMode,
    rotation: DisplayRotation,
}

#[maybe_async_cfg::maybe(
    sync(keep_self,),
    async(feature = "async", idents(DisplaySize(async = "DisplaySizeAsync")))
)]
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

#[maybe_async_cfg::maybe(
    sync(keep_self,),
    async(
        feature = "async",
        idents(
            DisplaySize(async = "DisplaySizeAsync"),
            BufferedGraphicsMode(async = "BufferedGraphicsModeAsync"),
            TerminalMode(async = "TerminalModeAsync"),
        )
    )
)]
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
}

#[maybe_async_cfg::maybe(
    sync(keep_self),
    async(
        feature = "async",
        idents(
            Command(async = "CommandAsync"),
            DisplaySize(async = "DisplaySizeAsync"),
            WriteOnlyDataCommand(async = "AsyncWriteOnlyDataCommand"),
        )
    )
)]
impl<DI, SIZE, MODE> Ssd1306<DI, SIZE, MODE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    /// Initialise the display in one of the available addressing modes.
    pub async fn init_with_addr_mode(&mut self, mode: AddrMode) -> Result<(), DisplayError> {
        let rotation = self.rotation;

        Command::DisplayOn(false).send(&mut self.interface).await?;
        Command::DisplayClockDiv(0x8, 0x0)
            .send(&mut self.interface)
            .await?;
        Command::Multiplex(SIZE::HEIGHT - 1)
            .send(&mut self.interface)
            .await?;
        Command::DisplayOffset(0).send(&mut self.interface).await?;
        Command::StartLine(0).send(&mut self.interface).await?;
        // TODO: Ability to turn charge pump on/off
        Command::ChargePump(true).send(&mut self.interface).await?;
        Command::AddressMode(mode).send(&mut self.interface).await?;

        self.size.configure(&mut self.interface).await?;
        self.set_rotation(rotation).await?;

        self.set_brightness(Brightness::default()).await?;
        Command::VcomhDeselect(VcomhLevel::Auto)
            .send(&mut self.interface)
            .await?;
        Command::AllOn(false).send(&mut self.interface).await?;
        Command::Invert(false).send(&mut self.interface).await?;
        Command::EnableScroll(false)
            .send(&mut self.interface)
            .await?;
        Command::DisplayOn(true).send(&mut self.interface).await?;

        self.addr_mode = mode;

        Ok(())
    }

    /// Change the addressing mode
    pub async fn set_addr_mode(&mut self, mode: AddrMode) -> Result<(), DisplayError> {
        Command::AddressMode(mode).send(&mut self.interface).await?;
        self.addr_mode = mode;
        Ok(())
    }

    /// Send the data to the display for drawing at the current position in the framebuffer
    /// and advance the position accordingly. Cf. `set_draw_area` to modify the affected area by
    /// this method.
    ///
    /// This method takes advantage of a bounding box for faster writes.
    pub async fn bounded_draw(
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
        .await
    }

    /// Send a raw buffer to the display.
    pub async fn draw(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.interface.send_data(U8(buffer)).await
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

    /// Set the display rotation.
    pub async fn set_rotation(&mut self, rotation: DisplayRotation) -> Result<(), DisplayError> {
        self.rotation = rotation;

        match rotation {
            DisplayRotation::Rotate0 => {
                Command::SegmentRemap(true)
                    .send(&mut self.interface)
                    .await?;
                Command::ReverseComDir(true)
                    .send(&mut self.interface)
                    .await?;
            }
            DisplayRotation::Rotate90 => {
                Command::SegmentRemap(false)
                    .send(&mut self.interface)
                    .await?;
                Command::ReverseComDir(true)
                    .send(&mut self.interface)
                    .await?;
            }
            DisplayRotation::Rotate180 => {
                Command::SegmentRemap(false)
                    .send(&mut self.interface)
                    .await?;
                Command::ReverseComDir(false)
                    .send(&mut self.interface)
                    .await?;
            }
            DisplayRotation::Rotate270 => {
                Command::SegmentRemap(true)
                    .send(&mut self.interface)
                    .await?;
                Command::ReverseComDir(false)
                    .send(&mut self.interface)
                    .await?;
            }
        };

        Ok(())
    }

    /// Set mirror enabled/disabled.
    pub async fn set_mirror(&mut self, mirror: bool) -> Result<(), DisplayError> {
        if mirror {
            match self.rotation {
                DisplayRotation::Rotate0 => {
                    Command::SegmentRemap(false)
                        .send(&mut self.interface)
                        .await?;
                    Command::ReverseComDir(true)
                        .send(&mut self.interface)
                        .await?;
                }
                DisplayRotation::Rotate90 => {
                    Command::SegmentRemap(false)
                        .send(&mut self.interface)
                        .await?;
                    Command::ReverseComDir(false)
                        .send(&mut self.interface)
                        .await?;
                }
                DisplayRotation::Rotate180 => {
                    Command::SegmentRemap(true)
                        .send(&mut self.interface)
                        .await?;
                    Command::ReverseComDir(false)
                        .send(&mut self.interface)
                        .await?;
                }
                DisplayRotation::Rotate270 => {
                    Command::SegmentRemap(true)
                        .send(&mut self.interface)
                        .await?;
                    Command::ReverseComDir(true)
                        .send(&mut self.interface)
                        .await?;
                }
            };
        } else {
            self.set_rotation(self.rotation).await?;
        }
        Ok(())
    }

    /// Change the display brightness.
    pub async fn set_brightness(&mut self, brightness: Brightness) -> Result<(), DisplayError> {
        Command::PreChargePeriod(1, brightness.precharge)
            .send(&mut self.interface)
            .await?;
        Command::Contrast(brightness.contrast)
            .send(&mut self.interface)
            .await
    }

    /// Turn the display on or off. The display can be drawn to and retains all
    /// of its memory even while off.
    pub async fn set_display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        Command::DisplayOn(on).send(&mut self.interface).await
    }

    /// Set the position in the framebuffer of the display limiting where any sent data should be
    /// drawn. This method can be used for changing the affected area on the screen as well
    /// as (re-)setting the start point of the next `draw` call.
    pub async fn set_draw_area(
        &mut self,
        start: (u8, u8),
        end: (u8, u8),
    ) -> Result<(), DisplayError> {
        Command::ColumnAddress(start.0, end.0.saturating_sub(1))
            .send(&mut self.interface)
            .await?;

        if self.addr_mode != AddrMode::Page {
            Command::PageAddress(start.1.into(), (end.1.saturating_sub(1)).into())
                .send(&mut self.interface)
                .await?;
        }

        Ok(())
    }

    /// Set the column address in the framebuffer of the display where any sent data should be
    /// drawn.
    pub async fn set_column(&mut self, column: u8) -> Result<(), DisplayError> {
        Command::ColStart(column).send(&mut self.interface).await
    }

    /// Set the page address (row 8px high) in the framebuffer of the display where any sent data
    /// should be drawn.
    ///
    /// Note that the parameter is in pixels, but the page will be set to the start of the 8px
    /// row which contains the passed-in row.
    pub async fn set_row(&mut self, row: u8) -> Result<(), DisplayError> {
        Command::PageStart(row.into())
            .send(&mut self.interface)
            .await
    }

    /// Set the screen pixel on/off inversion
    pub async fn set_invert(&mut self, invert: bool) -> Result<(), DisplayError> {
        Command::Invert(invert).send(&mut self.interface).await
    }

    async fn flush_buffer_chunks(
        interface: &mut DI,
        buffer: &[u8],
        disp_width: usize,
        upper_left: (u8, u8),
        lower_right: (u8, u8),
    ) -> Result<(), DisplayError> {
        // Divide by 8 since each row is actually 8 pixels tall
        let num_pages = ((lower_right.1 - upper_left.1) / 8) as usize + 1;

        // Each page is 8 bits tall, so calculate which page number to start at (rounded down) from
        // the top of the display
        let starting_page = (upper_left.1 / 8) as usize;

        // Calculate start and end X coordinates for each page
        let page_lower = upper_left.0 as usize;
        let page_upper = lower_right.0 as usize;

        for c in buffer
            .chunks(disp_width)
            .skip(starting_page)
            .take(num_pages)
            .map(|s| &s[page_lower..page_upper])
        {
            interface.send_data(U8(c)).await?
        }
        Ok(())
    }

    /// Release the contained interface.
    pub fn release(self) -> DI {
        self.interface
    }
}

// SPI-only reset
#[maybe_async_cfg::maybe(
    sync(keep_self),
    async(
        feature = "async",
        idents(
            Command(async = "CommandAsync"),
            DisplaySize(async = "DisplaySizeAsync"),
            WriteOnlyDataCommand(async = "AsyncWriteOnlyDataCommand"),
            DelayNs(async = "DelayNsAsync")
        )
    )
)]
impl<DI, SIZE, MODE> Ssd1306<DI, SIZE, MODE> {
    /// Reset the display.
    pub async fn reset<RST, DELAY>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), Error<Infallible, RST::Error>>
    where
        RST: OutputPin,
        DELAY: DelayNs,
    {
        async fn inner_reset<RST, DELAY>(rst: &mut RST, delay: &mut DELAY) -> Result<(), RST::Error>
        where
            RST: OutputPin,
            DELAY: DelayNs,
        {
            rst.set_high()?;
            delay.delay_ms(1).await;
            rst.set_low()?;
            delay.delay_ms(10).await;
            rst.set_high()
        }

        inner_reset(rst, delay).await.map_err(Error::Pin)
    }
}
