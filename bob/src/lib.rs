use core::iter::Iterator;

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.chars().all(|c| c.is_whitespace()) {
        "Fine. Be that way!"
    } else {
        let is_question = message.chars().last().is_some_and(|c| c == '?');
        let is_yelling = message
            .chars()
            .all(|c| !c.is_alphabetic() || c.is_uppercase())
            && message.chars().any(|c| c.is_alphabetic());
        match (is_question, is_yelling) {
            (true, true) => "Calm down, I know what I'm doing!",
            (true, false) => "Sure.",
            (false, true) => "Whoa, chill out!",
            (false, false) => "Whatever.",
        }
    }
}
