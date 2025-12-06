use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

pub fn extended_gcd(mut a: BigInt, mut b: BigInt) -> (BigInt, BigInt, BigInt) {
    let mut x0 = BigInt::one();
    let mut y0 = BigInt::zero();
    let mut x1 = BigInt::zero();
    let mut y1 = BigInt::one();

    while !b.is_zero() {
        let q = &a / &b;
        let r = a.mod_floor(&b);
        a = b;
        b = r;

        let nx = &x0 - &q * &x1;
        let ny = &y0 - &q * &y1;

        //the logic is 
        // let x = y1.clone();
        // let y = x1 - (a / b) * y1;

        x0 = x1;
        y0 = y1;
        x1 = nx;
        y1 = ny;
    }

    (a, x0, y0)
}
