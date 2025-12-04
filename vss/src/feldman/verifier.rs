// src/feldman/verifier.rs
use num_bigint::BigUint;
use num_traits::{One, Zero};
use crate::crypto::mod_pow;

pub fn verify_share(share: &(BigUint, BigUint), commitments: &[BigUint], g: &BigUint, prime: &BigUint) -> bool {
    let (x, y) = share;
    let lhs = mod_pow(g, y, prime);
    let mut rhs = BigUint::one();
    let order = prime - BigUint::one();
    for (j, c) in commitments.iter().enumerate() {
        let j_big = BigUint::from(j as u32);
        let exp = x.modpow(&j_big, &order);
        let term = mod_pow(c, &exp, prime);
        rhs = (rhs * term) % prime;
    }
    lhs == rhs
}

pub fn verify_all(shares: &[(BigUint, BigUint)], commitments: &[BigUint], g: &BigUint, prime: &BigUint) -> Vec<bool> {
    shares.iter().map(|s| verify_share(s, commitments, g, prime)).collect()
}
