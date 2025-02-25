use std::collections::HashMap;

// We assume that the vectors that make up a valid Puzzle have the
// same length.

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    /// Varaible(v) :: a variable with the name v.  Where 0 <= v < 26.
    Variable(u8),
    /// Digit(d) :: a fixed digit d.
    Digit(u8),
}

impl Cell {
    fn is_var(&self) -> bool {
        match self {
            Cell::Variable(_) => true,
            _ => false,
        }
    }
    fn is_digit(&self) -> bool {
        match self {
            Cell::Digit(_) => true,
            _ => false,
        }
    }
}
/// A valid Puzzle representing the equation sum(args) == target,
/// Solutions to Alphametics puzzles must give a one-to-one mapping
/// from variable names to their values.  Environments represent this
/// mapping as an array var_to_digit.  We assume that digit_mask[d] ==
/// 1 ⬄ a unique element of var_to_digit equals Some(d).
#[derive(Clone, Debug, Eq, PartialEq)]
struct Environment {
    var_to_digit: [Option<u8>; 26],
    digit_mask: [bool; 10]
}

impl Environment {
    // Advantage of this design: since self is moved, even if the
    // substitution fails caller doesn't have access to the previous
    // env.  Enforces the idea that envs are for Puzzle::play alone...

    // Main problem is that I want the fn's in the Puzzle trait to
    // take self by reference, otherwise the search routine would have to
    // clone them which seems bad.  If Environments are consumed by value, then
    // their container Puzzles have to be consumed by value as well.
    // pub fn add(self, var: u8, digit: u8) -> Option<Self> { }
    // pub fn get(&self, var: u8) -> Option<u8> { }

    // Can enforce error handling with must_use I think.
    // Inspired by HashMap::try_insert
    // pub fn try_bind(&mut self, var: u8, digit: u8) -> Option<&mut u8> {}
    // pub fn try_bind(&mut self, var: u8, digit: u8) -> Result<&mut u8, OccupiedError> {}
    // pub fn get(&self, var: u8) -> Option<u8>
}
/// where each entry of args is a decimal number represented as a
/// _little-endian_ vector of digits, and likewise with target.
#[derive(Clone)]
struct Puzzle {
    args: Vec<Vec<Cell>>,
    target: Vec<Cell>,
    carry: Vec<bool>,
    var_to_digit: [Option<u8>; 26],
    /// We assume that 0 <= col <= len where len is the common length
    /// of the elements of args.
    col: usize,
}

struct Move {
    var: u8,
    digit: u8,
}

impl Puzzle {
    fn subst_row(row: &mut Vec<Cell>, var_to_digit: &[Option<u8>; 26]) {
        for c in row.iter_mut() {
            if let Cell::Variable(v) = *c {
                if let Some(d) = var_to_digit[v as usize] {
                    *c = Cell::Digit(d);
                }
            }
        }
    }
    fn subst(&mut self, m: Move) {
        self.var_to_digit[m.var as usize] = Some(m.digit);
        for row in self.args.iter_mut() {
            Puzzle::subst_row(row, &self.var_to_digit);
        }
        Puzzle::subst_row(&mut self.target, &self.var_to_digit);
    }
    pub fn play(&self, m: Move) -> Option<Self> {
        debug_assert!((0..26).contains(&m.var));
        let mut result = self.clone();
        result.subst(m);
        for col in result.col..result.target.len() {
            if result.args.iter().all(|arg| arg[col].is_digit()) {
                let mut sum: u8 = result
                    .args
                    .iter()
                    .map(|arg| match arg[col] {
                        Cell::Variable(_) => 0,
                        Cell::Digit(d) => d,
                    })
                    .sum();
                sum += result.carry[col] as u8;
                let (carry_next, digit) = (sum / 10, sum % 10);
                match result.target[col] {
                    Cell::Variable(var) => result.subst(Move { var, digit }),
                    Cell::Digit(d) => {
                        if digit != d {
                            return None;
                        }
                    }
                }
                if carry_next != 0 {
                    *result.carry.get_mut(col + 1)? = true;
                }
            }
        }
        result.col += (result.col..result.target.len())
            .take_while(|&col| result.args.iter().all(|arg| arg[col].is_digit()))
            .count();
        return Some(result);
    }
    pub fn succ(&self) -> impl Iterator<Item = Self> {
        todo!()
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let words: Vec<Vec<u8>> = input
        .split(|c: char| !c.is_ascii_uppercase())
        .map(|word| word.bytes().map(|b| b - b'A').collect::<Vec<u8>>())
        .collect();
    todo!()
}
