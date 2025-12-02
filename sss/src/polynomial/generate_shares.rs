use num_bigint::BigUint;
use crate::polynomial::polynomial::Polynomial;

pub fn generate_shares(
    secret: &BigUint,
    n: u32,
    k: u32,
    prime: &BigUint,
) -> Vec<(BigUint, BigUint)> {
    let poly = Polynomial::new(secret, k - 1, prime);

    let mut shares = Vec::new();

    for i in 1..=n {
        let x = BigUint::from(i);
        let y = poly.evaluate(&x, prime);
        shares.push((x, y));
    }

    return shares;
}
