#![no_main]
#![no_std]


use stm32f4xx_hal::pac;
use cortex_m_rt::entry;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

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

    let mut led = gpio_a.pa5.into_push_pull_output();
    let mut delay = core.SYST.delay(&clocks);

    rprintln!("Starting to blink");

    loop {
        led.toggle();
        delay.delay_ms(75u8);
    }
}
