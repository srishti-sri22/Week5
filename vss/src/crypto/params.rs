// crypto/params.rs - Fixed cryptographic parameters
// This file defines the mathematical parameters used in the Feldman VSS scheme

use num_bigint::BigUint;

/// Returns the fixed cryptographic parameters (p, q, g)
/// - p: A large prime number (the modulus)
/// - q: A prime number where p = 2q + 1 (Sophie Germain prime)
/// - g: Generator of the group
pub fn get_fixed_params() -> (BigUint, BigUint, BigUint) {
    // q is a large prime number
    let q = BigUint::parse_bytes(b"170141183460469231731687303715884105727", 10)
        .expect("Failed to parse q");
    
    // p = 2q + 1 (Sophie Germain prime)
    let p = &q * 2u32 + 1u32;
    
    // g is the generator (2 in this case)
    let g = BigUint::from(2u32);
    
    (p, q, g)
}