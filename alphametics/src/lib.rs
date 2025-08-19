use std::{
    collections::HashMap,
    fmt::{self, Write},
    ops::{Index, IndexMut},
};

/// Solutions to Alphametics puzzles must give a one-to-one mapping
/// from variable names to their values.  Environments represent this
/// mapping as an array var_to_digit.  We assume that (digit_mask[d] ==
/// true) ⬄ a unique element of var_to_digit equals Some(d).
#[derive(Debug, Eq, PartialEq)]
struct Environment {
    var_to_digit: [Option<u8>; 26],
    digit_mask: [bool; 10],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BindError {
    VariableBound,
    DigitBound,
    InvalidVariable,
    InvalidDigit,
}

impl Environment {
    pub fn try_subst(&mut self, var: u8, digit: u8) -> Result<(), BindError> {
        if !(0..26).contains(&var) {
            Err(BindError::InvalidVariable)
        } else if self.var_to_digit[var as usize].is_some() {
            Err(BindError::VariableBound)
        } else if !(0..10).contains(&digit) {
            Err(BindError::InvalidDigit)
        } else if self.digit_mask[digit as usize] {
            Err(BindError::DigitBound)
        } else {
            self.var_to_digit[var as usize] = Some(digit);
            self.digit_mask[digit as usize] = true;
            Ok(())
        }
    }
    pub fn get(&self, var: u8) -> Option<u8> {
        *self.var_to_digit.get(var as usize)?
    }
}

impl<const N: usize> TryFrom<[(u8, u8); N]> for Environment {
    type Error = BindError;
    fn try_from(table: [(u8, u8); N]) -> Result<Self, Self::Error> {
        let mut acc = Environment {
            var_to_digit: [None; 26],
            digit_mask: [false; 10],
        };
        for (var, digit) in table {
            acc.try_subst(var, digit)?
        }
        Ok(acc)
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = self
            .var_to_digit
            .iter()
            .enumerate()
            .filter_map(|(var, &digit)| Some((var, digit?)));
        if let Some((var, d)) = table.next() {
            write!(f, "{} => {}", char::from(b'A' + (var as u8)), d)?
        }
        for (var, d) in table {
            write!(f, ", {} => {}", char::from(b'A' + (var as u8)), d)?
        }
        Ok(())
    }
}

impl From<&Environment> for HashMap<char, u8> {
    fn from(e: &Environment) -> Self {
        e.var_to_digit
            .iter()
            .enumerate()
            .flat_map(|(v, &digit)| Some((char::from(b'A' + (v as u8)), digit?)))
            .collect()
    }
}

/// A Puzzle struct represets a partially completed solution to the
/// equation sum(args + carry) == target, where addition is performed
/// using the column addition method.
#[derive(Clone, PartialEq, Eq, Debug)]
struct Puzzle {
    /// `args` represents a 2D array of [squares](Sell).
    args: Box<[Box<[Square]>]>,
    /// `target` is represented as a _little-endian_ base 10 numeral
    /// of [squares](Square).
    target: Box<[Square]>,
    /// `carry` the carry for each column as a _little-endian_ base 10
    /// numeral.
    carry: Box<[u16]>,
    env: Environment,
    /// We assume that `0 <= col <= target.len()` and `∀ i: (0 <= i <
    /// col, args.all(|arg| arg[i].is_digit())`.  We include
    /// target.len() as a valid value of col to deal with the case
    /// where the puzzle is empty.
    col: usize,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cols = self.args.get(0).map_or(0, |row| row.len());
        for row in self.args.iter() {
            for sq in row.iter().rev() {
                write!(f, "{sq}")?
            }
            f.write_char('\n')?
        }

        writeln!(f, "{:->width$}", "", width = cols)?;

        for sq in self.target.iter().rev() {
            write!(f, "{sq}")?
        }
        f.write_char('\n')?;

        writeln!(f, "{:->width$}", "", width = cols)?;
        writeln!(f, "{}", self.env)?;
        write!(f, "col = {}", self.col)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Move {
    NextCol,
    Bind { var: u8, digit: u8 },
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::NextCol => write!(f, "Next column"),
            Move::Bind { var, digit } => {
                write!(f, "{} => {}", Square::Variable(*var), Square::Digit(*digit))
            }
        }
    }
}

enum PuzzleParseErr {
    MissingRows,
    MightOverflow,
}

enum MoveErr {
    BindError(BindError),
    TrailingZero,
}

impl From<BindError> for MoveErr {
    fn from(value: BindError) -> Self {
        MoveErr::BindError(value)
    }
}

