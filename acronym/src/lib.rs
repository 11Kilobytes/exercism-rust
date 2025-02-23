use std::str;

pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    for word in phrase.split(|c: char| c.is_whitespace() || c == '-') {
        let mut chars = word.chars().filter(|&c| c.is_alphabetic());
        if let Some(c) = chars.next() {
            result.push_str(&c.to_uppercase().to_string());
            let _ = chars.by_ref().take_while(|&c| c.is_uppercase()).last();
        }
        while let Some(c) = chars.find(|&c| c.is_uppercase()) {
            result.push(c);
            let _ = chars.by_ref().take_while(|&c| c.is_uppercase()).last();
        }
    }
    result
}
