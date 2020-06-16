//! Crate prelude

pub use display_interface::WriteOnlyDataCommand;
pub use display_interface_i2c::I2CInterface;
pub use display_interface_spi::{SPIInterface, SPIInterfaceNoCS};

pub use super::{
    brightness::Brightness,
    displayrotation::DisplayRotation,
    displaysize::{
        DisplaySize128x32, DisplaySize128x64, DisplaySize64x48, DisplaySize72x40, DisplaySize96x16,
    },
    mode::{displaymode::DisplayModeTrait, GraphicsMode, TerminalMode},
};
