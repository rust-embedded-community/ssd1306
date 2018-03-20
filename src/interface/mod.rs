pub mod i2c;
pub mod spi;

pub trait DisplayInterface {
    type Error;

    fn send_command(&mut self, cmd: u8) -> Result<(), Self::Error>;
    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

pub use self::i2c::I2cInterface;
pub use self::spi::SpiInterface;
