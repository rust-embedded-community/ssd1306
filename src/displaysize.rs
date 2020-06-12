//! Display size

use super::command::Command;
use generic_array::ArrayLength;
use typenum::{U1024, U192, U360, U384, U512};

pub trait DisplaySize {
    const WIDTH: u8;
    const HEIGHT: u8;
    const OFFSETX: u8 = 0;
    const OFFSETY: u8 = 0;
    type BufferSize: ArrayLength<u8>;

    fn com_pin_config() -> Command;
}

pub struct DisplaySize128x64;
impl DisplaySize for DisplaySize128x64 {
    const WIDTH: u8 = 128;
    const HEIGHT: u8 = 64;
    type BufferSize = U1024;

    fn com_pin_config() -> Command {
        Command::ComPinConfig(true, false)
    }
}

pub struct DisplaySize128x32;
impl DisplaySize for DisplaySize128x32 {
    const WIDTH: u8 = 128;
    const HEIGHT: u8 = 32;
    type BufferSize = U512;

    fn com_pin_config() -> Command {
        Command::ComPinConfig(false, false)
    }
}

pub struct DisplaySize96x16;
impl DisplaySize for DisplaySize96x16 {
    const WIDTH: u8 = 96;
    const HEIGHT: u8 = 16;
    type BufferSize = U192;

    fn com_pin_config() -> Command {
        Command::ComPinConfig(false, false)
    }
}

pub struct DisplaySize72x40;
impl DisplaySize for DisplaySize72x40 {
    const WIDTH: u8 = 72;
    const HEIGHT: u8 = 40;
    const OFFSETX: u8 = 28;
    const OFFSETY: u8 = 0;
    type BufferSize = U360;

    fn com_pin_config() -> Command {
        Command::ComPinConfig(true, false)
    }
}

pub struct DisplaySize64x48;
impl DisplaySize for DisplaySize64x48 {
    const WIDTH: u8 = 64;
    const HEIGHT: u8 = 48;
    const OFFSETX: u8 = 32;
    const OFFSETY: u8 = 0;
    type BufferSize = U384;

    fn com_pin_config() -> Command {
        Command::ComPinConfig(true, false)
    }
}
