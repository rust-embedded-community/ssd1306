# SSD1306 driver

[![Build Status](https://circleci.com/gh/jamwaffles/ssd1306/tree/master.svg?style=shield)](https://circleci.com/gh/jamwaffles/ssd1306/tree/master)
[![Crates.io](https://img.shields.io/crates/v/ssd1306.svg)](https://crates.io/crates/ssd1306)
[![Docs.rs](https://docs.rs/ssd1306/badge.svg)](https://docs.rs/ssd1306)

[![CRIUS display showing the Rust logo](readme_banner.jpg?raw=true)](examples/image_i2c.rs)

I2C and SPI (4 wire) driver for the SSD1306 OLED display.

See the [announcement blog post](https://wapl.es/electronics/rust/2018/04/30/ssd1306-driver.html) for more information.

Please consider [becoming a sponsor](https://github.com/sponsors/jamwaffles/) so I may continue to maintain this crate in my spare time!

## [Documentation](https://docs.rs/ssd1306)

## [Changelog](CHANGELOG.md)

## [Examples](examples)

From [`examples/image_i2c.rs`](examples/image_i2c.rs):

```rust
#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use panic_halt as _;
use ssd1306::{prelude::*, Builder};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64, 64);

    let im = Image::new(&raw, Point::new(32, 0));

    im.draw(&mut disp).unwrap();

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
