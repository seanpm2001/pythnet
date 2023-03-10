use crate::hashers::Hasher;
use {
    sha3::{Digest, Sha3_256},
    slow_primes::is_prime_miller_rabin,
};
#[derive(Clone, Default)]
pub struct PrimeHasher {}

impl Hasher for PrimeHasher {
    type Hash = u128;

    fn hash(data: &[u8]) -> u128 {
        // Scan for prime's generated by hashing the bytes starting from 0. We use a number like
        // this so once the prime is found we can directly compute the hash instead of scanning
        // the range again.
        let mut search = 0usize;

        loop {
            // Increment Search Counter.
            search += 1;

            // Hash Input.
            let mut hasher = sha3::Sha3_256::new();
            hasher.update(data);
            hasher.update(&search.to_be_bytes());
            let hash_bytes: [u8; 32] = hasher.finalize().into();

            // Take only a u32 from the end, return if it's prime.
            let prime = u32::from_be_bytes(hash_bytes[28..].try_into().unwrap()) | 1;
            if is_prime_miller_rabin(prime as u64) {
                return prime as u128;
            }
        }
    }

    fn hashv(data: &[&[u8]]) -> u128 {
        // Scan for prime's generated by hashing the bytes starting from 0. We use a number like
        // this so once the prime is found we can directly compute the hash instead of scanning
        // the range again.
        let mut search = 0usize;

        loop {
            // Increment Search Counter.
            search += 1;

            // Hash Input.
            let mut hasher = sha3::Sha3_256::new();
            for d in data {
                hasher.update(d);
            }
            hasher.update(&search.to_be_bytes());
            let hash_bytes: [u8; 32] = hasher.finalize().into();

            // Take only a u32 from the end, return if it's prime.
            let prime = u32::from_be_bytes(hash_bytes[28..].try_into().unwrap()) | 1;
            if is_prime_miller_rabin(prime as u64) {
                return prime as u128;
            }
        }
    }
}
