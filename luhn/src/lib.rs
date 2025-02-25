/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.trim().len() > 1 && (code.chars().all(|ch| ch.is_digit(10) || ch.is_whitespace())) && {
        let total: u32 = code
            .chars()
            .map(|ch| ch.to_digit(10))
            .flatten()
            .rev()
            .enumerate()
            .map(|(i, d)| {
                let doubled = d * ((1 + i) as u32);
                doubled % 10 + doubled / 10
            })
            .sum();
        total % 10 == 0
    }
}
