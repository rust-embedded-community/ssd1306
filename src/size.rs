//! Display size.

use super::command::Command;
use crate::Ssd1306Framebuffer;
use display_interface::{DisplayError, WriteOnlyDataCommand};
use embedded_graphics::{framebuffer::buffer_size, pixelcolor::BinaryColor};

/// Workaround trait, since `Default` is only implemented to arrays up to 32 of size
pub trait NewZeroed {
    /// Creates a new value with its memory set to zero
    fn new_zeroed() -> Self;
}

impl<const N: usize> NewZeroed for [u8; N] {
    fn new_zeroed() -> Self {
        [0u8; N]
    }
}

/// Display information.
///
/// This trait describes information related to a particular display.
/// This includes resolution, offset and framebuffer size.
pub trait DisplaySize {
    /// Width in pixels
    const WIDTH: u8;

    /// Height in pixels
    const HEIGHT: u8;

    /// Maximum width supported by the display driver
    const DRIVER_COLS: u8 = 128;

    /// Maximum height supported by the display driver
    const DRIVER_ROWS: u8 = 64;

    /// Horizontal offset in pixels
    const OFFSETX: u8 = 0;

    /// Vertical offset in pixels
    const OFFSETY: u8 = 0;

    /// Pixel data frame buffer.
    type Buffer;

    /// Send resolution and model-dependent configuration to the display
    ///
    /// See [`Command::ComPinConfig`](crate::Command::ComPinConfig)
    /// and [`Command::InternalIref`](crate::Command::InternalIref)
    /// for more information
    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError>;

    /// Create a new instance of [`DisplaySize::Buffer`].
    fn new_buffer() -> Self::Buffer;
}

/// Size information for the common 128x64 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize128x64;
impl DisplaySize for DisplaySize128x64 {
    const WIDTH: u8 = 128;
    const HEIGHT: u8 = 64;

    type Buffer = Ssd1306Framebuffer<
        { Self::WIDTH as usize },
        { Self::HEIGHT as usize },
        { buffer_size::<BinaryColor>(Self::WIDTH as usize, Self::HEIGHT as usize) },
    >;

    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::ComPinConfig(true, false).send(iface)
    }

    fn new_buffer() -> Self::Buffer {
        Self::Buffer::new()
    }
}

/// Size information for the common 128x32 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize128x32;
impl DisplaySize for DisplaySize128x32 {
    const WIDTH: u8 = 128;
    const HEIGHT: u8 = 32;

    type Buffer = Ssd1306Framebuffer<
        { Self::WIDTH as usize },
        { Self::HEIGHT as usize },
        { buffer_size::<BinaryColor>(Self::WIDTH as usize, Self::HEIGHT as usize) },
    >;

    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::ComPinConfig(false, false).send(iface)
    }

    fn new_buffer() -> Self::Buffer {
        Self::Buffer::new()
    }
}

/// Size information for the common 96x16 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize96x16;
impl DisplaySize for DisplaySize96x16 {
    const WIDTH: u8 = 96;
    const HEIGHT: u8 = 16;

    type Buffer = Ssd1306Framebuffer<
        { Self::WIDTH as usize },
        { Self::HEIGHT as usize },
        { buffer_size::<BinaryColor>(Self::WIDTH as usize, Self::HEIGHT as usize) },
    >;

    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::ComPinConfig(false, false).send(iface)
    }

    fn new_buffer() -> Self::Buffer {
        Self::Buffer::new()
    }
}

/// Size information for the common 72x40 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize72x40;
impl DisplaySize for DisplaySize72x40 {
    const WIDTH: u8 = 72;
    const HEIGHT: u8 = 40;
    const OFFSETX: u8 = 28;
    const OFFSETY: u8 = 0;

    type Buffer = Ssd1306Framebuffer<
        { Self::WIDTH as usize },
        { Self::HEIGHT as usize },
        { buffer_size::<BinaryColor>(Self::WIDTH as usize, Self::HEIGHT as usize) },
    >;

    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::ComPinConfig(true, false).send(iface)?;
        Command::InternalIref(true, true).send(iface)
    }

    fn new_buffer() -> Self::Buffer {
        Self::Buffer::new()
    }
}

/// Size information for the common 64x48 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize64x48;
impl DisplaySize for DisplaySize64x48 {
    const WIDTH: u8 = 64;
    const HEIGHT: u8 = 48;
    const OFFSETX: u8 = 32;
    const OFFSETY: u8 = 0;

    type Buffer = Ssd1306Framebuffer<
        { Self::WIDTH as usize },
        { Self::HEIGHT as usize },
        { buffer_size::<BinaryColor>(Self::WIDTH as usize, Self::HEIGHT as usize) },
    >;

    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::ComPinConfig(true, false).send(iface)
    }

    fn new_buffer() -> Self::Buffer {
        Self::Buffer::new()
    }
}

/// Size information for the common 64x32 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize64x32;
impl DisplaySize for DisplaySize64x32 {
    const WIDTH: u8 = 64;
    const HEIGHT: u8 = 32;
    const OFFSETX: u8 = 32;
    const OFFSETY: u8 = 0;

    type Buffer = Ssd1306Framebuffer<
        { Self::WIDTH as usize },
        { Self::HEIGHT as usize },
        { buffer_size::<BinaryColor>(Self::WIDTH as usize, Self::HEIGHT as usize) },
    >;

    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::ComPinConfig(true, false).send(iface)
    }

    fn new_buffer() -> Self::Buffer {
        Self::Buffer::new()
    }
}
