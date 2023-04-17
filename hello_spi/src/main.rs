#![no_main]
#![no_std]

use cortex_m_rt::entry;

use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{pac, prelude::*};
use stm32f4xx_hal::gpio::Speed;
use stm32f4xx_hal::spi::{Mode, Phase, Polarity};

#[allow(unused_imports)]
use panic_halt as _;


#[entry]
fn main() -> ! {
    if let (Some(device_peripherals), Some(core)) = (
        pac::Peripherals::take(),
        cortex_m::Peripherals::take()
    ) {
        rtt_init_print!();

        rprintln!("Started");

        let sys_cfg = device_peripherals.SYSCFG.constrain();
        let rcc = device_peripherals.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let mut delay = device_peripherals.TIM1.delay_us(&clocks);

        let gpio_a = device_peripherals.GPIOA.split();
        let gpio_c = device_peripherals.GPIOC.split();
        let gpio_b = device_peripherals.GPIOB.split();

        let mut spi_clock = gpio_a.pa5
            .into_alternate::<5>()
            .speed(Speed::VeryHigh)
            .internal_pull_up(true);

        let spi_miso = gpio_a.pa6
            .into_alternate::<5>()
            .speed(Speed::VeryHigh);

        let spi_mosi = gpio_a.pa7
            .into_alternate::<5>()
            .speed(Speed::VeryHigh);

        let mut spi_cs = gpio_b.pb6
            .into_push_pull_output();

        rprintln!("Pins configured.");

        let mut spi1 = device_peripherals.SPI1.spi(
            (spi_clock, spi_miso, spi_mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            3.MHz(),
            &clocks,
        ).to_bidi_transfer_mode();

        rprintln!("SPI1 configured.");

        let mut buffer = [0u8; 1];
        buffer[0] = 0x88;

        while !spi1.is_tx_empty() {}

        match spi1.transfer(&mut buffer) {
            Ok(received) => { rprintln!("Write Ok: {:?}", received) }
            Err(cause) => { rprintln!("Write Err: {:?}", cause) }
        };

        // while spi1.is_busy() {
        //     rprintln!("BUSY")
        // }

        // spi_cs.set_low();

        // buffer[0] = 0x007F;
        // match spi.transfer(&mut buffer) {
        //     Ok(received) => rprintln!("Configuration Ok: {:#06x}", received[0]),
        //     Err(cause) => rprintln!("Configuration Err: {:?}", cause),
        // }
        //
        // delay.delay_ms(200u8);
        //
        // buffer[0] = 0xFFFF;
        // match spi.transfer(&mut buffer) {
        //     Ok(received) => rprintln!("Brightness Ok: {:#06x}", received[0]),
        //     Err(cause) => rprintln!("Brightness Err: {:?}", cause),
        // }
        //
        // delay.delay_ms(200u8);
        //
        // buffer[0] = 0xFFFF;
        // match spi.transfer(&mut buffer) {
        //     Ok(received) => rprintln!("Global Latch Ok: {:#06x}", received[0]),
        //     Err(cause) => rprintln!("Global Latch Err: {:?}", cause),
        // }
        //
        // delay.delay_ms(200u8);
        //
        // buffer[0] = 0x0001;
        // match spi.transfer(&mut buffer) {
        //     Ok(received) => rprintln!("Switch Control Ok: {:#06x}", received[0]),
        //     Err(cause) => rprintln!("Switch Control Err: {:?}", cause),
        // }
        //
        // delay.delay_ms(200u8);
        //
        // spi_cs.set_high();

        loop {
            rprintln!("Hello");
            delay.delay_ms(1000u32);
        }
    }

    loop {}
}
