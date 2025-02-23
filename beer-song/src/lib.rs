pub fn verse(n: u32) -> String {
    fn n_bottles(n: u32, capitalize: bool) -> String {
        if n == 0 {
            if capitalize {
                "No more bottles".to_string()
            } else {
                "no more bottles".to_string()
            }
        } else if n == 1 {
            "1 bottle".to_string()
        } else {
            format!("{n} bottles")
        }
    }
    let stanza_1 = format!(
        "{} of beer on the wall, {} of beer.",
        n_bottles(n, true),
        n_bottles(n, false)
    );
    let stanza_2 = if n == 0 {
        ("Go to the store and buy some more, 99 bottles of beer on the wall.").to_string()
    } else {
        format!(
            "Take {} down and pass it around, {} of beer on the wall.",
            if n == 1 { "it" } else { "one" },
            n_bottles(n - 1, false)
        )
    };
    stanza_1 + "\n" + &stanza_2
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .reduce(|acc, v| acc + "\n\n" + &v)
        .into_iter()
        .collect()
}
