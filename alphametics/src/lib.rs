use std::collections::HashMap;

// We assume that the vectors that make up a valid Puzzle have the
// same length.

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Variable(u8),
    Digit(u8),
}

/// A valid Puzzle representing A + B == C, where a is the
/// _little-endian_ encoding of A and likewise with b, c.
#[derive(Clone)]
struct Puzzle {
    a: Vec<Cell>,
    b: Vec<Cell>,
    c: Vec<Cell>,
    carry: Vec<bool>,
    var_to_digit: [Option<u8>; 26],
    /// We assume that 0 <= col < len where len is the common length
    /// of the vector members of a valid Puzzle.
    col: usize,
}
enum Row {
    A,
    B,
}
struct Move {
    row: Row,
    var: u8,
    digit: u8,
}

impl Puzzle {
    fn subst(&self, m: Move) -> Option<Self> {
        debug_assert!((0..26).contains(&m.var));
        let mut result = self.clone();
        result.var_to_digit[m.var as usize] = Some(m.digit);
        for i in result.col..result.c.len() {
            if let Cell::Variable(v) = result.a[i] {
                if let Some(d) = result.var_to_digit[v as usize] {
                    result.a[i] = Cell::Digit(d);
                }
            }
            if let Cell::Variable(v) = result.b[i] {
                if let Some(d) = result.var_to_digit[v as usize] {
                    result.b[i] = Cell::Digit(d);
                }
            }
            if let Cell::Variable(v) = result.c[i] {
                if let Some(d) = result.var_to_digit[v as usize] {
                    result.c[i] = Cell::Digit(d);
                }
            }
            if let (Cell::Digit(a_i), Cell::Digit(b_i)) = (result.a[i], result.b[i]) {
                let sum = a_i + b_i + (result.carry[i] as u8);
                let (carry, d) = (sum / 10, sum % 10);
                match result.c[i] {
                    Cell::Variable(v) => {
                        result.var_to_digit[v as usize] = Some(d);
                        result.c[i] = Cell::Digit(d);
                    }
                    Cell::Digit(c_i) => {
                        if c_i != d {
                            return None;
                        }
                    }
                }
                if carry != 0 {
                    *result.carry.get_mut(i + 1)? = true;
                }
            }
        }
        result.col += result.a[result.col..]
            .iter()
            .zip(result.b[result.col..].iter())
            .position(|p| match p {
                (Cell::Digit(_), Cell::Digit(_)) => true,
                _ => false,
            })
            .map(|i| i + 1)
            .unwrap_or(0);
        return Some(result);
    }
    fn succ(&self) -> impl Iterator<Item = Self> {
        todo!()
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let words: [Vec<u8>; 3] = input
        .split(|c: char| !c.is_ascii_uppercase())
        .map(|word| word.bytes().map(|b| b - b'A').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
        .try_into()
        .expect("Invalid Puzzle");
    let (wa, wb, wc) = words.into();
    todo!()
}
