//! SSD1306 OLED display driver

#![no_std]
// TODO: Docs
// #![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

#[cfg(feature = "graphics")]
extern crate embedded_graphics;
extern crate embedded_hal as hal;

mod command;
mod displaysize;
pub mod displayrotation;
pub mod builder;
pub mod interface;

pub use builder::Builder;
pub use displaysize::DisplaySize;
pub use displayrotation::DisplayRotation;
use command::{AddrMode, Command, VcomhLevel};

use hal::blocking::delay::DelayMs;
use hal::digital::OutputPin;
use interface::DisplayInterface;

/// SSD1306
pub struct SSD1306<DI> {
    iface: DI,
    buffer: [u8; 1024],
    display_size: DisplaySize,
    display_rotation: DisplayRotation,
}

impl<DI> SSD1306<DI>
where
    DI: DisplayInterface,
{
    /// Create new SSD1306 instance
    pub fn new(
        iface: DI,
        display_size: DisplaySize,
        display_rotation: DisplayRotation,
    ) -> SSD1306<DI> {
        SSD1306 {
            iface,
            display_size,
            display_rotation,
            buffer: [0; 1024],
        }
    }

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
        let (display_width, display_height) = self.display_size.dimensions();

        Command::ColumnAddress(0, display_width - 1).send(&mut self.iface)?;
        Command::PageAddress(0.into(), (display_height - 1).into()).send(&mut self.iface)?;

        match self.display_size {
            DisplaySize::Display128x64 => self.iface.send_data(&self.buffer),
            DisplaySize::Display128x32 => self.iface.send_data(&self.buffer[0..512]),
            DisplaySize::Display96x16 => self.iface.send_data(&self.buffer[0..192]),
        }
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    //// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let (display_width, _) = self.display_size.dimensions();

        let idx = ((y as usize) / 8 * display_width as usize) + (x as usize);

        if idx < self.buffer.len() {
            let byte = &mut self.buffer[idx];
            let bit = 1 << (y % 8);

            if value == 0 {
                *byte &= !bit;
            } else {
                *byte |= bit;
            }
        }
    }

    // Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from column 0 on the left, to column _n_ on the right
    /// Initialize display in column mode.
    pub fn init(&mut self) -> Result<(), DI::Error> {
        let (_, display_height) = self.display_size.dimensions();

        Command::DisplayOn(false).send(&mut self.iface)?;
        Command::DisplayClockDiv(0x8, 0x0).send(&mut self.iface)?;
        Command::Multiplex(display_height - 1).send(&mut self.iface)?;
        Command::DisplayOffset(0).send(&mut self.iface)?;
        Command::StartLine(0).send(&mut self.iface)?;
        // TODO: Ability to turn charge pump on/off
        Command::ChargePump(true).send(&mut self.iface)?;
        Command::AddressMode(AddrMode::Horizontal).send(&mut self.iface)?;

        match self.display_rotation {
            DisplayRotation::Rotate180 => {
                Command::SegmentRemap(false).send(&mut self.iface)?;
                Command::ReverseComDir(false).send(&mut self.iface)?;
            }
            _ => {
                Command::SegmentRemap(false).send(&mut self.iface)?;
                Command::ReverseComDir(false).send(&mut self.iface)?;
            }
        };

        match self.display_size {
            DisplaySize::Display128x32 => Command::ComPinConfig(false, false).send(&mut self.iface),
            DisplaySize::Display128x64 => Command::ComPinConfig(true, false).send(&mut self.iface),
            DisplaySize::Display96x16 => Command::ComPinConfig(false, false).send(&mut self.iface),
        }?;

        Command::Contrast(0x8F).send(&mut self.iface)?;
        Command::PreChargePeriod(0x1, 0xF).send(&mut self.iface)?;
        Command::VcomhDeselect(VcomhLevel::Auto).send(&mut self.iface)?;
        Command::AllOn(false).send(&mut self.iface)?;
        Command::Invert(false).send(&mut self.iface)?;
        Command::EnableScroll(false).send(&mut self.iface)?;
        Command::DisplayOn(true).send(&mut self.iface)?;

        Ok(())
    }
}

#[cfg(feature = "graphics")]
use embedded_graphics::drawable;
#[cfg(feature = "graphics")]
use embedded_graphics::Drawing;

#[cfg(feature = "graphics")]
impl<DI> Drawing for SSD1306<DI>
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
