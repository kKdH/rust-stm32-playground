#![no_main]
#![no_std]

use bxcan::filter::Mask32;
use bxcan::{Frame, StandardId};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

use stm32f3xx_hal as hal;
use hal::pac;
use hal::prelude::*;
use hal::block;

#[allow(unused_imports)]
use panic_halt as _;

#[entry]
fn main() -> ! {

    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let device_peripherals = pac::Peripherals::take().unwrap();

    rtt_init_print!();

    rprintln!("Initializing");

    let mut flash = device_peripherals.FLASH.constrain();
    let mut rcc = device_peripherals.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(32.MHz())
        .hclk(64.MHz())
        .sysclk(64.MHz())
        .pclk1(32.MHz())
        .pclk2(64.MHz())
        .freeze(&mut flash.acr);

    rprintln!("Clocks initialized");

    let mut delay = hal::delay::Delay::new(core_peripherals.SYST, clocks);

    let mut gpio_a = device_peripherals.GPIOA.split(&mut rcc.ahb);
    let _gpio_b = device_peripherals.GPIOB.split(&mut rcc.ahb);

    let mut can = {

        let mut can_1 = {
            let rx = gpio_a.pa11
                .into_af_push_pull::<9>(&mut gpio_a.moder, &mut gpio_a.otyper, &mut gpio_a.afrh);
            let tx = gpio_a.pa12
                .into_af_push_pull::<9>(&mut gpio_a.moder, &mut gpio_a.otyper, &mut gpio_a.afrh);
            bxcan::Can::builder(hal::can::Can::new(device_peripherals.CAN, tx, rx, &mut rcc.apb1))
                .set_bit_timing(0x001c_0003)
                .set_loopback(false)
                .set_silent(false)
                .leave_disabled()
        };

        {
            let mut filters = can_1.modify_filters();
            // filters.set_split(14); // 28 filters are shared between the two CAN instances.
            filters.enable_bank(0, Mask32::accept_all());

            // let mut slave_filters = filters.slave_filters();
            // slave_filters.enable_bank(14, Fifo::Fifo0, Mask32::accept_all());
        }

        // let can_2 = {
        //     let rx = gpio_b.pb5;
        //     let tx = gpio_b.pb6;
        //     let can = device_peripherals.CAN2.can((tx, rx));
        //     bxcan::Can::builder(can)
        //         .set_bit_timing(0x0019_0003)
        //         .set_loopback(true)
        //         .enable()
        // };

        block!(can_1.enable_non_blocking()).ok();
        can_1
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
