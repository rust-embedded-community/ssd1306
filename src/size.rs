//! Display size.

use super::command::Command;
#[cfg(feature = "async")]
use super::command::CommandAsync;
#[cfg(feature = "async")]
use display_interface::AsyncWriteOnlyDataCommand;
use display_interface::{DisplayError, WriteOnlyDataCommand};

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
#[maybe_async_cfg::maybe(
    sync(keep_self),
    async(
        feature = "async",
        idents(WriteOnlyDataCommand(async = "AsyncWriteOnlyDataCommand"))
    )
)]
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

    /// Size of framebuffer. Because the display is monochrome, this is
    /// width * height / 8
    type Buffer: AsMut<[u8]> + NewZeroed;

    /// Send resolution and model-dependent configuration to the display
    ///
    /// See [`Command::ComPinConfig`]
    /// and [`Command::InternalIref`]
    /// for more information
    async fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError>;
}

maybe_async_cfg::content! {
#![maybe_async_cfg::default(
        idents(
                WriteOnlyDataCommand(sync, async = "AsyncWriteOnlyDataCommand"),
                Command(sync, async = "CommandAsync"),
                DisplaySize(sync, async="DisplaySizeAsync")
        )
)]

/// Size information for the common 128x64 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize128x64;
#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async", keep_self))]
impl DisplaySize for DisplaySize128x64 {
    const WIDTH: u8 = 128;
    const HEIGHT: u8 = 64;
    type Buffer = [u8; <Self as DisplaySize>::WIDTH as usize *
        <Self as DisplaySize>::HEIGHT as usize / 8];

    async fn configure(
        &self,
        iface: &mut impl WriteOnlyDataCommand,
    ) -> Result<(), DisplayError> {
        Command::ComPinConfig(true, false).send(iface).await
    }
}

/// Size information for the common 128x32 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize128x32;
#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async", keep_self))]
impl DisplaySize for DisplaySize128x32 {
    const WIDTH: u8 = 128;
    const HEIGHT: u8 = 32;
    type Buffer = [u8; <Self as DisplaySize>::WIDTH as usize *
        <Self as DisplaySize>::HEIGHT as usize / 8];

    async fn configure(
        &self,
        iface: &mut impl WriteOnlyDataCommand,
    ) -> Result<(), DisplayError> {
        Command::ComPinConfig(false, false).send(iface).await
    }
}

/// Size information for the common 96x16 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize96x16;
#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async", keep_self))]
impl DisplaySize for DisplaySize96x16 {
    const WIDTH: u8 = 96;
    const HEIGHT: u8 = 16;
    type Buffer = [u8; <Self as DisplaySize>::WIDTH as usize *
        <Self as DisplaySize>::HEIGHT as usize / 8];

    async fn configure(
        &self,
        iface: &mut impl WriteOnlyDataCommand,
    ) -> Result<(), DisplayError> {
        Command::ComPinConfig(false, false).send(iface).await
    }
}

/// Size information for the common 72x40 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize72x40;
#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async", keep_self))]
impl DisplaySize for DisplaySize72x40 {
    const WIDTH: u8 = 72;
    const HEIGHT: u8 = 40;
    const OFFSETX: u8 = 28;
    const OFFSETY: u8 = 0;
    type Buffer = [u8; <Self as DisplaySize>::WIDTH as usize *
        <Self as DisplaySize>::HEIGHT as usize / 8];

    async fn configure(
        &self,
        iface: &mut impl WriteOnlyDataCommand,
    ) -> Result<(), DisplayError> {
        Command::ComPinConfig(true, false).send(iface).await?;
        Command::InternalIref(true, true).send(iface).await
    }
}

/// Size information for the common 64x48 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize64x48;
#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async", keep_self))]
impl DisplaySize for DisplaySize64x48 {
    const WIDTH: u8 = 64;
    const HEIGHT: u8 = 48;
    const OFFSETX: u8 = 32;
    const OFFSETY: u8 = 0;
    type Buffer = [u8; <Self as DisplaySize>::WIDTH as usize *
        <Self as DisplaySize>::HEIGHT as usize / 8];

    async fn configure(
        &self,
        iface: &mut impl WriteOnlyDataCommand,
    ) -> Result<(), DisplayError> {
        Command::ComPinConfig(true, false).send(iface).await
    }
}

/// Size information for the common 64x32 variants
#[derive(Debug, Copy, Clone)]
pub struct DisplaySize64x32;
#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async", keep_self))]
impl DisplaySize for DisplaySize64x32 {
    const WIDTH: u8 = 64;
    const HEIGHT: u8 = 32;
    const OFFSETX: u8 = 32;
    const OFFSETY: u8 = 0;
    type Buffer = [u8; <Self as DisplaySize>::WIDTH as usize *
        <Self as DisplaySize>::HEIGHT as usize / 8];

    async fn configure(
        &self,
        iface: &mut impl WriteOnlyDataCommand,
    ) -> Result<(), DisplayError> {
        Command::ComPinConfig(true, false).send(iface).await
    }
}

} // content
