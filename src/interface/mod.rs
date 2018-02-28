pub mod i2c;
pub mod spi;

// TODO: Implement me
pub trait DisplayInterface {
	fn cmds(&mut self, cmds: &[u8]);
	fn flush(&mut self, buf: &[u8]);
	fn init(&mut self);
}

pub use self::i2c::I2cInterface;
pub use self::spi::SpiInterface;