use num_bigint::BigUint;
use num_traits::{One, Zero};

pub fn evaluate(coeffs: &[BigUint], x: &BigUint, q: &BigUint) -> BigUint {
    let mut result = BigUint::zero();
    let mut power = BigUint::one(); // x^0 = 1
    
    for coefficient in coeffs {
        result = (result + (coefficient * &power)) % q;
        power = power * x; 
    }
    
    result
}