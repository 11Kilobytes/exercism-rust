use core::iter::Iterator;

const OPENS: [char; 3] = ['(', '[', '{'];
const CLOSES: [char; 3] = [')', ']', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        if OPENS.iter().any(|&ob| c == ob) {
            stack.push(c);
        } else if let Some((&o, _)) = OPENS.iter().zip(CLOSES.iter()).find(|&(_, &cb)| c == cb) {
            if !stack.pop().is_some_and(|b| b == o) {
                return false;
            }
        }
    }
    return stack.is_empty();
}
