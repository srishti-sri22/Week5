// commands/verify_share.rs - Verify share command implementation
// This file verifies that shares are valid using Feldman VSS

use num_bigint::BigUint;
use num_traits::One;
use crate::crypto::{params, feldman};

/// Verify one or more shares given the commitments
///
/// # Arguments
/// * `share` - Semicolon-separated shares in format "x1,y1;x2,y2;..."
/// * `commitments` - Comma-separated commitment values
/// * `verbose` - Show detailed verification steps
pub fn execute(share: String, commitments: String, verbose: bool) {
    // Get cryptographic parameters
    let (p, q, g) = params::get_fixed_params();
    
    // Parse shares - support single share or multiple shares separated by semicolons
    let share_list: Vec<(BigUint, BigUint)> = share
        .split(';')
        .map(|s| {
            let parts: Vec<&str> = s.trim().split(',').collect();
            if parts.len() != 2 {
                eprintln!("Error: Each share must be in format 'x,y'");
                std::process::exit(1);
            }
            
            let x = BigUint::parse_bytes(parts[0].as_bytes(), 10)
                .expect("Failed to parse x coordinate");
            let y = BigUint::parse_bytes(parts[1].as_bytes(), 10)
                .expect("Failed to parse y coordinate");
            
            (x, y)
        })
        .collect();
    
    // Parse commitments from comma-separated string
    let commitment_list: Vec<BigUint> = commitments
        .split(',')
        .map(|s| {
            BigUint::parse_bytes(s.trim().as_bytes(), 10)
                .expect("Failed to parse commitment")
        })
        .collect();
    
    println!("Verify Share Mode");
    println!("Shares to verify: {}", share_list.len());
    println!("Commitments: {}", commitment_list.len());
    println!();
    
    // Verify each share
    let mut all_valid = true;
    for (idx, (x, y)) in share_list.iter().enumerate() {
        println!("Verifying Share {}: ({}, {})", idx + 1, x, y);
        
        if verbose {
            // Show detailed verification steps
            let left_side = g.modpow(y, &p);
            println!("  Left side (g^y mod p): {}", left_side);
            
            let mut right_side = BigUint::one();
            let mut x_power = BigUint::one();
            
            for (i, commitment) in commitment_list.iter().enumerate() {
                println!("  Step {}: x^{} = {}", i, i, x_power);
                let term = commitment.modpow(&x_power, &p);
                println!("    C[{}]^(x^{}) mod p = {}", i, i, term);
                right_side = (right_side * term) % &p;
                println!("    Running product = {}", right_side);
                x_power = (&x_power * x) % &q;
            }
            
            println!("  Right side: {}", right_side);
            println!("  Match: {}", left_side == right_side);
        }
        
        let is_valid = feldman::verify_share(x, y, &commitment_list, &g, &p, &q);
        
        if is_valid {
            println!("  ✓ Valid\n");
        } else {
            println!("  ✗ Invalid\n");
            all_valid = false;
        }
    }
    
    println!("═══════════════════════════════════════");
    if all_valid {
        println!("✓ All shares verified successfully!");
        println!("  All shares are valid and can be used for reconstruction.");
    } else {
        println!("✗ Some shares failed verification!");
        println!("  Invalid shares cannot be used for reconstruction.");
    }
}