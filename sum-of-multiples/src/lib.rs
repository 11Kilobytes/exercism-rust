use core::iter::Iterator;
use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let base_values: HashSet<u32> = factors
        .iter()
        .filter(|&x| *x != 0)
        .flat_map(|&x| (0..limit).step_by(x as usize))
        .collect();
    base_values.iter().sum()
}
