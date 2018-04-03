//! Abstraction of different operating modes for the SSD1306

pub mod displaymode;
pub mod graphics;
pub mod raw;

pub use self::graphics::GraphicsMode;
pub use self::raw::RawMode;
