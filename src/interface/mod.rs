pub mod i2c;
pub mod spi;

pub trait DisplayInterface {
    fn send_command(&mut self, cmd: u8);
    fn send_data(&mut self, buf: &[u8]);
}

pub use self::i2c::I2cInterface;
pub use self::spi::SpiInterface;
