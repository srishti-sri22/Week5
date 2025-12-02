use num_bigint::BigUint;
use super::lagrange::compute_lj_zero;

pub fn reconstruct_secret(shares: &[(BigUint, BigUint)], prime: &BigUint) -> BigUint {
    let mut secret = BigUint::from(0 as u32);

    for (j, (_, yj)) in shares.iter().enumerate() {
        let lj = compute_lj_zero(j, shares, prime);
        let term = (yj * lj) % prime;
        secret = (secret + term) % prime;
    }

    return secret;
}
