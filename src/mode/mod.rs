//! Abstraction of different operating modes for the SSD1306

pub mod terminal;
pub mod displaymode;
pub mod graphics;
pub mod raw;

pub use self::terminal::TerminalMode;
pub use self::graphics::GraphicsMode;
pub use self::raw::RawMode;
