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

    let gpio_d = device.GPIOD.split();

    let mut led = gpio_a.pa5.into_push_pull_output();
    let mut delay = core.SYST.delay(&clocks);

    let mut trigger = gpio_a.pa0.into_push_pull_output();
    let mut echo = gpio_a.pa1.into_pull_down_input();


    rprintln!("Starting to blink");

    let sampling_rate: u32 = 5;
    let mut delay = device.TIM1.delay_us(&clocks);
    let mut counter = device.TIM2.counter_us(&clocks);

    loop {

        trigger.set_high();
        delay.delay_us(20u32);
        trigger.set_low();

        while echo.is_low() {}
        counter.start(1000.millis()).unwrap();
        while echo.is_high() {}

        let duration = counter.now().duration_since_epoch();
        counter.cancel().unwrap();
        let distance_cm = duration.to_micros() / 2 / 29;
        rprintln!("distance: {}", distance_cm);

        delay.delay_ms(1000u32)
    }
}
