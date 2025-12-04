use num_bigint::BigUint;
use num_traits::{Zero, One};

pub struct Polynomial {
    pub coefficients: Vec<BigUint>,
}

impl Polynomial {
    pub fn new(secret: &BigUint, degree: u32, prime: &BigUint, rng_gen: impl Fn(&BigUint) -> BigUint) -> Self {
        let mut coeffs = Vec::with_capacity(degree as usize + 1);
        coeffs.push(secret.clone());
        for _ in 0..degree {
            let c = rng_gen(prime) % prime;
            coeffs.push(c);
        }
        Polynomial { coefficients: coeffs }
    }

    pub fn evaluate(&self, x: &BigUint, prime: &BigUint) -> BigUint {
        let mut result = BigUint::zero();
        for coeff in self.coefficients.iter().rev() {
            result = (result * x + coeff) % prime;
        }
        result
    }
}
