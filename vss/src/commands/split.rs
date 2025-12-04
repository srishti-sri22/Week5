// commands/split.rs - Split command implementation
// This file handles splitting a secret into shares

use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;
use crate::crypto::{params, polynomial, feldman};

/// Split a secret into n shares where k shares are needed to reconstruct
///
/// # Arguments
/// * `secret` - The secret string to split
/// * `n` - Total number of shares to create
/// * `k` - Minimum number of shares needed to reconstruct
pub fn execute(secret: String, n: usize, k: usize) {
    // Validate input parameters
    if k > n || k == 0 || n == 0 {
        eprintln!("Error: Invalid n or k values");
        eprintln!("  - k must be <= n");
        eprintln!("  - Both k and n must be > 0");
        std::process::exit(1);
    }

    // Initialize random number generator
    let mut rng = thread_rng();
    
    // Convert secret to a big number
    let secret_num = BigUint::from_bytes_be(secret.as_bytes());
    
    // Get cryptographic parameters
    let (p, q, g) = params::get_fixed_params();

    println!("Split Mode");
    println!("Secret: {}", secret);
    println!("n={}, k={}", n, k);
    println!();

    // Create polynomial coefficients
    // First coefficient is the secret, others are random
    let mut coefficients = vec![secret_num.clone()];
    for _ in 1..k {
        // Generate random coefficient smaller than q
        coefficients.push(rng.gen_biguint(q.bits() as u64) % &q);
    }

    // Generate Feldman commitments for verification
    let commitments = feldman::generate_commitments(&coefficients, &g, &p);

    // Create shares by evaluating polynomial at points 1, 2, ..., n
    let mut shares = Vec::new();
    for i in 1..=n {
        let x = BigUint::from(i as u64);
        let y = polynomial::evaluate(&coefficients, &x, &q);
        shares.push((x, y));
    }

    // Display results
    println!("Public Parameters:");
    println!("p = {}", p);
    println!("q = {}", q);
    println!("g = {}", g);
    println!();
    
    println!("Commitments:");
    for (i, commitment) in commitments.iter().enumerate() {
        println!("C[{}] = {}", i, commitment);
    }
    println!();

    let commitments_str: Vec<String> = commitments
        .iter()
        .map(|c| c.to_string())
        .collect();
    println!("Commitments (CLI format):");
    println!("{}", commitments_str.join(","));
    println!();
    
    println!("Shares:");
    for (i, (x, y)) in shares.iter().enumerate() {
        println!("Share {}: {},{}", i + 1, x, y);
    }
    println!();

    let shares_str: Vec<String> = shares
        .iter()
        .map(|(x, y)| format!("{},{}", x, y))
        .collect();
    println!("Shares (CLI format):");
    println!("{}", shares_str.join(";"));
}