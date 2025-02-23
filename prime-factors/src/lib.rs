use core::iter::Iterator;

fn isqrt(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        let mut x0 = n / 2;
        let mut x1 = (x0 + n / x0) / 2;
        while x1 < x0 {
            x0 = x1;
            x1 = (x0 + n / x0) / 2;
        }
        x0
    }
}

fn is_prime(n: u64) -> bool {
    n != 1 && (n == 2 || n == 3 || (1..=isqrt(n)).filter(|&d| n % d == 0).count() == 1)
}

fn factors_of_divisors(n: u64, prime_divisors: &Vec<u64>) -> Vec<u64> {
    assert!(n != 0, "Can only factorize positive numbers");
    println!("n = {n}, pd = {prime_divisors:?}");
    let mut result: Vec<u64> = Vec::new();
    let mut n = n;
    for &p in prime_divisors.iter() {
        while n % p == 0 {
            result.push(p);
            n /= p;
        }
    }
    if n != 1 {
        result.push(n);
    }
    result
}

pub fn factors(n: u64) -> Vec<u64> {
    assert!(n != 0, "Can only factorize positive numbers");
    if n == 1 {
        vec![]
    } else if n == 2 {
        vec![2]
    } else if n == 3 {
        vec![3]
    } else {
        let prime_divisors = (1..=isqrt(n))
            .filter(|&d| is_prime(d) && n % d == 0)
            .collect();
        factors_of_divisors(n, &prime_divisors)
    }
}
