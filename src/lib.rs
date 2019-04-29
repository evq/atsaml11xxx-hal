#![no_std]
#![feature(const_transmute)]

#[cfg(test)]
#[macro_use]
extern crate std;

pub extern crate atsaml11xxx;
pub extern crate embedded_hal as hal;

extern crate cortex_m;

extern crate arrayvec;

#[cfg(feature = "rand_core")]
extern crate rand_core;

extern crate nb;

#[cfg(test)]
extern crate crypto as test_crypto;
#[cfg(test)]
#[macro_use]
extern crate hex_literal;

pub mod gpio;
pub mod rng;
pub mod sercom;

pub mod clock;
pub mod delay;
pub mod prelude;
pub mod time;

pub mod crypto;
