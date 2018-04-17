//! Operating modes for the SSD1306
//!
//! This driver can be used in different modes. A mode defines how the driver will behave, and what
//! methods it exposes. Look at the modes below for more information on what they expose.

pub mod displaymode;
pub mod graphics;
pub mod raw;
pub mod terminal;

pub use self::graphics::GraphicsMode;
pub use self::raw::RawMode;
pub use self::terminal::TerminalMode;
