//! Draw a filled rectangle then clear a sub-region of it using `clear_region`.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//!      Display -> Blue Pill
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PB7
//! (green)  SCL -> PB6
//! ```
//!
//! Run on a Blue Pill with `cargo run --example clear_region_i2c`.

#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use defmt_rtt as _;
use embassy_stm32::time::Hertz;
#[cfg(feature = "async")]
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use panic_probe as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    #[cfg(feature = "async")]
    bind_interrupts!(struct Irqs {
        I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
        I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    });

    #[cfg(feature = "async")]
    let i2c = embassy_stm32::i2c::I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH7,
        Hertz::khz(400),
        Default::default(),
    );

    #[cfg(not(feature = "async"))]
    let i2c = embassy_stm32::i2c::I2c::new_blocking(
        p.I2C1,
        p.PB6,
        p.PB7,
        Hertz::khz(400),
        Default::default(),
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let filled_style = PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::On)
        .build();

    // Draw a filled rectangle covering most of the screen.
    Rectangle::new(Point::new(10, 10), Size::new(108, 44))
        .into_styled(filled_style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    // Clear a rectangular sub-region from the framebuffer, then flush only
    // the changed area back to the display.
    display.clear_region(30, 20, 98, 44);
    display.flush().unwrap();

    loop {
        nop()
    }
}
