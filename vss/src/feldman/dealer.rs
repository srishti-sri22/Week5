use num_bigint::BigUint;
use num_traits::Zero;
use crate::crypto::{mod_pow, random_biguint_below};

pub fn generate_polynomial(secret: &BigUint, degree: u32, prime: &BigUint) -> Vec<BigUint> {
    let mut coeffs = Vec::new();
    coeffs.push(secret.clone());
    for _ in 0..degree {
        let r = random_biguint_below(prime);
        coeffs.push(r % prime);
    }
    coeffs
}

pub fn compute_commitments(coeffs: &[BigUint], g: &BigUint, prime: &BigUint) -> Vec<BigUint> {
    coeffs.iter().map(|a| mod_pow(g, a, prime)).collect()
}

pub fn evaluate_polynomial(coeffs: &[BigUint], x: &BigUint, prime: &BigUint) -> BigUint {
    let mut result = BigUint::zero();
    for coeff in coeffs.iter().rev() {
        result = (result * x + coeff) % prime;
    }
    result
}

pub fn make_shares(
    secret: &BigUint,
    n: u32,
    k: u32,
    g: &BigUint,
    prime: &BigUint,
) -> (Vec<(BigUint, BigUint)>, Vec<BigUint>) {
    let degree = k - 1;
    let coeffs = generate_polynomial(secret, degree, prime);
    let commitments = compute_commitments(&coeffs, g, prime);
    let mut shares = Vec::new();
    for i in 1..=n {
        let x = BigUint::from(i);
        let y = evaluate_polynomial(&coeffs, &x, prime);
        shares.push((x, y));
    }
    (shares, commitments)
}
