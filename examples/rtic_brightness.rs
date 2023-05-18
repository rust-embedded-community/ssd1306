//! Cycle the screen brightness through 5 predefined brightness levels when the "DVD" logo hits one
//! of the sides of the display.
//!
//! For best results, run with the `--release` flag.

#![no_std]
#![no_main]

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use display_interface_spi::SPIInterfaceNoCS;
    use embedded_graphics::{
        geometry::Point,
        image::Image,
        pixelcolor::{BinaryColor, Rgb565},
        prelude::*,
        primitives::{PrimitiveStyle, Rectangle},
    };
    use panic_halt as _;
    use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};
    use stm32f1xx_hal::{
        gpio,
        pac::{self, SPI1},
        prelude::*,
        spi::{self, Mode, Phase, Polarity, Spi},
        timer::{CounterMs, Event, Timer},
    };
    use tinybmp::Bmp;

    type Display = Ssd1306<
        SPIInterfaceNoCS<
            spi::Spi<
                SPI1,
                spi::Spi1NoRemap,
                (
                    gpio::gpioa::PA5<gpio::Alternate<gpio::PushPull>>,
                    gpio::gpioa::PA6<gpio::Input<gpio::Floating>>,
                    gpio::gpioa::PA7<gpio::Alternate<gpio::PushPull>>,
                ),
                u8,
            >,
            gpio::gpiob::PB1<gpio::Output<gpio::PushPull>>,
        >,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >;

    #[shared]
    struct SharedResources {}

    #[local]
    struct Resources {
        display: Display,
        timer: CounterMs<pac::TIM1>,
        top_left: Point,
        velocity: Point,
        bmp: Bmp<Rgb565, 'static>,
        brightness: Brightness,
    }

    #[init]
    fn init(cx: init::Context) -> (SharedResources, Resources, init::Monotonics) {
        let dp = cx.device;
        let core = cx.core;

        let mut flash = dp.FLASH.constrain();
        let rcc = dp.RCC.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .freeze(&mut flash.acr);

        let mut afio = dp.AFIO.constrain();

        let mut gpiob = dp.GPIOB.split();
        let mut gpioa = dp.GPIOA.split();

        // SPI1
        let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
        let miso = gpioa.pa6;
        let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

        let mut delay = Timer::syst(core.SYST, &clocks).delay();

        let mut rst = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
        let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

        let spi = Spi::spi1(
            dp.SPI1,
            (sck, miso, mosi),
            &mut afio.mapr,
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            8.MHz(),
            clocks,
        );

        let interface = display_interface_spi::SPIInterfaceNoCS::new(spi, dc);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180)
            .into_buffered_graphics_mode();

        display.reset(&mut rst, &mut delay).unwrap();
        display.init().unwrap();

        // Update framerate
        let mut timer = dp.TIM1.counter_ms(&clocks);
        timer.start(50.millis()).unwrap(); // 20 FPS

        timer.listen(Event::Update);

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
                brightness: Brightness::default(),
            },
            init::Monotonics(),
        )
    }

    #[task(binds = TIM1_UP, local = [display, top_left, velocity, timer, bmp, brightness])]
    fn update(cx: update::Context) {
        let update::LocalResources {
            display,
            top_left,
            velocity,
            timer,
            bmp,
            brightness,
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
            let mut collision = false;
            if bottom_right.x > display.size().width as i32 || top_left.x < 0 {
                velocity.x = -velocity.x;
                collision = true;
            }

            if bottom_right.y > display.size().height as i32 || top_left.y < 0 {
                velocity.y = -velocity.y;
                collision = true;
            }

            if collision {
                // Change the brightness
                *brightness = match *brightness {
                    Brightness::DIMMEST => Brightness::DIM,
                    Brightness::DIM => Brightness::NORMAL,
                    Brightness::NORMAL => Brightness::BRIGHT,
                    Brightness::BRIGHT => Brightness::BRIGHTEST,
                    Brightness::BRIGHTEST => Brightness::DIMMEST, // restart cycle
                    _ => Brightness::NORMAL, // Brightness is not an enum, cover rest of patterns
                };

                // Send the new brightness value to the display
                display.set_brightness(*brightness).unwrap();
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
        timer.clear_interrupt(Event::Update);
    }
}
