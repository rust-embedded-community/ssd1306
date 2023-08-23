//! Helpers for use in examples and tests

use core::convert::Infallible;

use display_interface::{DisplayError, WriteOnlyDataCommand};
use embedded_hal::{
    digital::{self, OutputPin},
    i2c, spi,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SpiStub;

impl spi::ErrorType for SpiStub {
    type Error = Infallible;
}

impl spi::SpiDevice<u8> for SpiStub {
    fn transaction(
        &mut self,
        _operations: &mut [spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct I2cStub;

impl i2c::ErrorType for I2cStub {
    type Error = Infallible;
}

impl i2c::I2c for I2cStub {
    fn write(&mut self, _addr: u8, _buf: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct PinStub;

impl digital::ErrorType for PinStub {
    type Error = Infallible;
}

impl OutputPin for PinStub {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct StubInterface;

impl WriteOnlyDataCommand for StubInterface {
    fn send_commands(
        &mut self,
        _cmd: display_interface::DataFormat<'_>,
    ) -> Result<(), DisplayError> {
        Ok(())
    }
    fn send_data(&mut self, _buf: display_interface::DataFormat<'_>) -> Result<(), DisplayError> {
        Ok(())
    }
}
