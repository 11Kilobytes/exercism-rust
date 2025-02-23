use std::cmp::Ordering;

pub fn find<A: Ord, T: AsRef<[A]>>(array: T, key: A) -> Option<usize> {
    let mut low = 0usize;
    let mut hi = array.as_ref().len();
    while low < hi {
        let mid = (low + hi) / 2;
        match key.cmp(&array.as_ref()[mid]) {
            Ordering::Less => {hi = mid},
            Ordering::Equal => {return Some(mid)},
            Ordering::Greater => {low = mid + 1}
        }
    }
    return None
}
