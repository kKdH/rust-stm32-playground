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

    let gpio_d = device.GPIOD.split();

    let mut led = gpio_a.pa5.into_push_pull_output();
    let mut delay = core.SYST.delay(&clocks);

    let mut trigger = gpio_a.pa0.into_push_pull_output();
    let mut echo = gpio_a.pa1.into_pull_down_input();


    rprintln!("Starting to blink");

    let sampling_rate: u32 = 5;
    let mut measuring: bool = false;
    let mut counter: u32 = 0;

    loop {

        if measuring == false {
            trigger.set_high();
            delay.delay_us(20u32);
            trigger.set_low();
            measuring = true;
        }

        if measuring && echo.is_high() {
            counter += 1;
        }

        if echo.is_low() && counter > 0 {
            let distance_in_time: f64 = (counter * sampling_rate) as f64 / 1000000.0;
            let distance = (343.0 * distance_in_time) / 2.0;
            rprintln!("distance: {}", distance);
            measuring = false;
            counter = 0;
            delay.delay_ms(1000u32)
        }

        delay.delay_us(sampling_rate)
    }
}
