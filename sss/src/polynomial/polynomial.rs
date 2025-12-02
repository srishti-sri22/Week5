use num_bigint::BigUint;
use num_traits::Zero;
use rand::Rng;

pub struct Polynomial {
    coefficients: Vec<BigUint>,
}

impl Polynomial {
    pub fn new(secret: &BigUint, degree: u32, prime: &BigUint) -> Self {
        let mut coefficients = vec![secret.clone()];
        let mut rng = rand::thread_rng();
        
        for _ in 1..=degree {
            let coeff = BigUint::from(rng.gen_range(1..prime.to_u32_digits()[0] as u128));
            coefficients.push(coeff % prime);
        }
        
        Polynomial { coefficients }
    }

    pub fn evaluate(&self, x: &BigUint, prime: &BigUint) -> BigUint {
        let mut result = BigUint::zero();
        let mut x_power = BigUint::from(1 as u32);
        
        for coeff in &self.coefficients {
            let term = (coeff * &x_power) % prime;
            result = (result + term) % prime;
            x_power = (x_power * x) % prime;
        }
        
        result
    }
      
}
