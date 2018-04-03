//! Buffered display module for use with the embedded_graphics crate

use displaysize::DisplaySize;
use displayrotation::DisplayRotation;

use command::{AddrMode, Command, VcomhLevel};

use hal::blocking::delay::DelayMs;
use hal::digital::OutputPin;
use interface::DisplayInterface;
use properties::DisplayProperties;

use mode::displaymode::DisplayModeTrait;

/// GraphicsMode
pub struct GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    properties: DisplayProperties<DI>,
    buffer: [u8; 1024],
}

impl<DI> DisplayModeTrait<DI> for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    /// Create new GraphicsMode instance
    fn new(properties: DisplayProperties<DI>) -> Self {
        GraphicsMode {
            properties,
            buffer: [0; 1024],
        }
    }

    /// Release all resources used by GraphicsMode
    fn release(self) -> DisplayProperties<DI> {
        self.properties
    }
}

impl<DI> GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    /// Clear the display buffer. You need to call `disp.flush()` for any effect on the screen
    pub fn clear(&mut self) {
        self.buffer = [0; 1024];
    }

    /// Reset display
    pub fn reset<RST, DELAY>(&mut self, rst: &mut RST, delay: &mut DELAY)
    where
        RST: OutputPin,
        DELAY: DelayMs<u8>,
    {
        rst.set_high();
        delay.delay_ms(1);
        rst.set_low();
        delay.delay_ms(10);
        rst.set_high();
    }

    /// Write out data to display
    pub fn flush(&mut self) -> Result<(), DI::Error> {
        let display_size = self.properties.get_size();
        let iface = self.properties.borrow_iface_mut();

        let (display_width, display_height) = display_size.dimensions();

        Command::ColumnAddress(0, display_width - 1).send(iface)?;
        Command::PageAddress(0.into(), (display_height - 1).into()).send(iface)?;

        match display_size {
            DisplaySize::Display128x64 => iface.send_data(&self.buffer),
            DisplaySize::Display128x32 => iface.send_data(&self.buffer[0..512]),
            DisplaySize::Display96x16 => iface.send_data(&self.buffer[0..192]),
        }
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let (display_width, _) = self.properties.get_size().dimensions();
        let display_rotation = self.properties.get_rotation();

        let idx = match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                ((y as usize) / 8 * display_width as usize) + (x as usize)
            }

            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                ((x as usize) / 8 * display_width as usize) + (y as usize)
            }
        };

        if idx >= self.buffer.len() {
            return;
        }

        let (byte, bit) = match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                let byte =
                    &mut self.buffer[((y as usize) / 8 * display_width as usize) + (x as usize)];
                let bit = 1 << (y % 8);

                (byte, bit)
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                let byte =
                    &mut self.buffer[((x as usize) / 8 * display_width as usize) + (y as usize)];
                let bit = 1 << (x % 8);

                (byte, bit)
            }
        };

        if value == 0 {
            *byte &= !bit;
        } else {
            *byte |= bit;
        }
    }

    // Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from column 0 on the left, to column _n_ on the right
    /// Initialize display in column mode.
    pub fn init(&mut self) -> Result<(), DI::Error> {
        let display_size = self.properties.get_size();
        let display_rotation = self.properties.get_rotation();
        let (_, display_height) = display_size.dimensions();

        {
            let iface = self.properties.borrow_iface_mut();
            Command::DisplayOn(false).send(iface)?;
            Command::DisplayClockDiv(0x8, 0x0).send(iface)?;
            Command::Multiplex(display_height - 1).send(iface)?;
            Command::DisplayOffset(0).send(iface)?;
            Command::StartLine(0).send(iface)?;
            // TODO: Ability to turn charge pump on/off
            Command::ChargePump(true).send(iface)?;
            Command::AddressMode(AddrMode::Horizontal).send(iface)?;
        }

        self.set_rotation(display_rotation)?;

        let iface = self.properties.borrow_iface_mut();
        match display_size {
            DisplaySize::Display128x32 => Command::ComPinConfig(false, false).send(iface),
            DisplaySize::Display128x64 => Command::ComPinConfig(true, false).send(iface),
            DisplaySize::Display96x16 => Command::ComPinConfig(false, false).send(iface),
        }?;

        Command::Contrast(0x8F).send(iface)?;
        Command::PreChargePeriod(0x1, 0xF).send(iface)?;
        Command::VcomhDeselect(VcomhLevel::Auto).send(iface)?;
        Command::AllOn(false).send(iface)?;
        Command::Invert(false).send(iface)?;
        Command::EnableScroll(false).send(iface)?;
        Command::DisplayOn(true).send(iface)?;

        Ok(())
    }

    /// Get display dimensions, taking into account the current rotation of the display
    // TODO: Replace (u8, u8) with a dimensioney type for consistency
    pub fn get_dimensions(&self) -> (u8, u8) {
        let (w, h) = self.properties.get_size().dimensions();
        let display_rotation = self.properties.get_rotation();

        match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (w, h),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (h, w),
        }
    }

    /// Set the display rotation
    pub fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DI::Error> {
        self.properties.set_rotation(rot)
    }
}

#[cfg(feature = "graphics")]
extern crate embedded_graphics;
#[cfg(feature = "graphics")]
use self::embedded_graphics::drawable;
#[cfg(feature = "graphics")]
use self::embedded_graphics::Drawing;

#[cfg(feature = "graphics")]
impl<DI> Drawing for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = drawable::Pixel>,
    {
        for (pos, color) in item_pixels {
            self.set_pixel(pos.0, pos.1, color);
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO lol
}
