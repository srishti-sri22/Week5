// crypto/polynomial.rs - Polynomial evaluation
// This file handles polynomial operations for secret sharing

use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Evaluate a polynomial at a given point x
/// The polynomial is defined by its coefficients: a0 + a1*x + a2*x^2 + ...
/// All operations are done modulo q
///
/// # Arguments
/// * `coeffs` - The coefficients of the polynomial [a0, a1, a2, ...]
/// * `x` - The point at which to evaluate the polynomial
/// * `q` - The modulus for all operations
///
/// # Returns
/// The value of the polynomial at point x, modulo q
pub fn evaluate(coeffs: &[BigUint], x: &BigUint, q: &BigUint) -> BigUint {
    let mut result = BigUint::zero();
    let mut power = BigUint::one(); // x^0 = 1
    
    // For each coefficient, add (coefficient * x^i) to the result
    for coefficient in coeffs {
        result = (result + (coefficient * &power)) % q;
        power = (power * x) % q; // Compute next power of x
    }
    
    result
}