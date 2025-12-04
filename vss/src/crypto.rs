// src/crypto.rs
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::rngs::OsRng;
use rand::RngCore;

pub fn mod_pow(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    base.modpow(exp, modulus)
}

pub fn mod_inv(a: &BigUint, prime: &BigUint) -> BigUint {
    let one = BigUint::one();
    let exp = prime - &one - &one;
    a.modpow(&exp, prime)
}

pub fn random_biguint_below(limit: &BigUint) -> BigUint {
    let mut rng = OsRng;
    if limit.is_zero() {
        return BigUint::zero();
    }
    let mut buf = vec![0u8; (limit.bits() as usize / 8) + 1];
    loop {
        rng.fill_bytes(&mut buf);
        let candidate = BigUint::from_bytes_be(&buf) % limit;
        if candidate < *limit && !candidate.is_zero() {
            return candidate;
        }
    }
}
