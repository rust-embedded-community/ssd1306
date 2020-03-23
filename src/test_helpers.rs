//! Helpers for use in examples and tests

use crate::interface::DisplayInterface;
use embedded_hal::{
    blocking::{
        i2c,
        spi::{self, Transfer},
    },
    digital::v2::OutputPin,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SpiStub;

impl spi::Write<u8> for SpiStub {
    type Error = ();

    fn write(&mut self, _buf: &[u8]) -> Result<(), ()> {
        Ok(())
    }
}

impl Transfer<u8> for SpiStub {
    type Error = ();

    fn transfer<'a>(&mut self, buf: &'a mut [u8]) -> Result<&'a [u8], ()> {
        Ok(buf)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct I2cStub;

impl i2c::Write for I2cStub {
    type Error = ();

    fn write(&mut self, _addr: u8, _buf: &[u8]) -> Result<(), ()> {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct PinStub;

impl OutputPin for PinStub {
    type Error = ();

    fn set_high(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), ()> {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct StubInterface;

impl DisplayInterface for StubInterface {
    type Error = ();

    fn send_commands(&mut self, _cmd: &[u8]) -> Result<(), ()> {
        Ok(())
    }
    fn send_data(&mut self, _buf: &[u8]) -> Result<(), ()> {
        Ok(())
    }
}
