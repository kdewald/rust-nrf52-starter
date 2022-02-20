#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

use nrf52840_hal as hal;
use nrf52840_hal::gpio::Level;
use nrf52840_hal::prelude::InputPin;
use nrf52840_hal::prelude::OutputPin;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let button = port0.p0_11.into_pullup_input();
    let mut led = port0.p0_13.into_push_pull_output(Level::Low);

    rprintln!("Hello, world!");
    loop {
        if button.is_high().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
    }
}
