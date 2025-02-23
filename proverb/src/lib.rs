use core::iter::Iterator;

pub fn build_proverb(list: &[&str]) -> String {
    let mut sentences = list
        .iter()
        .zip(list.iter().skip(1))
        .map(|(w1, w2)| format!("For want of a {w1} the {w2} was lost."))
        .collect::<Vec<String>>();
    if !list.is_empty() {
        sentences.push(format!("And all for the want of a {}.", list[0]))
    }
    sentences.join("\n")
}
