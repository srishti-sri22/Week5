use clap::{Parser, Subcommand};
mod secret_generation;
mod models;
use models::{Args, Commands};

use secret_generation::{string_to_biguint, biguint_to_string};
use num_bigint::BigUint;

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Split { secret, n, k, p } => {
            println!("Original secret string: '{}'", secret);
            let secret_num = string_to_biguint(&secret);
            println!("Secret as BigUint: {}", secret_num);
            println!("Secret bytes: {:?}", secret.as_bytes());
            
            let prime = BigUint::from(p);
            println!("Prime: {}", prime);

            if secret_num >= prime {
                eprintln!("Error: Secret must be smaller than prime {}!", p);
                eprintln!("Secret value: {}", secret_num);
                std::process::exit(1);
            }   
           
        }
        Commands::Reconstruct { prime, shares } => {
            println!("Prime: {}", prime);
            println!("Shares string: '{}'", shares);
            
            let p = BigUint::from(prime);
            let shares_vec: Vec<(String, String)> = Vec::new();
            
            println!("Parsed {} shares:", shares_vec.len());
            for (i, (x, y)) in shares_vec.iter().enumerate() {
                println!("  Share {}: ({}, {})", i + 1, x, y);
            }
            let secret_num = BigUint::from(12334534 as u128);
            let secret_str = biguint_to_string(&secret_num);
            println!("Reconstructed secret as string: '{}'", secret_str);
            println!("String bytes: {:?}", secret_str.as_bytes());
        }
    }
}