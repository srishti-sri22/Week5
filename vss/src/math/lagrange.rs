// math/lagrange.rs - Lagrange Interpolation
// This file implements Lagrange interpolation for reconstructing secrets

use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Zero};
use super::gcd::extended_gcd;

/// Reconstruct the secret using Lagrange interpolation
/// Given k shares, we can reconstruct the polynomial's constant term (the secret)
///
/// # Arguments
/// * `shares` - A vector of (x, y) coordinate pairs representing shares
/// * `q` - The modulus for all operations
///
/// # Returns
/// The reconstructed secret (the constant term of the polynomial)
pub fn interpolate(shares: &[(BigUint, BigUint)], q: &BigUint) -> BigUint {
    let mut secret = BigInt::zero();
    let q_int = BigInt::from(q.clone());
    
    println!("Starting Lagrange interpolation...");
    
    // For each share, compute its contribution to the secret
    for (i, (xi, yi)) in shares.iter().enumerate() {
        println!("\nProcessing share {}: x={}, y={}", i + 1, xi, yi);
        
        let mut numerator = BigInt::one();
        let mut denominator = BigInt::one();
        
        // Compute the Lagrange basis polynomial at x=0
        for (j, (xj, _)) in shares.iter().enumerate() {
            if i != j {
                // Numerator: product of all (0 - xj) = product of (-xj)
                numerator = (numerator * BigInt::from(xj.clone())).mod_floor(&q_int);
                
                // Denominator: product of all (xi - xj)
                let difference = (BigInt::from(xj.clone()) - BigInt::from(xi.clone()))
                    .mod_floor(&q_int);
                denominator = (denominator * difference).mod_floor(&q_int);
            }
        }
        
        println!("  numerator = {}", numerator);
        println!("  denominator = {}", denominator);
        
        // Find the modular inverse of the denominator
        let (gcd, inverse, _) = extended_gcd(denominator.clone(), q_int.clone());
        println!("  gcd = {}", gcd);
        
        if gcd != BigInt::one() {
            eprintln!("  ERROR: gcd is not 1, cannot compute modular inverse!");
            continue;
        }
        
        let denominator_inv = inverse.mod_floor(&q_int);
        println!("  denominator_inv = {}", denominator_inv);
        
        // Compute the Lagrange coefficient
        let lagrange_coeff = (numerator * denominator_inv).mod_floor(&q_int);
        println!("  lagrange_coeff = {}", lagrange_coeff);
        
        // Add this share's contribution: yi * lagrange_coeff
        let term = (BigInt::from(yi.clone()) * lagrange_coeff).mod_floor(&q_int);
        println!("  term = {}", term);
        
        secret = (secret + term).mod_floor(&q_int);
        println!("  running secret = {}", secret);
    }
    
    println!("\nFinal secret: {}", secret);
    
    secret.to_biguint().expect("Secret should be positive")
}