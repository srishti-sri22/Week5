use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Zero};
use super::gcd::extended_gcd;

pub fn interpolate(shares: &[(BigUint, BigUint)], q: &BigUint) -> Result<BigUint, &'static str> {
    if shares.is_empty() {
        return Err("No shares provided for interpolation");
    }

    let mut secret = BigInt::zero();
    let q_int = BigInt::from(q.clone());

    println!("Starting Lagrange interpolation...");

    for (i, (xi, yi)) in shares.iter().enumerate() {
        println!("\nProcessing share {}: x={}, y={}", i + 1, xi, yi);

        let mut numerator = BigInt::one();
        let mut denominator = BigInt::one();

        for (j, (xj, _)) in shares.iter().enumerate() {
            if i != j {
                numerator = (numerator * BigInt::from(xj.clone())).mod_floor(&q_int);
                let difference = (BigInt::from(xj.clone()) - BigInt::from(xi.clone()))
                    .mod_floor(&q_int);
                denominator = (denominator * difference).mod_floor(&q_int);
            }
        }

        println!("  numerator = {}", numerator);
        println!("  denominator = {}", denominator);

        let (gcd, inverse, _) = extended_gcd(denominator.clone(), q_int.clone());
        println!("  gcd = {}", gcd);

        if gcd != BigInt::one() {
            eprintln!("  ERROR: gcd is not 1, cannot compute modular inverse!");
            return Err("GCD of denominator and modulus is not 1");
        }

        let denominator_inv = inverse.mod_floor(&q_int);
        println!("  denominator_inv = {}", denominator_inv);

        let lagrange_coeff = (numerator * denominator_inv).mod_floor(&q_int);
        println!("  lagrange_coeff = {}", lagrange_coeff);

        let term = (BigInt::from(yi.clone()) * lagrange_coeff).mod_floor(&q_int);
        println!("  term = {}", term);

        secret = (secret + term).mod_floor(&q_int);
        println!("  running secret = {}", secret);
    }

    println!("\nFinal secret: {}", secret);

    match secret.to_biguint() {
        Some(val) => Ok(val),
        None => Err("Secret is negative, cannot convert to BigUint"),
    }
}
