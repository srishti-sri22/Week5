use clap::{Parser, Subcommand};
use num_bigint::{BigInt, BigUint, RandBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::thread_rng;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Split {
        #[arg(long)]
        secret: String,
        #[arg(long)]
        n: usize,
        #[arg(long)]
        k: usize,
    },
    VerifySecret {
        #[arg(long)]
        secret: String,
        #[arg(long)]
        commitments: String,
    },
    VerifyShare {
        #[arg(long)]
        share: String,
        #[arg(long)]
        commitments: String,
    },
    Reconstruct {
        #[arg(long)]
        shares: String,
    },
}

fn get_fixed_params() -> (BigUint, BigUint, BigUint) {
    let q = BigUint::parse_bytes(b"170141183460469231731687303715884105727", 10).unwrap(); 
    let p = &q * 2u32 + 1u32;
    let g = BigUint::from(2u32);
    (p, q, g)
}

fn poly_eval(coeffs: &[BigUint], x: &BigUint, q: &BigUint) -> BigUint {
    let mut result = BigUint::zero();
    let mut pow = BigUint::one();
    for c in coeffs {
        result = (result + (c * &pow)) % q;
        pow = (pow * x) % q;
    }
    result
}

fn feldman_commitments(coeffs: &[BigUint], g: &BigUint, p: &BigUint) -> Vec<BigUint> {
    coeffs.iter().map(|a| g.modpow(a, p)).collect()
}

fn __verify_share(x: &BigUint, y: &BigUint, commits: &[BigUint], g: &BigUint, p: &BigUint, q: &BigUint) -> bool {
    let left = g.modpow(y, p);
    let mut right = BigUint::one();
    let mut pow = BigUint::one();
    for c in commits {
        right = (right * c.modpow(&pow, p)) % p;
        pow = (pow * x) % q;
    }
    left == right
}

fn lagrange_interpolate(shares: &[(BigUint, BigUint)], q: &BigUint) -> BigUint {
    let mut secret = BigInt::zero();
    let q_int = BigInt::from(q.clone());
    
    println!("Starting Lagrange interpolation...");
    
    for (i, (xi, yi)) in shares.iter().enumerate() {
        println!("\nProcessing share {}: x={}, y={}", i + 1, xi, yi);
        
        let mut numerator = BigInt::one();
        let mut denominator = BigInt::one();
        
        for (j, (xj, _)) in shares.iter().enumerate() {
            if i != j {

                numerator = (numerator * BigInt::from(xj.clone())).mod_floor(&q_int);

                let diff = (BigInt::from(xj.clone()) - BigInt::from(xi.clone())).mod_floor(&q_int);
                denominator = (denominator * diff).mod_floor(&q_int);
            }
        }
        
        println!("  numerator = {}", numerator);
        println!("  denominator = {}", denominator);

        let (g, inv, _) = extended_gcd(denominator.clone(), q_int.clone());
        println!("  gcd = {}", g);
        
        if g != BigInt::one() {
            eprintln!("  ERROR: gcd is not 1, cannot compute modular inverse!");
            continue;
        }
        
        let denominator_inv = inv.mod_floor(&q_int);
        println!("  denominator_inv = {}", denominator_inv);
        
        let lagrange_coeff = (numerator * denominator_inv).mod_floor(&q_int);
        println!("  lagrange_coeff = {}", lagrange_coeff);
        
        let term = (BigInt::from(yi.clone()) * lagrange_coeff).mod_floor(&q_int);
        println!("  term = {}", term);
        
        secret = (secret + term).mod_floor(&q_int);
        println!("  running secret = {}", secret);
    }
    
    println!("\nFinal secret: {}", secret);
    
    secret.to_biguint().expect("Secret should be positive")
}

fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (g, x1, y1) = extended_gcd(b.clone(), a.mod_floor(&b));
        let x = y1.clone();
        let y = x1 - (a / b) * y1;
        (g, x, y)
    }
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Command::Split { secret, n, k } => {
            if k > n || k == 0 || n == 0 {
                eprintln!("Error: Invalid n or k values");
                std::process::exit(1);
            }

            let mut rng = thread_rng();
            let secret_num = BigUint::from_bytes_be(secret.as_bytes());
            let (p, q, g) = get_fixed_params();

            println!("Split Mode");
            println!("Secret: {}", secret);
            println!("n={}, k={}", n, k);
            println!();

            let mut coeffs = vec![secret_num.clone()];
            for _ in 1..k {
                coeffs.push(rng.gen_biguint(q.bits() as u64) % &q);
            }

            let commits = feldman_commitments(&coeffs, &g, &p);

            let mut shares = Vec::new();
            for i in 1..=n {
                let x = BigUint::from(i as u64);
                let y = poly_eval(&coeffs, &x, &q);
                shares.push((x, y));
            }

            println!("Public Parameters:");
            println!("p = {}", p);
            println!("q = {}", q);
            println!("g = {}", g);
            println!();
            
            println!("Commitments:");
            for (i, c) in commits.iter().enumerate() {
                println!("C[{}] = {}", i, c);
            }
            println!();

            let commits_str: Vec<String> = commits.iter().map(|c| c.to_string()).collect();
            println!("Commitments (CLI format):");
            println!("{}", commits_str.join(","));
            println!();
            
            println!("Shares:");
            for (i, (x, y)) in shares.iter().enumerate() {
                println!("Share {}: {},{}", i + 1, x, y);
            }
            println!();

            let shares_str: Vec<String> = shares.iter().map(|(x, y)| format!("{},{}", x, y)).collect();
            println!("Shares (CLI format):");
            println!("{}", shares_str.join(";"));
        }

        Command::VerifySecret { secret, commitments } => {
            let secret_num = BigUint::from_bytes_be(secret.as_bytes());
            let (p, _q, g) = get_fixed_params();
            
            let commits: Vec<BigUint> = commitments
                .split(',')
                .map(|s| BigUint::parse_bytes(s.trim().as_bytes(), 10)
                    .expect("Failed to parse commitment"))
                .collect();
            
            println!("Verify Secret Mode");
            println!("Secret: {}", secret);
            println!("Secret as number: {}", secret_num);
            println!();
            
            let computed_commit = g.modpow(&secret_num, &p);
            let first_commit = &commits[0];
            
            println!("First commitment: {}", first_commit);
            println!("Computed g^secret mod p: {}", computed_commit);
            println!();
            
            if computed_commit == *first_commit {
                println!("✓ Secret verified successfully!");
            } else {
                println!("✗ Secret verification failed!");
            }
        }

        Command::VerifyShare { share, commitments } => {
            let (p, q, g) = get_fixed_params();
            
            let parts: Vec<&str> = share.split(',').collect();
            if parts.len() != 2 {
                eprintln!("Error: Share must be in format 'x,y'");
                std::process::exit(1);
            }
            
            let x = BigUint::parse_bytes(parts[0].as_bytes(), 10)
                .expect("Failed to parse x");
            let y = BigUint::parse_bytes(parts[1].as_bytes(), 10)
                .expect("Failed to parse y");
            
            let commits: Vec<BigUint> = commitments
                .split(',')
                .map(|s| BigUint::parse_bytes(s.trim().as_bytes(), 10)
                    .expect("Failed to parse commitment"))
                .collect();
            
            println!("Verify Share Mode");
            println!("Share: ({}, {})", x, y);
            println!("Commitments: {} total", commits.len());
            println!();
            
            // Debug: Show the verification computation
            let left = g.modpow(&y, &p);
            println!("Left side (g^y mod p): {}", left);
            
            let mut right = BigUint::one();
            let mut pow = BigUint::one();
            for (i, c) in commits.iter().enumerate() {
                println!("Step {}: pow = {}, C[{}] = {}", i, pow, i, c);
                let term = c.modpow(&pow, &p);
                println!("  C[{}]^pow mod p = {}", i, term);
                right = (right * term) % &p;
                println!("  right = {}", right);
                pow = (&pow * &x) % &q;
            }
            
            println!();
            println!("Final left: {}", left);
            println!("Final right: {}", right);
            println!();
            
            let is_valid = left == right;
            
            if is_valid {
                println!("✓ Share verified successfully!");
            } else {
                println!("✗ Share verification failed!");
            }
        }
        
        Command::Reconstruct { shares } => {
            let (_p, q, _g) = get_fixed_params();
            
            let share_list: Vec<(BigUint, BigUint)> = shares
                .split(';')
                .map(|s| {
                    let parts: Vec<&str> = s.trim().split(',').collect();
                    if parts.len() != 2 {
                        eprintln!("Error: Each share must be in format 'x,y'");
                        std::process::exit(1);
                    }
                    let x = BigUint::parse_bytes(parts[0].as_bytes(), 10)
                        .expect("Failed to parse x");
                    let y = BigUint::parse_bytes(parts[1].as_bytes(), 10)
                        .expect("Failed to parse y");
                    (x, y)
                })
                .collect();
            
            println!("Reconstruct Mode");
            println!("Shares provided: {}", share_list.len());
            println!();
            
            // Debug output
            println!("Using q = {}", q);
            for (i, (x, y)) in share_list.iter().enumerate() {
                println!("Share {}: x={}, y={}", i + 1, x, y);
            }
            println!();
            
            let reconstructed = lagrange_interpolate(&share_list, &q);
            
            println!("Secret (number): {}", reconstructed);
            
            let secret_bytes = reconstructed.to_bytes_be();
            match String::from_utf8(secret_bytes.clone()) {
                Ok(secret_str) => {
                    println!("Secret (string): {}", secret_str);
                }
                Err(_) => {
                    println!("Secret (string): [Invalid UTF-8]");
                }
            }
        }
    }
}