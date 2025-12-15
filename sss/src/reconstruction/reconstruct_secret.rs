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

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    #[test]
    fn test_reconstruct_secret_single_share() {
        let prime = BigUint::from_u32(101).unwrap();
        let shares = vec![(BigUint::from_u32(1).unwrap(), BigUint::from_u32(42).unwrap())];
        let secret = reconstruct_secret(&shares, &prime);
        assert_eq!(secret, BigUint::from_u32(42).unwrap());
    }

     #[test]
    fn test_reconstruct_secret_three_shares() {
        let prime = BigUint::from_u32(101).unwrap();
        let shares = vec![
            (BigUint::from_u32(1).unwrap(), BigUint::from_u32(42).unwrap()),
            (BigUint::from_u32(2).unwrap(), BigUint::from_u32(84).unwrap()),
            (BigUint::from_u32(3).unwrap(), BigUint::from_u32(21).unwrap()),
        ];
        let secret = reconstruct_secret(&shares, &prime);
        assert_ne!(secret, BigUint::from_u32(42).unwrap());
    }
}