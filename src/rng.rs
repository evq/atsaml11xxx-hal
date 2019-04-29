use core::iter;

use arrayvec::ArrayVec;
use atsaml11xxx::{MCLK, TRNG};
use hal::blocking::rng::Read;

#[cfg(feature = "rand_core")]
use rand_core::{impls, Error as RandError, RngCore, CryptoRng};

/// System random number generator `TRNG` as a random number provider
pub struct Rng {
    trng: TRNG,
}

#[derive(Debug)]
pub enum Error {}

impl Rng {
    pub fn new(trng: TRNG, mclk: &mut MCLK) -> Self {
        // Enable the clock
        mclk.apbcmask.write(|w| w.trng_().set_bit());

        // Enable random number generation
        trng.ctrla.write(|w| w.enable().set_bit());

        Rng { trng }
    }

    pub fn free(self) -> TRNG {
        self.trng
    }
}

impl Read for Rng {
    type Error = Error;

    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let data = iter::repeat_with(|| {
            while self.trng.intflag.read().bits() == 0 {};
            // A new random value has been generated
            self.trng.data.read().bits()
        }).flat_map(|data| ArrayVec::from(data.to_ne_bytes()));

        for (in_, next) in &mut buffer.into_iter().zip(data) {
            // Write number into provided buffer
            *in_ = next;
        }

        Ok(())
    }
}

#[cfg(feature = "rand_core")]
impl RngCore for Rng {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.read(dest).unwrap();
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), RandError> {
        self.fill_bytes(dest);
        Ok(())
    }

    fn next_u32(&mut self) -> u32 {
        impls::next_u32_via_fill(self)
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_fill(self)
    }
}

#[cfg(feature = "rand_core")]
impl CryptoRng for Rng {}
