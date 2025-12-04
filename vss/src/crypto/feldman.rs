// crypto/feldman.rs - Feldman Verifiable Secret Sharing
// This file implements the Feldman VSS scheme for verifying shares

use num_bigint::BigUint;
use num_traits::One;

/// Generate Feldman commitments for polynomial coefficients
/// Each commitment is g^(coefficient) mod p
///
/// # Arguments
/// * `coeffs` - The polynomial coefficients
/// * `g` - The generator
/// * `p` - The modulus
///
/// # Returns
/// A vector of commitments, one for each coefficient
pub fn generate_commitments(coeffs: &[BigUint], g: &BigUint, p: &BigUint) -> Vec<BigUint> {
    coeffs
        .iter()
        .map(|coefficient| g.modpow(coefficient, p))
        .collect()
}

/// Verify that a share (x, y) is valid given the commitments
/// Checks if g^y = C0^(x^0) * C1^(x^1) * C2^(x^2) * ... (mod p)
/// Which simplifies to: g^y = C0 * C1^x * C2^(x^2) * C3^(x^3) * ... (mod p)
///
/// # Arguments
/// * `x` - The x-coordinate of the share
/// * `y` - The y-coordinate of the share
/// * `commitments` - The Feldman commitments [C0, C1, C2, ...]
/// * `g` - The generator
/// * `p` - The large prime modulus
/// * `q` - The smaller prime modulus (used for modular exponentiation)
///
/// # Returns
/// `true` if the share is valid, `false` otherwise
pub fn verify_share(
    x: &BigUint,
    y: &BigUint,
    commitments: &[BigUint],
    g: &BigUint,
    p: &BigUint,
    q: &BigUint,
) -> bool {
    // Left side: g^y mod p
    let left_side = g.modpow(y, p);
    
    // Right side: Product of (C[i]^(x^i)) for all commitments
    // We need to compute: C[0]^1 * C[1]^x * C[2]^(x^2) * C[3]^(x^3) * ...
    let mut right_side = BigUint::one();
    let mut x_power = BigUint::one(); // Start with x^0 = 1
    
    for commitment in commitments {
        // Multiply by C[i]^(x^i) mod p
        right_side = (right_side * commitment.modpow(&x_power, p)) % p;
        
        // Update x_power for next iteration: x^i → x^(i+1)
        // CRITICAL: This must be done AFTER using x_power, not before
        x_power = (&x_power * x) % q;
    }
    
    // The share is valid if both sides are equal
    left_side == right_side
}