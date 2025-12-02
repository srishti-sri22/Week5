use num_bigint::BigUint;

pub fn string_to_biguint(s: &str) -> BigUint {
    let mut result = BigUint::from(0u32);
    
    for ch in s.chars() {
        result = result * BigUint::from(256u32);
        result = result + BigUint::from(ch as u32);
    }
    
    result
}

pub fn biguint_to_string(n: &BigUint) -> String {
    let mut n = n.clone();
    let mut bytes = Vec::new();
    
    while n > BigUint::from(0u32) {
        let byte = (&n % BigUint::from(256u32)).to_u32_digits()[0] as u8;
        bytes.push(byte);
        n = n / BigUint::from(256u32);
    }
    
    bytes.reverse();
    
    String::from_utf8_lossy(&bytes).to_string()
}