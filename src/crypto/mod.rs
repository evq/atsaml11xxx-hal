use core::iter;

use arrayvec::ArrayVec;

#[cfg(test)]
use test_crypto::sha2::sha256_digest_block;

const ROM_CRYPTO_API: *const RomCryptoApi = 0x02001900 as *const RomCryptoApi;

const SHA256_STATE_LEN: usize = 8;
const SHA256_RAM_BUFFER_LEN: usize = 64;
const SHA256_BLOCKSIZE_BYTES: usize = 64;

#[repr(C)]
struct RomCryptoApi {
    /// CRYA SHA function.
    /// typedef void (*crya_sha_process_t) (uint32_t hash_in_out[8], const uint8_t data[64], uint32_t ram_buf[64]);
    /// hash_in_out[In/out]: A pointer to a hash location (hash input and output)
    /// data[In]: A pointer to a 512 bit data block
    /// ram_buf[In]: A pointer to a RAM buffer (256B needed for internal algorithm)
    crya_sha_process: extern "C" fn(hash_in_out: *mut u32, data: *const u8, ram_buf: *mut u32),
}

impl RomCryptoApi {
    fn api_table() -> &'static RomCryptoApi {
        unsafe { &*(ROM_CRYPTO_API) }
    }

    fn sha256_process_block(
        &self,
        hash_in_out: &mut [u32; SHA256_STATE_LEN],
        block: &[u8],
        ram_buf: &mut [u32; SHA256_RAM_BUFFER_LEN],
    ) {
        (self.crya_sha_process)(
            hash_in_out.as_mut_ptr(),
            block.as_ptr(),
            ram_buf.as_mut_ptr(),
        );
    }
}

static H256: [u32; SHA256_STATE_LEN] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

pub struct Sha256 {
    state: [u32; SHA256_STATE_LEN],
    ram_buffer: [u32; SHA256_RAM_BUFFER_LEN],
    block_buffer: ArrayVec<[u8; SHA256_BLOCKSIZE_BYTES]>,
    bit_length: u64,
}

impl Sha256 {
    pub fn new() -> Self {
        Sha256 {
            state: H256,
            ram_buffer: [0u32; SHA256_RAM_BUFFER_LEN],
            block_buffer: ArrayVec::<[u8; SHA256_BLOCKSIZE_BYTES]>::new(),
            bit_length: 0,
        }
    }

    #[cfg(not(test))]
    fn process_block(&mut self, block: &[u8]) {
        let rom_api = RomCryptoApi::api_table();
        //println!("ROM I2C API address is: {:p}", rom_api);

        rom_api.sha256_process_block(&mut self.state, block, &mut self.ram_buffer);
    }

    #[cfg(test)]
    fn process_block(&mut self, block: &[u8]) {
        sha256_digest_block(&mut self.state, block);
    }

    #[cfg(not(test))]
    fn process_buffer(&mut self) {
        let rom_api = RomCryptoApi::api_table();
        //println!("ROM I2C API address is: {:p}", rom_api);

        rom_api.sha256_process_block(&mut self.state, &self.block_buffer, &mut self.ram_buffer);
    }

    #[cfg(test)]
    fn process_buffer(&mut self) {
        sha256_digest_block(&mut self.state, &self.block_buffer);
    }

    #[cfg(test)]
    pub fn input(&mut self, data: &[u8]) {
        self.bit_length += (data.len() as u64) * 8;
        let mut data_in = data;

        if self.block_buffer.len() > 0 {
            let mut remaining = self.block_buffer.capacity() - self.block_buffer.len();
            if data_in.len() < remaining {
                remaining = data_in.len();
            }

            let (left, right) = data_in.split_at(remaining);
            self.block_buffer.extend(left.iter().cloned());
            data_in = right;

            if self.block_buffer.len() == SHA256_RAM_BUFFER_LEN {
                self.process_buffer();
                self.block_buffer.clear();
            }

            if data_in.is_empty() {
                return;
            }
        }

        let mut iter = data_in.chunks_exact(SHA256_BLOCKSIZE_BYTES);

        for block in iter.by_ref() {
            self.process_block(block);
        }

        self.block_buffer.extend(iter.remainder().iter().cloned());
    }

    #[cfg(test)]
    pub fn result(mut self) -> [u8; SHA256_STATE_LEN * 4] {
        self.block_buffer.push(0x80);

        let remaining = self.block_buffer.capacity() - self.block_buffer.len();
        if remaining < 8 {
            self.block_buffer.extend(iter::repeat(0u8).take(remaining));

            self.process_buffer();

            self.block_buffer.clear();

            self.block_buffer
                .extend(iter::repeat(0u8).take(SHA256_BLOCKSIZE_BYTES - 8));
        } else {
            self.block_buffer
                .extend(iter::repeat(0u8).take(remaining - 8));
        }
        self.block_buffer
            .extend(self.bit_length.to_be_bytes().iter().cloned());

        self.process_buffer();

        let mut ret = [0u8; SHA256_STATE_LEN * 4];
        let mut tmp = ArrayVec::<[u8; SHA256_STATE_LEN * 4]>::new();
        for u32tmp in &self.state {
            for u8tmp in &u32tmp.to_be_bytes() {
                tmp.push(*u8tmp);
            }
        }
        ret.copy_from_slice(&tmp);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let rom_api = RomCryptoApi::api_table();
        println!("{:p}", rom_api.crya_sha_process);
    }

    #[test]
    fn test_sha256() {
        let mut hasher = Sha256::new();
        hasher.input(b"hello world");
        let result = hasher.result();
        assert_eq!(
            result[..],
            hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")[..]
        );

        let mut hasher = Sha256::new();
        hasher.input(b"hello");
        hasher.input(b" world");
        let result = hasher.result();
        assert_eq!(
            result[..],
            hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")[..]
        );

        let mut hasher = Sha256::new();
        hasher.input(b"");
        let result = hasher.result();
        assert_eq!(
            result[..],
            hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")[..]
        );

        let mut hasher = Sha256::new();
        hasher.input(b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
        let result = hasher.result();
        assert_eq!(
            result[..],
            hex!("2d8c2f6d978ca21712b5f6de36c9d31fa8e96a4fa5d8ff8b0188dfb9e7c171bb")[..]
        );

        let mut hasher = Sha256::new();
        hasher.input(b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est");
        hasher.input(b" laborum.");
        let result = hasher.result();
        assert_eq!(
            result[..],
            hex!("2d8c2f6d978ca21712b5f6de36c9d31fa8e96a4fa5d8ff8b0188dfb9e7c171bb")[..]
        );
    }
}
