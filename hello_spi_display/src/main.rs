#![no_main]
#![no_std]


use stm32f4xx_hal::pac;
use cortex_m_rt::entry;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::prelude::*;


#[entry]
fn main() -> ! {
    loop {}
}
