//! Bounce a DVD player logo around the screen
//!
//! Like this, but with no color changing: https://bouncingdvdlogo.com/
//!
//! For best results, run with the `--release` flag.

#![no_std]
#![no_main]

pub mod pac {
    pub use embassy_stm32::pac::Interrupt as interrupt;
    pub use embassy_stm32::pac::*;
}

#[rtic::app(device = crate::pac, peripherals= false, dispatchers = [EXTI0])]
mod app {
    use defmt_rtt as _;
    use display_interface_spi::SPIInterface;
    use embassy_stm32::{
        gpio,
        mode::Blocking,
        spi::{self, Spi},
        time::Hertz,
        timer::low_level::Timer,
        Config,
    };
    use embedded_graphics::{
        geometry::Point,
        image::Image,
        pixelcolor::{BinaryColor, Rgb565},
        prelude::*,
        primitives::{PrimitiveStyle, Rectangle},
    };
    use panic_probe as _;
    use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};
    use tinybmp::Bmp;

    type Display = Ssd1306<
        SPIInterface<
            embedded_hal_bus::spi::ExclusiveDevice<
                Spi<'static, Blocking>,
                gpio::Output<'static>,
                embedded_hal_bus::spi::NoDelay,
            >,
            gpio::Output<'static>,
        >,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >;

    #[shared]
    struct SharedResources {}

    #[local]
    struct Resources {
        display: Display,
        timer: Timer<'static, embassy_stm32::peripherals::TIM1>,
        top_left: Point,
        velocity: Point,
        bmp: Bmp<Rgb565, 'static>,
    }

    #[init]
    fn init(_cx: init::Context) -> (SharedResources, Resources, init::Monotonics) {
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
        let mut config = spi::Config::default();
        config.frequency = Hertz::mhz(8);
        let spi = Spi::new_blocking_txonly(p.SPI1, p.PA5, p.PA7, config);

        let mut rst = gpio::Output::new(p.PB0, gpio::Level::Low, gpio::Speed::Low);
        let dc = gpio::Output::new(p.PB1, gpio::Level::Low, gpio::Speed::Low);
        let cs = gpio::Output::new(p.PB10, gpio::Level::Low, gpio::Speed::Low);
        let spi = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();

        let interface = display_interface_spi::SPIInterface::new(spi, dc);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180)
            .into_buffered_graphics_mode();

        display
            .reset(&mut rst, &mut embassy_time::Delay {})
            .unwrap();
        display.init().unwrap();

        // Forget the RST pin to keep the display out of reset
        core::mem::forget(rst);

        // Update framerate
        let timer = Timer::new(p.TIM1);
        timer.set_frequency(Hertz(20)); // 20 FPS
        timer.enable_update_interrupt(true);
        timer.start();

        let bmp = Bmp::from_slice(include_bytes!("dvd.bmp")).unwrap();

        // Init the static resources to use them later through RTIC
        (
            SharedResources {},
            Resources {
                timer,
                display,
                top_left: Point::new(5, 3),
                velocity: Point::new(1, 1),
                bmp,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = TIM1_UP, local = [display, top_left, velocity, timer, bmp ])]
    fn update(cx: update::Context) {
        let update::LocalResources {
            display,
            top_left,
            velocity,
            timer,
            bmp,
            ..
        } = cx.local;

        let bottom_right = *top_left + bmp.bounding_box().size;

        // Erase previous image position with a filled black rectangle
        Rectangle::with_corners(*top_left, bottom_right)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(display)
            .unwrap();

        // Check if the image collided with a screen edge
        {
            if bottom_right.x > display.size().width as i32 || top_left.x < 0 {
                velocity.x = -velocity.x;
            }

            if bottom_right.y > display.size().height as i32 || top_left.y < 0 {
                velocity.y = -velocity.y;
            }
        }

        // Move the image
        *top_left += *velocity;

        // Draw image at new position
        Image::new(bmp, *top_left)
            .draw(&mut display.color_converted())
            .unwrap();

        // Write changes to the display
        display.flush().unwrap();

        // Clears the update flag
        timer.clear_update_interrupt();
    }
}
