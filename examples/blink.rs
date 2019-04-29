#![no_main]
#![no_std]

extern crate atsaml11xxx_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;
// makes `panic!` print messages to the host stderr using semihosting
extern crate panic_semihosting;

extern crate cortex_m_semihosting;

use cortex_m_rt::entry;

use cortex_m::asm::nop;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::hal::digital::OutputPin;
use hal::prelude::*;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    let mut p = atsaml11xxx::Peripherals::take().unwrap();
    let core = atsaml11xxx::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        p.GCLK,
        &mut p.MCLK,
        &mut p.OSCCTRL,
        &mut p.OSC32KCTRL,
        &mut p.NVMCTRL,
    );

    let mut pins = p.PORT_SEC.split();

    let mut led = pins.pa7.into_push_pull_output(&mut pins.port);

    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        led.set_high();

        delay.delay_ms(1000u16);

        led.set_low();

        delay.delay_ms(1000u16);
    }
}
