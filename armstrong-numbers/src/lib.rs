pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits(num);
    let N: u32 = digits.len().try_into().unwrap();
    (num as u64) == digits.into_iter().map(|d| (d as u64).pow(N)).sum()
}

fn digits(mut num: u32) -> Vec<u32> {
    let mut result = Vec::new();
    while num != 0 {
        result.push(num % 10);
        num /= 10
    }
    result
}