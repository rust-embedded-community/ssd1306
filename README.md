# SSD1306 driver

[![Build Status](https://travis-ci.org/jamwaffles/ssd1306.svg?branch=master)](https://travis-ci.org/jamwaffles/ssd1306)

[![CRIUS display showing the Rust logo](readme_banner.jpg?raw=true)](examples/image_i2c.rs)

I2C and SPI (4 wire) driver for the SSD1306 OLED display for use with RTFM.

See the [announcement blog post](https://wapl.es/electronics/rust/2018/04/30/ssd1306-driver.html) for more information.

## [Documentation](https://jamwaffles.github.io/ssd1306)

## [Examples](examples)

From [`examples/image_i2c.rs`](examples/image_i2c.rs):

```rust
#![no_std]

extern crate cortex_m;
extern crate embedded_graphics;
extern crate embedded_hal as hal;
extern crate ssd1306;
extern crate stm32f103xx_hal as blue_pill;

use blue_pill::i2c::{DutyCycle, I2c, Mode};
use blue_pill::prelude::*;
use embedded_graphics::Drawing;
use embedded_graphics::image::{Image, Image1BPP};
use embedded_graphics::transform::Transform;
use ssd1306::{Builder, mode::GraphicsMode};

fn main() {
    let dp = blue_pill::stm32f103xx::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = I2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio1to1,
        },
        clocks,
        &mut rcc.apb1,
    );

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();
    disp.flush().unwrap();

    let im = Image1BPP::new(include_bytes!("./rust.raw"), 64, 64).translate((32, 0));

    disp.draw(im.into_iter());

    disp.flush().unwrap();
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
