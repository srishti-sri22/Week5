use clap::{Parser};
mod secret_generation;
mod models;
use models::{Args, Commands};
use num_traits::Zero;
use secret_generation::{string_to_biguint, biguint_to_string};
use num_bigint::BigUint;

mod polynomial;
use polynomial::{
    generate_shares::generate_shares
};

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Split { secret, n, k, p } => {
            let secret_num = string_to_biguint(&secret);
            let prime = BigUint::from(p);

            if secret_num >= prime {
                eprintln!("Secret must be smaller than prime");
                std::process::exit(1);
            }

            let shares = generate_shares(&secret_num, n, k, &prime);

            println!("\nGenerated Shares");
            for (i, (x, y)) in shares.iter().enumerate() {
                println!("Share {} -> x: {}, y: {}", i + 1, x, y);
            }

            let cli_format: Vec<String> =
                shares.iter().map(|(x, y)| format!("{},{}", x, y)).collect();

            let cli_string = cli_format.join(";");

            println!("\nCLI Ready Format");
            println!("--shares \"{}\"", cli_string);
        }

        Commands::Reconstruct { prime, shares } => {
            let p = BigUint::from(prime);
            let mut shares_vec = Vec::new();

            for s in shares.split(';') {
                let s = s.trim();
                if s.is_empty() {
                    continue;
                }
                let parts: Vec<&str> = s.split(',').collect();
                let x = BigUint::parse_bytes(parts[0].trim().as_bytes(), 10).unwrap();
                let y = BigUint::parse_bytes(parts[1].trim().as_bytes(), 10).unwrap();
                shares_vec.push((x, y));
            }

            let secret_num = BigUint::zero();
            let secret_str = biguint_to_string(&secret_num);

            println!("{}", secret_str);
        }
    }
}
