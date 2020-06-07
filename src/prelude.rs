//! Crate prelude

pub use display_interface::WriteOnlyDataCommand;
pub use display_interface_i2c::I2CInterface;
pub use display_interface_spi::{SPIInterface, SPIInterfaceNoCS};

pub use super::{
    brightness::Brightness,
    displayrotation::DisplayRotation,
    displaysize::DisplaySize,
    mode::{displaymode::DisplayModeTrait, GraphicsMode, TerminalMode},
};
