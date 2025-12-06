use num_bigint::BigUint;
use crate::crypto::params;

pub fn execute(secret: String, commitments: String) {

    let secret_num = BigUint::from_bytes_be(secret.as_bytes());

    let (p, _q, g) = params::get_fixed_params();

    let commitment_list: Vec<BigUint> = commitments
        .split(',')
        .map(|s| {
            BigUint::parse_bytes(s.trim().as_bytes(), 10)
                .expect("Failed to parse commitment to BigUint")
        })
        .collect();
    
    println!("Verify Secret Mode");
    println!("Secret: {}", secret);
    println!("Secret as number: {}", secret_num);
    println!();
    
    let computed_commitment = g.modpow(&secret_num, &p);
    let first_commitment = &commitment_list[0];
    
    println!("First commitment (C[0]): {}", first_commitment);
    println!("Computed (g^secret mod p): {}", computed_commitment);
    println!();

    if computed_commitment == *first_commitment {
        println!("✓ Secret verified successfully!");
        println!("  The secret matches the commitment.");
    } else {
        println!("✗ Secret verification failed!");
        println!("  The secret does NOT match the commitment.");
    }
}