impl Puzzle {
    pub fn new(words: Vec<Vec<u8>>) -> Result<Self, PuzzleParseErr> {
        let rows = words.len();
        if rows < 3 {
            Err(PuzzleParseErr::MissingRows)
        } else if rows > usize::from(u16::MAX / 10) {
            Err(PuzzleParseErr::MightOverflow)
        } else {
            let cols = words.iter().map(|word| word.len()).max().unwrap_or(0) + 1;

            let words: Box<[Box<[Square]>]> = words
                .iter()
                .map(|arg| {
                    let mut arg_buffer: Vec<Square> = Vec::with_capacity(cols);
                    arg_buffer.extend(arg.iter().map(|&letter| Square::Variable(letter)).rev());
                    arg_buffer.resize(cols, Square::Digit(0));
                    arg_buffer.into_boxed_slice()
                })
                .collect();

            if let Some((target, args)) = words.split_last() {
                Ok(Self {
                    args: args.into(),
                    target: target.clone(),
                    carry: vec![0; cols].into_boxed_slice(),
                    env: Environment {
                        var_to_digit: [None; 26],
                        digit_mask: [false; 10],
                    },
                    col: 0,
                })
            } else {
                Err(PuzzleParseErr::MissingRows)
            }
        }
    }
    fn subst_row<'a>(cells: &Box<[Square]>, e: &Environment, col: usize) -> Option<Box<[Square]>> {
        let mut row = cells.clone();
        let cols = row.len();
        for (i, sq) in row[col..].iter_mut().enumerate() {
            if let Square::Variable(var) = sq {
                if let Some(digit) = e.get(*var) {
                    if i + col + 2 == cols && digit == 0 {
                        return None;
                    } else {
                        *sq = Square::Digit(digit);
                    }
                }
            }
        }
        Some(row)
    }
    fn try_subst(&self, var: u8, digit: u8) -> Result<Self, MoveErr> {
        let env = self.env.try_subst(var, digit)?;

        let mut args = self.args.clone();
        for arg in args.iter_mut() {
            match Self::subst_row(arg, &env, self.col) {
                Some(arg_) => *arg = arg_,
                None => return Err(MoveErr::TrailingZero),
            }
        }
        match Self::subst_row(&self.target, &env, self.col) {
            Some(target) => Ok(Self {
                col: self.col,
                carry: self.carry.clone(),
                env,
                args,
                target,
            }),
            None => Err(MoveErr::TrailingZero),
        }
    }
    fn decide_col(&self) -> Result<u8, (u16, u8)> {
        debug_assert!((0..self.target.len()).contains(&self.col));
        self.args
            .iter()
            .find_map(|arg| arg[self.col].get_var())
            .ok_or_else(|| {
                let sum: u16 = self.carry[self.col]
                    + self
                        .args
                        .iter()
                        .map(|arg| match arg[self.col] {
                            Square::Variable(_) => 0u16,
                            Square::Digit(d) => u16::from(d),
                        })
                        .sum::<u16>();
                (sum / 10, (sum % 10) as u8)
            })
    }

    // TODO: Need to borrow &mut self to avoid FP allocations, also
    // allows try subst to modify both args + target.  Not sure if the
    // allocations make the program all that slow for small inputs,
    // because I still after all would have to undo all the moves
    // comprising the path to every failed edge.  I guess it depends
    // on whether the allocator can "see" that it needs to reserve
    // space for the entire path, because the memory used to traverse
    // from the start to any leaf is independent of the leaf.  Would
    // also have to refactor so that every mutation of self comes from
    // one Move that's recorded on the call stack, so that we can undo
    // them later.
    fn play(&self, m: Move) -> Option<Self> {
        match m {
            Move::NextCol => {
                if self.target.get(self.col)?.is_digit() {
                    Some(Self {
                        col: self.col + 1,
                        ..self.clone()
                    })
                } else {
                    None
                }
            }
            Move::Bind { var, digit } => self.try_subst(var, digit).ok(),
        }
    }

    pub fn frontier(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        if self.col == self.target.len() {
            Box::new(std::iter::empty())
        } else {
            match self.decide_col() {
                Ok(var) => {
                    Box::new((0..10).flat_map(move |digit| self.play(Move::Bind { var, digit })))
                }
                Err((carry, sum)) => {
                    let mut result = Self {
                        carry: self.carry.clone(),
                        ..self.clone()
                    };
                    if result.col + 1 >= result.carry.len() {
                        return Box::new(std::iter::empty());
                    }
                    result.carry[(result.col + 1) as usize] = carry;
                    match result.target[result.col] {
                        Square::Variable(var) => Box::new(
                            (0..10).flat_map(move |digit| result.play(Move::Bind { var, digit })),
                        ),
                        Square::Digit(d) => {
                            if d == sum {
                                Box::new(result.play(Move::NextCol).into_iter())
                            } else {
                                Box::new(std::iter::empty())
                            }
                        }
                    }
                }
            }
        }
    }

    fn solve(&self) -> Option<HashMap<char, u8>> {
        if self.col + 1 == self.target.len() {
            Some((&self.env).into())
        } else {
            self.frontier().find_map(|p| p.solve())
        }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let words: Vec<Vec<u8>> = input
        .split(|c: char| !c.is_ascii_uppercase())
        .map(|word| word.bytes().map(|b| b - b'A').collect::<Vec<u8>>())
        .filter(|word| word.len() > 0)
        .collect();
    Puzzle::new(words).ok()?.solve()
}
