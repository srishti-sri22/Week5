// commands/verify_secret.rs - Verify secret command implementation
// This file verifies that a secret matches its first commitment

use num_bigint::BigUint;
use crate::crypto::params;

/// Verify that a secret matches the first commitment
/// Checks if g^secret mod p equals the first commitment C[0]
///
/// # Arguments
/// * `secret` - The secret string to verify
/// * `commitments` - Comma-separated commitment values
pub fn execute(secret: String, commitments: String) {
    // Convert secret to a big number
    let secret_num = BigUint::from_bytes_be(secret.as_bytes());
    
    // Get cryptographic parameters
    let (p, _q, g) = params::get_fixed_params();
    
    // Parse commitments from comma-separated string
    let commitment_list: Vec<BigUint> = commitments
        .split(',')
        .map(|s| {
            BigUint::parse_bytes(s.trim().as_bytes(), 10)
                .expect("Failed to parse commitment")
        })
        .collect();
    
    println!("Verify Secret Mode");
    println!("Secret: {}", secret);
    println!("Secret as number: {}", secret_num);
    println!();
    
    // Compute g^secret mod p
    let computed_commitment = g.modpow(&secret_num, &p);
    let first_commitment = &commitment_list[0];
    
    println!("First commitment (C[0]): {}", first_commitment);
    println!("Computed (g^secret mod p): {}", computed_commitment);
    println!();
    
    // Check if they match
    if computed_commitment == *first_commitment {
        println!("✓ Secret verified successfully!");
        println!("  The secret matches the commitment.");
    } else {
        println!("✗ Secret verification failed!");
        println!("  The secret does NOT match the commitment.");
    }
}