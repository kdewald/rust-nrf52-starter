#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_probe as _;
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!(); // You may prefer to initialize another way    
    rprintln!("Hello, world!");
    loop { asm::bkpt() }
}
