pub fn egg_count(display_value: u32) -> usize {
    let mut acc = 0usize;
    let mut rest = display_value;
    while rest != 0 {
        acc += (rest & 1) as usize;
        rest >>= 1;
    }
    acc
}
