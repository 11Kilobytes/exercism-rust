use core::iter::Iterator;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..(digits.len() + 1).saturating_sub(len))
        .map(|i| digits.chars().skip(i).take(len).collect())
        .collect()
}
