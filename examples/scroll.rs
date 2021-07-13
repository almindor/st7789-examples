#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;

extern crate nrf52832_hal;
extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use nrf52832_hal::gpio::p0::Parts;
use nrf52832_hal::gpio::Level;
use nrf52832_hal::prelude::*;
use nrf52832_hal::spim;
use nrf52832_hal::Delay;
use st7789::{Orientation, ST7789};

#[entry]
fn main() -> ! {
    let core = nrf52832_hal::pac::CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST);

    let p = nrf52832_hal::pac::Peripherals::take().unwrap();
    let port0 = Parts::new(p.P0);

    let _backlight = port0.p0_22.into_push_pull_output(Level::Low); // set medium backlight on
    let rst = port0.p0_26.into_push_pull_output(Level::Low); // reset pin
    let _cs = port0.p0_25.into_push_pull_output(Level::Low); // keep low while drivign display
    let dc = port0.p0_18.into_push_pull_output(Level::Low); // data/clock switch

    let spiclk = port0.p0_02.into_push_pull_output(Level::Low).degrade(); // SPI clock to LCD
    let spimosi = port0.p0_03.into_push_pull_output(Level::Low).degrade(); // SPI MOSI to LCD

    let pins = spim::Pins {
        sck: spiclk,
        miso: None,
        mosi: Some(spimosi),
    };

    // create SPI interface
    let spi = spim::Spim::new(p.SPIM0, pins, spim::Frequency::M8, spim::MODE_3, 122);

    // display interface abstraction from SPI and DC
    let di = SPIInterfaceNoCS::new(spi, dc);

    // create driver
    let mut display = ST7789::new(di, rst, 240, 240);

    // initialize
    display.init(&mut delay).unwrap();
    // set default orientation
    display.set_orientation(Orientation::Landscape).unwrap();

    // 3 lines composing a big "F"
    let line1 = Line::new(Point::new(100, 20), Point::new(100, 220))
        .into_styled(PrimitiveStyle::with_stroke(RgbColor::WHITE, 10));
    let line2 = Line::new(Point::new(100, 20), Point::new(160, 20))
        .into_styled(PrimitiveStyle::with_stroke(RgbColor::WHITE, 10));
    let line3 = Line::new(Point::new(100, 105), Point::new(160, 105))
        .into_styled(PrimitiveStyle::with_stroke(RgbColor::WHITE, 10));

    // triangle to be shown "in the scroll zone"
    let triangle = Triangle::new(
        Point::new(240, 100),
        Point::new(240, 140),
        Point::new(320, 120),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN));

    // draw the "F" + scroll-section arrow triangle
    display.clear(Rgb565::BLACK).unwrap();
    line1.draw(&mut display).unwrap();
    line2.draw(&mut display).unwrap();
    line3.draw(&mut display).unwrap();
    triangle.draw(&mut display).unwrap();

    hprintln!("Rendering done, scrolling...").unwrap();

    let mut scroll = 1u16; // absolute scroll offset
    let mut direction = true; // direction
    let scroll_delay = 20u8; // delay between steps
    loop {
        delay.delay_ms(scroll_delay);
        display.set_scroll_offset(scroll).unwrap();

        if scroll % 80 == 0 {
            direction = !direction;
        }

        match direction {
            true => scroll += 1,
            false => scroll -= 1,
        }
    }
}
