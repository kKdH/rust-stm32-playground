#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f4xx_hal::spi::{Spi, Mode, Phase, Polarity};
use stm32f4xx_hal::gpio::{self, gpiob::PB14, Alternate, AF5};
use stm32f4xx_hal::prelude::*;

#[allow(unused_imports)]
use panic_halt as _;


static mut RX_BUFFER: u8 = 0; // Empfangspuffer
static mut TX_BUFFER: u8 = 0; // Sendepuffer

#[entry]
fn main() -> ! {
    fn main() -> ! {
        // STM32 Initialisierung
        let dp = stm32f4xx_hal::stm32::Peripherals::take().unwrap();
        let cp = cortex_m::peripheral::Peripherals::take().unwrap();

        // GPIO-Bank B aktivieren
        let gpiob = dp.GPIOB.split();

        // SPI1 konfigurieren
        let sck = gpiob.pb3.into_alternate_af5();
        let miso = gpiob.pb4.into_alternate_af5();
        let mosi = gpiob.pb5.into_alternate_af5();
        let mut spi = Spi::spi1(
            dp.SPI1,
            (sck, miso, mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            1_000_000.hz(),
            cp.SYST,
            &mut cp.NVIC,
        );
        spi.enable_interrupt(Event::Rxne); // Interrupt bei Empfangsbereitschaft aktivieren

        loop {
                // Daten vom Master empfangen
                let received_data = unsafe { cortex_m::interrupt::free(|cs| {
                    spi.read_to(&mut RX_BUFFER).unwrap();
                    RX_BUFFER
                }) };

                // Verarbeiten Sie die empfangenen Daten und aktualisieren Sie die zu sendenden Daten
                let processed_data = received_data + 1;
                unsafe {
                    TX_BUFFER = processed_data;
                }

                // Daten an den Master senden
                unsafe { cortex_m::interrupt::free(|cs| {
                    spi.send(&[TX_BUFFER]).unwrap();
                }) };
        }
}
