#![no_main]
#![no_std]


use stm32f4xx_hal::pac;
use cortex_m_rt::entry;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::prelude::*;

enum RotaryEncoderStates {
    Idle,
    ClockwiseCLKFirst,
    ContinueClockwise,
    CompleteStepClockwise,
    AnticlockwiseDTFirst,
    ContinueAnticlockwise,
    CompleteStepAnticlockwise,
}

#[entry]
fn main() -> ! {

    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    rtt_init_print!();

    let mut sys_cfg = device.SYSCFG.constrain();
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    let gpio_a = device.GPIOA.split();
    let gpio_b = device.GPIOB.split();

    let mut delay = core.SYST.delay(&clocks);

    let mut pin_clk = gpio_a.pa8
        .into_pull_up_input();
    let mut pin_dt = gpio_b.pb10
        .into_pull_up_input();
    let mut button = gpio_b.pb4
        .into_pull_up_input();

    // let mut last = pin_clk.is_low();

    let mut counter = 0;

    let mut state: RotaryEncoderStates = RotaryEncoderStates::Idle;

    loop {
        // let current = pin_clk.is_low();

        if button.is_low() {
            counter = 0;
        }

        match state {
            RotaryEncoderStates::Idle => {
                if pin_clk.is_low() {
                    state = RotaryEncoderStates::ClockwiseCLKFirst;
                }
                else if pin_dt.is_low() {
                    state = RotaryEncoderStates::AnticlockwiseDTFirst;
                }
            }
            RotaryEncoderStates::ClockwiseCLKFirst => {
                if pin_dt.is_low() {
                    state = RotaryEncoderStates::ContinueClockwise;
                }
            }
            RotaryEncoderStates::ContinueClockwise => {
                if pin_clk.is_high() {
                    state = RotaryEncoderStates::CompleteStepClockwise;
                }
            }
            RotaryEncoderStates::CompleteStepClockwise => {
                if pin_clk.is_high() && pin_dt.is_high() {
                    state = RotaryEncoderStates::Idle;
                    counter += 1;
                }
            }
            RotaryEncoderStates::AnticlockwiseDTFirst => {
                if pin_clk.is_low() {
                    state = RotaryEncoderStates::ContinueAnticlockwise;
                }
            }
            RotaryEncoderStates::ContinueAnticlockwise => {
                if pin_dt.is_high() {
                    state = RotaryEncoderStates::CompleteStepAnticlockwise;
                }
            }
            RotaryEncoderStates::CompleteStepAnticlockwise => {
                if pin_dt.is_high() && pin_clk.is_high() {
                    state = RotaryEncoderStates::Idle;
                    counter -= 1;
                }
            }
        }

        rprintln!("{:?}", counter);

        //delay.delay_ms(10u32)
    }
}
