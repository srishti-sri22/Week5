// commands/reconstruct.rs - Reconstruct secret command implementation
// This file reconstructs the original secret from k or more shares

use num_bigint::BigUint;
use crate::crypto::params;
use crate::math::lagrange;

/// Reconstruct the original secret from shares
///
/// # Arguments
/// * `shares` - Semicolon-separated list of shares (format: "x1,y1;x2,y2;...")
pub fn execute(shares: String) {
    // Get cryptographic parameters
    let (_p, q, _g) = params::get_fixed_params();
    
    // Parse shares from semicolon-separated string
    let share_list: Vec<(BigUint, BigUint)> = shares
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
    
    println!("Reconstruct Mode");
    println!("Shares provided: {}", share_list.len());
    println!();
    
    // Show the parameters being used
    println!("Using q = {}", q);
    for (i, (x, y)) in share_list.iter().enumerate() {
        println!("Share {}: x={}, y={}", i + 1, x, y);
    }
    println!();
    
    // Perform Lagrange interpolation to reconstruct the secret
    let reconstructed = lagrange::interpolate(&share_list, &q);
    
    println!("Secret (number): {}", reconstructed);
    
    // Try to convert the number back to a string
    let secret_bytes = reconstructed.to_bytes_be();
    match String::from_utf8(secret_bytes.clone()) {
        Ok(secret_string) => {
            println!("Secret (string): {}", secret_string);
        }
        Err(_) => {
            println!("Secret (string): [Invalid UTF-8]");
            println!("  The reconstructed secret is not valid UTF-8 text.");
        }
    }
}