#![no_main]
#![no_std]


use stm32f4xx_hal::pac;
use cortex_m_rt::entry;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::timer::{CounterMs, Event};
use stm32f4xx_hal::prelude::*;


#[entry]
fn main() -> ! {

    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    rtt_init_print!();

    let mut sys_cfg = device.SYSCFG.constrain();
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    let gpio_a = device.GPIOA.split();

    let mut tracker = gpio_a.pa0.into_pull_up_input();
    let mut delay = core.SYST.delay(&clocks);


    loop {
        if tracker.is_high() {
            rprintln!("tracker is on line")
        }
        else {
            rprintln!("tracker is not on line")
        }

        delay.delay_ms(500u32)
    }
}
