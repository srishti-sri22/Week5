use num_bigint::BigUint;

pub fn get_fixed_params() -> (BigUint, BigUint, BigUint) {
    let q = BigUint::parse_bytes(b"170141183460469231731687303715884105727", 10)
        .expect("Failed to parse q");
    
    let p = &q * 2 as u32 + 1 as u32;

    let g = BigUint::from(2 as u32);
    
    (p, q, g)
}