#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate nrf52832_hal;
extern crate panic_halt;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // See examples folder

    loop {
        continue;
    }
}
