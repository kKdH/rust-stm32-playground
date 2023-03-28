#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprint, rprintln, rtt_init_print};
use stm32f4xx_hal::{pac, prelude::*};
use stm32f4xx_hal::gpio::Speed;
use stm32f4xx_hal::spi::{BitFormat, Mode, Phase, Polarity};

#[allow(unused_imports)]
use panic_halt as _;


#[entry]
fn main() -> ! {
    if let (Some(device), Some(core)) = (
        pac::Peripherals::take(),
        cortex_m::Peripherals::take()
    ) {
        rtt_init_print!();

        rprintln!("Started");

        let mut device_peripherals = device;
        let mut sys_cfg = device_peripherals.SYSCFG.constrain();
        let rcc = device_peripherals.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
        let mut delay = core.SYST.delay(&clocks);

        let gpio_a = device_peripherals.GPIOA.split();
        let gpio_c = device_peripherals.GPIOC.split();
        let gpio_b = device_peripherals.GPIOB.split();

        let led_indication = gpio_c.pc7
            .into_push_pull_output()
            .speed(Speed::Low);

        let le_signal = gpio_a.pa10
            .into_push_pull_output()
            .speed(Speed::Low);

        let spi_sck = gpio_a.pa5
            .into_alternate::<5>()
            .speed(Speed::High);

        let spi_miso = gpio_a.pa6
            .into_alternate::<5>()
            .speed(Speed::High);

        let spi_mosi = gpio_a.pa7
            .into_alternate::<5>()
            .speed(Speed::High);

        let mut spi_nss = gpio_a.pa4
            .into_push_pull_output();

        rprintln!("Pins configured.");

        let mut spi = device_peripherals.SPI1
            .spi(
                (spi_sck, spi_miso, spi_mosi),
                Mode {
                    polarity: Polarity::IdleLow,
                    phase: Phase::CaptureOnFirstTransition,
                },
                42.MHz(),
                &clocks
            )
            .frame_size_16bit()
            .init();

        let mut buffer = [0u16; 1];

        spi_nss.set_low();

        buffer[0] = 0x007F;
        match spi.transfer(&mut buffer) {
            Ok(received) => rprintln!("Configuration Ok: {:#06x}", received[0]),
            Err(cause) => rprintln!("Configuration Err: {:?}", cause),
        }

        delay.delay_ms(200u8);

        buffer[0] = 0xFFFF;
        match spi.transfer(&mut buffer) {
            Ok(received) => rprintln!("Brightness Ok: {:#06x}", received[0]),
            Err(cause) => rprintln!("Brightness Err: {:?}", cause),
        }

        delay.delay_ms(200u8);

        buffer[0] = 0xFFFF;
        match spi.transfer(&mut buffer) {
            Ok(received) => rprintln!("Global Latch Ok: {:#06x}", received[0]),
            Err(cause) => rprintln!("Global Latch Err: {:?}", cause),
        }

        delay.delay_ms(200u8);

        buffer[0] = 0x0001;
        match spi.transfer(&mut buffer) {
            Ok(received) => rprintln!("Switch Control Ok: {:#06x}", received[0]),
            Err(cause) => rprintln!("Switch Control Err: {:?}", cause),
        }

        delay.delay_ms(200u8);

        spi_nss.set_high();

        loop {
            delay.delay_ms(200u8);
        }
    }

    loop {}
}
