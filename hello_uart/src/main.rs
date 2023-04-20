#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::serial::Config;
use stm32f4xx_hal::serial::config::{DmaConfig, Parity, StopBits, WordLength};
use stm32f4xx_hal::time::Bps;

#[entry]
fn main() -> ! {

    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let gpio_a = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze();
    let mut delay = dp.TIM1.delay_ms(&clocks);

    let config = Config {
        baudrate: Bps(9600),
        wordlength: WordLength::DataBits8,
        parity: Parity::ParityNone,
        stopbits: StopBits::STOP1,
        dma: DmaConfig::None,
    };
    let tx_pin = gpio_a.pa9.into_alternate();
    let mut tx = dp.USART1.tx(tx_pin, config, &clocks).unwrap();

    rprintln!("Started");

    let mut value: u8 = 0;
    loop {
        tx.write(value).unwrap();
        rprintln!("Wrote value: {}", value);
        value = value.wrapping_add(1);
        delay.delay(20.millis());
    }
}
