//! Alternate some text on the display at idle frequency. The "writing to a screen" equivalent of a Hello World blinky program.
//! It uses the RTFM `app!()` macro and the [`embedded_graphics`](https://crates.io/crates/embedded-graphics) crate which adds some complexity.
//! For a more barebones example, have a look at [the `pixelsquare` example]().

#![no_std]
#![feature(const_fn)]
#![feature(proc_macro)]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_rtfm_macros;
extern crate stm32f103xx_hal as blue_pill;
extern crate embedded_hal as hal;

extern crate ssd1306;

use blue_pill::prelude::*;
use cortex_m_rtfm_macros::app;
use rtfm::{ Threshold};
use blue_pill::spi::{ Spi };
use hal::spi::{ Mode, Phase, Polarity };
use blue_pill::gpio::{ Input, Output, PushPull, Floating, Alternate };
use blue_pill::gpio::gpioa::{ PA5, PA6, PA7 };
use blue_pill::gpio::gpiob::{ PB0, PB1 };
use blue_pill::stm32f103xx::SPI1;

use ssd1306::{ SSD1306, Drawing };

pub type OledDisplay = SSD1306<
    Spi<
        SPI1,
        (
            PA5<Alternate<PushPull>>,
            PA6<Input<Floating>>,
            PA7<Alternate<PushPull>>,
        ),
    >,
    PB0<Output<PushPull>>,  // B0 -> RST
    PB1<Output<PushPull>>,  // B1 -> DC
>;

// TASKS AND RESOURCES
app! {
    device: blue_pill::stm32f103xx,

    resources: {
        static DISP: OledDisplay;
        static STATE: bool;
    },

    idle: {
        resources: [
            DISP,
            STATE,
        ],
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);

    // SPI1
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let rst = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

    let spi = Spi::spi1(
        p.device.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let disp = SSD1306::new(spi, rst, dc);

    init::LateResources {
        DISP: disp,
        STATE: false,
    }
}

fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    let state: &'static mut bool = r.STATE;
    let disp: &'static mut OledDisplay = r.DISP;

    loop {
        match *state {
            true => disp.draw_text_1bpp("On!", 0, 0),
            false => disp.draw_text_1bpp("Off :(", 0, 0),
        }

        disp.flush();

        *state = !*state;
    }
}