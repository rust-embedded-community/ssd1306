//! Draw a 1 bit per pixel black and white rust logo to 128x64 SSD1306 displays over both I2C
//! and SPI. This uses async an approach to transfer to i2c and spi simultaniously using DMA
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1 and SPI1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//! I2c Display -> Blue Pill
//! GND -> GND
//! +5V -> VCC
//! SDA -> PB7
//! SCL -> PB6
//!
//! SPI display -> Blue Pill
//! GND -> GND
//! 3V3 -> VCC
//! PA5 -> SCL (D0)
//! PA7 -> SDA (D1)
//! PB0 -> RST
//! PB1 -> D/C
//! PB10 -> CS
//! ```
//!
//! Run on a Blue Pill with `cargo run --example async_i2c_spi`.

#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::{bind_interrupts, gpio, i2c, peripherals, spi::Spi, time::Hertz, Config};
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use panic_probe as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306Async};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config: Config = Default::default();
    config.rcc.hse = Some(embassy_stm32::rcc::Hse {
        freq: Hertz::mhz(8),
        mode: embassy_stm32::rcc::HseMode::Oscillator,
    });
    config.rcc.sys = embassy_stm32::rcc::Sysclk::PLL1_P;
    config.rcc.pll = Some(embassy_stm32::rcc::Pll {
        src: embassy_stm32::rcc::PllSource::HSE,
        prediv: embassy_stm32::rcc::PllPreDiv::DIV1,
        mul: embassy_stm32::rcc::PllMul::MUL9, // 8 * 9 = 72Mhz
    });

    // Scale down to 36Mhz (maximum allowed)
    config.rcc.apb1_pre = embassy_stm32::rcc::APBPrescaler::DIV2;
    let p = embassy_stm32::init(config);

    // I2C
    let i2c = embassy_stm32::i2c::I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH7,
        // According to the datasheet the stm32f1xx only supports up to 400khz, but 1mhz seems to
        // work just fine. WHen having issues try changing this to Hertz::khz(400).
        Hertz::mhz(1),
        Default::default(),
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display_i2c = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // SPI
    let spi = Spi::new_txonly(p.SPI1, p.PA5, p.PA7, p.DMA1_CH3, Default::default());

    let mut rst = gpio::Output::new(p.PB0, gpio::Level::Low, gpio::Speed::Low);
    let dc = gpio::Output::new(p.PB1, gpio::Level::Low, gpio::Speed::Low);
    let cs = gpio::Output::new(p.PB10, gpio::Level::Low, gpio::Speed::Low);
    let spi = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let interface = SPIInterface::new(spi, dc);
    let mut display_spi = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // Init and reset both displays as needed
    join(
        async {
            display_i2c.init().await.unwrap();
        },
        async {
            display_spi
                .reset(&mut rst, &mut embassy_time::Delay {})
                .await
                .unwrap();
            display_spi.init().await.unwrap();
        },
    )
    .await;

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);

    for i in (0..=64).chain((0..64).rev()).cycle() {
        let top_left = Point::new(i, 0);
        let im = Image::new(&raw, top_left);

        im.draw(&mut display_i2c).unwrap();
        im.draw(&mut display_spi).unwrap();

        join(async { display_i2c.flush().await.unwrap() }, async {
            display_spi.flush().await.unwrap()
        })
        .await;

        display_i2c.clear(BinaryColor::Off).unwrap();
        display_spi.clear(BinaryColor::Off).unwrap();
    }
}
