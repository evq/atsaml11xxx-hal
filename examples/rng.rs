#![no_main]
#![no_std]
extern crate atsaml11xxx_hal as hal;

extern crate cortex_m_rt;
// makes `panic!` print messages to the host stderr using semihosting
extern crate panic_semihosting;

extern crate cortex_m_semihosting;

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

use hal::atsaml11xxx;
use hal::rng::Rng;
use hal::hal::blocking::rng::Read;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    let mut p = atsaml11xxx::Peripherals::take().unwrap();

    write!(stdout, "initializing rng\n").unwrap();

    let mut rng = Rng::new(p.TRNG, &mut p.MCLK);

    let mut buf: [u8; 4] = [0; 4];

    write!(stdout, "reading rng\n").unwrap();

    match rng.read(&mut buf) {
        Ok(()) => write!(stdout, "Random number: {}\n", buf[0]),
        Err(_err) => write!(stdout, "ERROR occurred\n"),
    }.unwrap();
    loop {}
}
