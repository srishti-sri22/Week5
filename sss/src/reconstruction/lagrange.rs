use num_bigint::BigUint;

pub fn compute_lj_zero(j_index: usize, shares: &[(BigUint, BigUint)], prime: &BigUint) -> BigUint {
    let (xj, _) = &shares[j_index];

    let mut num = BigUint::from(1 as u32);
    let mut den = BigUint::from(1 as u32);

    for (m, (xm, _)) in shares.iter().enumerate() {
        if m != j_index {
            num = (num * xm) % prime;
            let diff = (xm + prime - xj) % prime;
            den = (den * diff) % prime;
        }
    }

    let den_inv = den.modpow(&(prime - 2u32), prime);
    return (num * den_inv) % prime ;
}
