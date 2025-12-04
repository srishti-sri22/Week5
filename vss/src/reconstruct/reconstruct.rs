use num_bigint::BigUint;
use num_traits::Zero;
use crate::reconstruct::lj_zero::compute_lj_zero;

pub fn reconstruct_secret(shares: &[(BigUint, BigUint)], prime: &BigUint) -> BigUint {
    let mut secret = BigUint::zero();
    for (j, (_, yj)) in shares.iter().enumerate() {
        let lj = compute_lj_zero(j, shares, prime);
        let term = (yj * lj) % prime;
        secret = (secret + term) % prime;
    }
    secret
}
