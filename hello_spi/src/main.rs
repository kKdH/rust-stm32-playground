#![no_main]
#![no_std]

use cortex_m_rt::entry;

use rtt_target::rtt_init_print;

use stm32f4xx_hal::{pac, prelude::*};
use stm32f4xx_hal::spi::{Mode, Phase, Polarity};

#[allow(unused_imports)]
use panic_halt as _;


#[entry]
fn main() -> ! {
    if let (Some(device), Some(core)) = (
        pac::Peripherals::take(),
        cortex_m::Peripherals::take()
    ) {
        rtt_init_print!();

        let mut device = device;
        let mut sys_cfg = device.SYSCFG.constrain();
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

        let gpio_a = device.GPIOA.split();
        let gpio_b = device.GPIOB.split();

        let spi_sck = gpio_a.pa5.into_pull_up_input();
        let spi_miso = gpio_a.pa6.into_pull_up_input();
        let spi_mosi = gpio_a.pa7.into_push_pull_output();
        let spi_cs = gpio_b.pb6.into_push_pull_output();

        let spi = device.SPI1.spi(
            (spi_sck, spi_miso, spi_mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            100.kHz(),
            &clocks
        );

        let mut delay = core.SYST.delay(&clocks);

        loop {
            delay.delay_ms(100u8);
        }
    }

    loop {}
}
