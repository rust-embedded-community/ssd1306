//! Crate prelude

pub use display_interface::WriteOnlyDataCommand;
pub use display_interface_i2c::I2CInterface;
pub use display_interface_spi::{SPIInterface, SPIInterfaceNoCS};

pub use super::{
    displayrotation::DisplayRotation,
    displaysize::DisplaySize,
    mode::{GraphicsMode, TerminalMode},
};
