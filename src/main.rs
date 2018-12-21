#![no_main]
#![no_std]

// set the panic handler
extern crate panic_semihosting;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use hal::gpio;
use hal::prelude::*;

// declare our global variable to hold the led device
type Led = gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>;
static LED: Mutex<RefCell<Option<Led>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut core = cortex_m::Peripherals::take().unwrap();
    let device = hal::stm32f103xx::Peripherals::take().unwrap();
    let mut rcc = device.RCC.constrain();
    let mut flash = device.FLASH.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(16.mhz())
        .freeze(&mut flash.acr);

    // configure the user led
    let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // move the led in the global variable
    cortex_m::interrupt::free(move |cs| {
        *LED.borrow(cs).borrow_mut() = Some(led);
    });

    // configure SysTick to generate an exception every second
    core.SYST.set_clock_source(SystClkSource::Core);
    core.SYST.set_reload(clocks.sysclk().0);
    core.SYST.enable_counter();
    core.SYST.enable_interrupt();

    // sleep
    loop {
        cortex_m::asm::wfi();
    }
}

#[exception]
fn SysTick() {
    static mut state: bool = false;

    cortex_m::interrupt::free(|cs| {
        if let Some(led) = LED.borrow(cs).borrow_mut().as_mut() {
            if *state {
                led.set_low();
            } else {
                led.set_high();
            }
            *state = !*state;
        }
    });
}
