#![no_main]
#![no_std]

use core::cell::RefCell;
use core::sync::atomic::{AtomicU16, Ordering};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

#[allow(unused_imports)]
use panic_halt as _; // halt on panic

use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{interrupt, pac, prelude::*};
use stm32f4xx_hal::gpio::{Edge, Input, PC13};


type Button = PC13<Input>;

static BUTTON: Mutex<RefCell<Option<Button>>> = Mutex::new(RefCell::new(None));

static COUNTER: AtomicU16 = AtomicU16::new(0);

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
        let gpio_c = device.GPIOC.split();

        let mut button = gpio_c.pc13.into_pull_up_input();
        button.make_interrupt_source(&mut sys_cfg);
        button.enable_interrupt(&mut device.EXTI);
        button.trigger_on_edge(&mut device.EXTI, Edge::Rising);

        let mut led = gpio_a.pa5.into_push_pull_output();
        let mut delay = core.SYST.delay(&clocks);

        cortex_m::interrupt::free(|cs| {
            BUTTON.borrow(cs).replace(Some(button))
        });

        unsafe {
            cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI15_10);
        }

        loop {
            if COUNTER.load(Ordering::SeqCst) % 2 == 0 {
                led.set_low();
            }
            else {
                led.toggle();
                delay.delay_ms(100u8);
            }
        }
    }

    loop {}
}

#[interrupt]
fn EXTI15_10() {

    cortex_m::interrupt::free(|cs| {
        BUTTON
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .clear_interrupt_pending_bit();
    });

    COUNTER.fetch_add(1, Ordering::SeqCst);

    rprintln!("Button pressed.");
}
