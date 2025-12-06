use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;
use crate::crypto::{params, polynomial, feldman};

pub fn execute(secret: String, n: usize, k: usize) {
    if k > n || k == 0 || n == 0 {
        eprintln!("Error: Invalid n or k values");
        eprintln!("  - k must be <= n");
        eprintln!("  - Both k and n must be > 0");
        std::process::exit(1);
    }

    let mut rng = thread_rng();

    let secret_num = BigUint::from_bytes_be(secret.as_bytes());

    let (p, q, g) = params::get_fixed_params();

    println!("Split Mode");
    println!("Secret: {}", secret);
    println!("n={}, k={}", n, k);
    println!();

    let mut coefficients = vec![secret_num.clone() % &q];
    for _ in 1..k {
        
        coefficients.push(rng.gen_biguint(q.bits() as u64) % &q);
    }

    let commitments = feldman::generate_commitments(&coefficients, &g, &p);

    let mut shares = Vec::new();
    for i in 1..=n {
        let x = BigUint::from(i as u64);
        let y = polynomial::evaluate(&coefficients, &x, &q);
        shares.push((x, y));
    }

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