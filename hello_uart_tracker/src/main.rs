#![no_main]
#![no_std]

use cortex_m::register::control::Npriv::Privileged;
use cortex_m_rt::entry;
use panic_halt as _;
use postcard::ser_flavors::{Flavor, HVec};
use rtt_target::{rprintln, rtt_init_print};
use serde::forward_to_deserialize_any;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::block;
use stm32f4xx_hal::gpio::{Input, Output, Pin};
use stm32f4xx_hal::pac::USART2;
use stm32f4xx_hal::serial::{Config, Rx};
use stm32f4xx_hal::serial::config::{DmaConfig, Parity, StopBits, WordLength};
use stm32f4xx_hal::time::Bps;
use stm32f4xx_hal::timer::{CounterUs, DelayUs, SysDelay};

use crate::hal::{pac, prelude::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq)]
struct Data {
    id: u32,
    //message: &'a str,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let device_peripherals = pac::Peripherals::take().unwrap();

    let gpio_a = device_peripherals.GPIOA.split();
    let gpio_b = device_peripherals.GPIOB.split();
    let gpio_c = device_peripherals.GPIOC.split();

    let rcc = device_peripherals.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze();

    let mut trigger = gpio_b.pb6.into_push_pull_output();
    let mut echo: Pin<'C', 7> = gpio_c.pc7.into_pull_down_input();

    let mut delay: DelayUs<stm32f4xx_hal::pac::TIM1> = device_peripherals.TIM1.delay_us(&clocks);
    let mut counter: CounterUs<stm32f4xx_hal::pac::TIM2> = device_peripherals.TIM2.counter_us(&clocks);

    let config = Config {
        baudrate: Bps(9600),
        wordlength: WordLength::DataBits8,
        parity: Parity::ParityNone,
        stopbits: StopBits::STOP1,
        dma: DmaConfig::None,
    };

    let mut tracker = gpio_a.pa0.into_pull_up_input();

    let tx_pin = gpio_a.pa9.into_alternate();
    let rx_pin = gpio_a.pa10.into_alternate();

    let serial = device_peripherals.USART1.serial(
        (tx_pin, rx_pin),
        config,
        &clocks,
    ).unwrap();

    let (mut tx, mut rx) = serial.split();

    rprintln!("Started");

    let mut value: u32 = 0;
    let mut buffer = [0u8; core::mem::size_of::<Data>()];
    rprintln!("Created buffer with size {}", buffer.len());
    loop {

        let message_length = block!(rx.read()).unwrap() as usize;
        for i in 0..message_length {
            buffer[i] = block!(rx.read()).unwrap();
        }

        match postcard::from_bytes::<Data>(&buffer) {
            Ok(data) => {
                rprintln!("Received command: {:?}", data);
            }
            Err(_) => {}
        }

        // delay.delay(200.millis());

        value = measure(&mut trigger, &mut echo, &mut delay, &mut counter);

        let data = Data {
            id: value
        };

        match postcard::to_slice(&data, &mut buffer) {
            Ok(data_bytes) => {

                let mut message_vec: HVec<8> = HVec::new();
                message_vec.try_push(data_bytes.len() as u8);
                message_vec.try_extend(data_bytes);

                let message = message_vec.finalize().unwrap();

                match tx.bwrite_all(message.as_slice()) {
                    Ok(_) => {
                        rprintln!("successfully wrote {:?}", data);
                    }
                    Err(_) => {
                        rprintln!("failed to write data into buffer");
                    }
                }
            }
            Err(_) => {
                rprintln!("failed to serialize data into byte array.");
            }
        };

            //expect("Failed to encode data as bytes");

        //block!(tx.write(value)).unwrap();
        //rprintln!("Sent {}", value);

        //value = block!(rx.read()).unwrap();
        //rprintln!("Received {}", value);

        delay.delay(50.millis());
    }
}

fn measure(
    trigger: &mut Pin<'B', 6, Output>,
    echo: &mut Pin<'C', 7, Input>,
    delay: &mut DelayUs<stm32f4xx_hal::pac::TIM1>,
    counter: &mut CounterUs<stm32f4xx_hal::pac::TIM2>
) -> u32 {
    trigger.set_high();
    delay.delay_us(20u32);
    trigger.set_low();

    while echo.is_low() {}
    counter.start(1000.millis()).unwrap();
    while echo.is_high() {}

    let duration = counter.now().duration_since_epoch();
    counter.cancel().unwrap();

    //Verhält sich wie ein return
    duration.to_micros() / 2 / 29
}