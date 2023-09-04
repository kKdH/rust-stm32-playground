#![no_main]
#![no_std]

use bxcan::filter::Mask32;
use bxcan::{Fifo, Frame, StandardId};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{block, pac, prelude::*};

#[allow(unused_imports)]
use panic_halt as _;

#[entry]
fn main() -> ! {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let device_peripherals = pac::Peripherals::take().unwrap();

    rtt_init_print!();

    rprintln!("Initializing");

    let _sys_cfg = device_peripherals.SYSCFG.constrain();
    let rcc = device_peripherals.RCC.constrain();
    let clocks = rcc.cfgr
        .use_hse(26.MHz())
        .freeze();

    rprintln!("Clocks initialized");

    let mut delay = device_peripherals.TIM1.delay_us(&clocks);

    let gpio_b = device_peripherals.GPIOB.split();

    let mut can = {

        let mut can_1 = {
            let rx = gpio_b.pb8;
            let tx = gpio_b.pb9;
            let can = device_peripherals.CAN1.can((tx, rx));
            bxcan::Can::builder(can)
                .set_bit_timing(0x0019_0003)
                .leave_disabled()
        };

        {
            let mut filters = can_1.modify_filters();
            filters.set_split(14); // 28 filters are shared between the two CAN instances.
            filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());

            let mut slave_filters = filters.slave_filters();
            slave_filters.enable_bank(14, Fifo::Fifo0, Mask32::accept_all());
        }

        let can_2 = {
            let rx = gpio_b.pb5;
            let tx = gpio_b.pb6;
            let can = device_peripherals.CAN2.can((tx, rx));
            bxcan::Can::builder(can)
                .set_bit_timing(0x0019_0003)
                .set_loopback(true)
                .enable()
        };

        can_2
    };

    rprintln!("CAN initialized");

    rprintln!("Initialization complete.");

    let mut test: [u8; 8] = [0; 8];
    let mut count: u8 = 0;
    let id: u16 = 0x500;

    test[1] = 1;
    test[2] = 2;
    test[3] = 3;
    test[4] = 4;
    test[5] = 5;
    test[6] = 6;
    test[7] = 7;

    loop {
        test[0] = count;

        let test_frame = Frame::new_data(StandardId::new(id).unwrap(), test);

        block!(can.transmit(&test_frame)).unwrap();
        rprintln!("Sent: {:?}", test_frame);

        let received_frame = block!(can.receive());
        rprintln!("Received: {:?}", received_frame);

        delay.delay_ms(1000u32);

        count = count.wrapping_add(1);
    }
}
