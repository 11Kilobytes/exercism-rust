use rand::prelude::*;

// Returns a ^ b mod n
fn exp_mod(a: u64, b: u64, n: u64) -> u128 {
    let a = a as u128;
    let b = b as u128;
    let n = n as u128;
    let mut acc = 1u128;
    let mut apow = a % n;
    let mut x = b as u128;
    while x != 0 {
        if (x & 1) == 1u128 {
            acc = (acc * apow) % n;
        }
        x >>= 1;
        apow = (apow * apow) % n;
    }
    return acc;
}

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    exp_mod(g, a, p) as u64
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    exp_mod(b_pub, a, p) as u64
}
