#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;

extern crate nrf52832_hal;
extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::image::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use nrf52832_hal::gpio::p0::Parts;
use nrf52832_hal::gpio::Level;
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

    let raw_image_data = ImageRawLE::new(include_bytes!("../assets/ferris.raw"), 86, 64);
    let ferris = Image::new(&raw_image_data, Point::new(34, 8));

    // draw image on black background
    display.clear(Rgb565::BLACK).unwrap();
    ferris.draw(&mut display).unwrap();

    hprintln!("Rendering done").unwrap();

    loop {
        continue; // keep optimizer from removing in --release
    }
}
