pub fn nth(n: u32) -> u32 {
    (2u32..)
        .filter(|&it| (1..=(it as f64).sqrt() as u32).filter(|x| it % x == 0).count() == 1)
        .take((n as usize) + 1)
        .last()
        .expect("Impossible case?")
}
