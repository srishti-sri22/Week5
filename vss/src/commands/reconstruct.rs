use num_bigint::BigUint;
use crate::crypto::params;
use crate::math::lagrange;

pub fn execute(shares: String) {
    let (_p, q, _g) = params::get_fixed_params();

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
            
            (x, y) //hm yaha tuple return krenge
        })
        .collect();
    
    println!("Reconstruct Mode");
    println!("Shares provided: {}", share_list.len());
    println!();

    println!("Using q = {}", q);
    for (i, (x, y)) in share_list.iter().enumerate() {
        println!("Share {}: x={}, y={}", i + 1, x, y);
    }
    println!();

    let reconstructed = lagrange::interpolate(&share_list, &q);
    
    println!("Secret (number): {}", reconstructed);

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