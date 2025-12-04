// math/gcd.rs - Extended Euclidean Algorithm
// This file implements the extended GCD algorithm for computing modular inverses

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

/// Extended Euclidean Algorithm
/// Computes gcd(a, b) and finds x, y such that ax + by = gcd(a, b)
/// This is used to find modular inverses
///
/// # Arguments
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
/// A tuple (gcd, x, y) where gcd = ax + by
pub fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        // Base case: gcd(a, 0) = a, and a = a*1 + 0*0
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        // Recursive case
        let (gcd, x1, y1) = extended_gcd(b.clone(), a.mod_floor(&b));
        
        // Update x and y based on recursive result
        let x = y1.clone();
        let y = x1 - (a / b) * y1;
        
        (gcd, x, y)
    }
}