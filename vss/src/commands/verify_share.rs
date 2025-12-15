use num_bigint::BigUint;
use num_traits::One;
use crate::crypto::{params, feldman};

pub fn execute(share: String, commitments: String, verbose: bool) {
    let (p, q, g) = params::get_fixed_params();

    let share_list: Vec<(BigUint, BigUint)> = share
        .split(';')
        .filter_map(|s| {
            let parts: Vec<&str> = s.trim().split(',').collect();
            if parts.len() != 2 {
                eprintln!("Error: Each share must be in format 'x,y'. Got '{}'", s);
                return None;
            }

            let x = match BigUint::parse_bytes(parts[0].trim().as_bytes(), 10) {
                Some(val) => val,
                None => {
                    eprintln!("Error: Failed to parse x coordinate '{}'", parts[0]);
                    return None;
                }
            };

            let y = match BigUint::parse_bytes(parts[1].trim().as_bytes(), 10) {
                Some(val) => val,
                None => {
                    eprintln!("Error: Failed to parse y coordinate '{}'", parts[1]);
                    return None;
                }
            };

            Some((x, y))
        })
        .collect();

    if share_list.is_empty() {
        eprintln!("Error: No valid shares provided. Exiting.");
        return;
    }

    let commitment_list: Vec<BigUint> = commitments
        .split(',')
        .filter_map(|s| {
            match BigUint::parse_bytes(s.trim().as_bytes(), 10) {
                Some(val) => Some(val),
                None => {
                    eprintln!("Error: Failed to parse commitment '{}'", s.trim());
                    None
                }
            }
        })
        .collect();

    if commitment_list.is_empty() {
        eprintln!("Error: No valid commitments provided. Exiting.");
        return;
    }

    println!("Shares to verify: {}", share_list.len());
    println!("Commitments: {}", commitment_list.len());
    println!("Parameters:");
    println!("  p = {}", p);
    println!("  q = {}", q);
    println!("  g = {}", g);

    let mut all_valid = true;
    for (idx, (x, y)) in share_list.iter().enumerate() {
        println!("Share {}", idx + 1);
        println!("x = {}, y = {}", x, y);

        if verbose {
            verify_share_verbose(x, y, &commitment_list, &g, &p, &q);
        }

        let is_valid = feldman::verify_share(x, y, &commitment_list, &g, &p, &q);

        if is_valid {
            println!("VALID - This share is correct and can be used for reconstruction");
        } else {
            println!("INVALID - This share has been tampered with or is incorrect");
            all_valid = false;
        }
    }

    if all_valid {
        println!("║ All shares are valid and can be used for secret reconstruction.");
    } else {
        println!("Some shares failed verification!");
    }
}

fn verify_share_verbose(
    x: &BigUint,
    y: &BigUint,
    commitments: &[BigUint],
    g: &BigUint,
    p: &BigUint,
    _q: &BigUint,
) {
    let left_side = g.modpow(y, p);
    println!("│  Left side:  g^y mod p");
    println!("{}^{} mod {}", g, y, p);
    println!("= {}", left_side);

    println!("│  Right side: ∏ C[j]^(x^j) mod p");

    let mut right_side = BigUint::one();
    let mut x_power = BigUint::one();

    for (j, commitment) in commitments.iter().enumerate() {
        println!("│    Step {}: x^{} = {}", j, j, x_power);
        let term = commitment.modpow(&x_power, p);
        println!("│            C[{}]^(x^{}) mod p = {}", j, j, term);
        right_side = (right_side * term) % p;
        println!("│            Running product = {}", right_side);
        x_power = &x_power * x;
    }

    println!("Comparison:");
    println!("   Left  = {}", left_side);
    println!("   Right = {}", right_side);
    println!("   Match = {}", left_side == right_side);
}
