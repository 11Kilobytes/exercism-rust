use core::{iter::Iterator, str};

const NUM_STUDENTS: usize = 12;
// const STUDENTS: [&str; NUM_STUDENTS] = [
//     "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
//     "Kincaid", "Larry",
// ];

const PLANTS: [&[u8]; 4] = [b"grass", b"clover", b"radishes", b"violets"];

// Looks up an item of STUDENTS which equals the given one. Note that
// i'th entry of STUDENTS beins with the ith letter of the alphabet.
fn find_student(student: &str) -> Option<usize> {
    let c = student.chars().next()?;
    if c.is_ascii_uppercase() {
        let id = (c as usize) - (b'A' as usize);
        if id < NUM_STUDENTS {
            Some(id)
        } else {
            None
        }
    } else {
        None
    }
}

fn find_plant(code: u8) -> Option<&'static [u8]> {
    PLANTS
        .iter()
        .find(|&&s| s[0] == code.to_ascii_lowercase())
        .map(|x| *x)
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    assert!(
        diagram.is_ascii(),
        "Valid diagrams must consist of ASCII characters only"
    );
    let bytes = diagram.as_bytes();
    let student_id = find_student(student).expect("Expected valid student,  {student}");
    bytes
        .split(|&b| b == b'\n')
        .flat_map(|row| {
            let idx = student_id * 2;
            row[idx..=idx + 1].iter().map(|&code| {
                str::from_utf8(find_plant(code).expect(&format!("Plant code {code} should be either G, C, R, or V.")))
                    .unwrap()
            })
        })
        .collect()
}
