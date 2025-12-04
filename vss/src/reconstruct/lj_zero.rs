use num_bigint::BigUint;
use num_traits::{One};
use crate::crypto::mod_inv;

pub fn compute_lj_zero(j_index: usize, shares: &[(BigUint, BigUint)], prime: &BigUint) -> BigUint {
    let (xj, _) = &shares[j_index];
    let mut num = BigUint::one();
    let mut den = BigUint::one();

    for (m, (xm, _)) in shares.iter().enumerate() {
        if m == j_index { continue; }
        num = (num * xm) % prime;
        let diff = (xm + prime - xj) % prime;
        den = (den * diff) % prime;
    }

    let den_inv = mod_inv(&den, prime);
    (num * den_inv) % prime
}
