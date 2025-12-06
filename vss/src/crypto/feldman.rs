use num_bigint::BigUint;
use num_traits::One;

pub fn generate_commitments(coeffs: &[BigUint], g: &BigUint, p: &BigUint) -> Vec<BigUint> {
    coeffs
        .iter()
        .map(|coefficient| g.modpow(coefficient, p))
        .collect()
}

/// The verification equation is:
/// g^y ≡ ∏(C[j]^(x^j)) (mod p)
/// Which expands to:
/// g^y ≡ C[0]^(x^0) * C[1]^(x^1) * C[2]^(x^2) * ... (mod p)
/// g^y ≡ C[0] * C[1]^x * C[2]^(x^2) * C[3]^(x^3) * ... (mod p)

pub fn verify_share(x: &BigUint,y: &BigUint,commitments: &[BigUint],g: &BigUint,p: &BigUint,_q: &BigUint) -> bool {
    let left_side = g.modpow(y, p);

    let mut right_side = BigUint::one();
    let mut x_power = BigUint::one();
    
    for commitment in commitments {
        let term = commitment.modpow(&x_power, p);
        right_side = right_side * term;
        x_power = &x_power * x;
    }
    right_side = right_side % p;
    left_side == right_side
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_verify_share_simple() {
        // Simple test with small numbers
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);
        let g = BigUint::from(2u32);
        
        // Polynomial: f(x) = 5 + 3x (coefficients: [5, 3])
        let coeffs = vec![BigUint::from(5u32), BigUint::from(3u32)];
        let commitments = generate_commitments(&coeffs, &g, &p);
        
        // Share at x=1: f(1) = 5 + 3*1 = 8 (mod 11)
        let x = BigUint::from(1u32);
        let y = BigUint::from(8u32);
        
        assert!(verify_share(&x, &y, &commitments, &g, &p, &q));
    }
    
    #[test]
    fn test_verify_share_larger_x() {
        // Test with larger x value to ensure modular arithmetic works
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);
        let g = BigUint::from(2u32);
        
        // Polynomial: f(x) = 5 + 3x (coefficients: [5, 3])
        let coeffs = vec![BigUint::from(5u32), BigUint::from(3u32)];
        let commitments = generate_commitments(&coeffs, &g, &p);
        
        // Share at x=7: f(7) = 5 + 3*7 = 26 ≡ 4 (mod 11)
        let x = BigUint::from(7u32);
        let y = BigUint::from(4u32);
        
        assert!(verify_share(&x, &y, &commitments, &g, &p, &q));
    }
    
    #[test]
    fn test_verify_share_invalid() {
        // Test that invalid shares are detected
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);
        let g = BigUint::from(2u32);
        
        // Polynomial: f(x) = 5 + 3x (coefficients: [5, 3])
        let coeffs = vec![BigUint::from(5u32), BigUint::from(3u32)];
        let commitments = generate_commitments(&coeffs, &g, &p);
        
        // Invalid share: x=1, y=7 (should be 8)
        let x = BigUint::from(1u32);
        let y = BigUint::from(7u32);
        
        assert!(!verify_share(&x, &y, &commitments, &g, &p, &q));
    }
}