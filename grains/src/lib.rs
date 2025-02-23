pub fn square(s: u32) -> u64 {
    assert!(1 <= s && s <= 64, "Expected that 0 ≤ s ≤ 64 where s = {s}");
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    u64::MAX
}
