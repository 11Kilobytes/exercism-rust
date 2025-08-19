use std::{
    collections::HashMap,
    fmt::{self, Write},
    ops::{Index, IndexMut},
};

/// Solutions to Alphametics puzzles must give a one-to-one mapping
/// from variable names to their values.  Environments represent this
/// mapping as an array var_to_digit.  We assume that (digit_mask[d] ==
/// true) ⬄ a unique place in var_to_digit equals Some(d).
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
            self.var_to_digit[usize::from(var)] = Some(digit);
            self.digit_mask[usize::from(digit)] = true;
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

/// A Grid<T> represents a square 2D-array of elements of type `T`.
#[derive(PartialEq, Eq)]
struct Grid<T> {
    /// We assume that `_data.len() == rows * cols`.  We use
    /// _column-major_ representation since we will often be doing
    /// columnwise sums and queries.
    _data: Box<[T]>,
    rows: usize,
    cols: usize,
}
impl<T> Grid<T> {
    fn get(&self, index: (usize, usize)) -> Option<&T> {
        let (row, col) = index;
        self._data.get(col * self.rows + row)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        if row >= self.rows {
            panic!(
                "Expected that the row {row} is less than the number of rows {}",
                self.rows
            );
        }
        if col >= self.cols {
            panic!(
                "Expected that the column {col} is less than the number of columns {}",
                self.cols
            )
        }
        &self._data[col * self.rows + row]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        if row >= self.rows {
            panic!(
                "Expected that the row {row} is less than the number of rows {}",
                self.rows
            );
        }
        if col >= self.cols {
            panic!(
                "Expected that the column {col} is less than the number of columns {}",
                self.cols
            )
        }
        &mut self._data[col * self.rows + row]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    /// Extracts a reference to the `col`-th column.
    fn index(&self, col: usize) -> &Self::Output {
        if col >= self.cols {
            panic!(
                "Expected that the row {col} is less than the number of rows {}",
                self.rows
            );
        }
        &self._data[(col * self.rows)..(col * (self.rows + 1))]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    /// Extracts an exclusive reference to the `col`-th column.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.cols {
            panic!(
                "Expected that the row {index} is less than the number of rows {}",
                self.rows
            );
        }
        &mut self._data[(index * self.rows)..(index * (self.rows + 1))]
    }
}

enum GridParseErr {
    JaggedInput,
    Empty,
    NoArgs,
}

impl<T: Clone> TryFrom<&[Vec<T>]> for Grid<T> {
    type Error = GridParseErr;

    fn try_from(nested_rows: &[Vec<T>]) -> Result<Self, Self::Error> {
        let rows = nested_rows.len();
        let cols = nested_rows.get(0).map_or(0, |row| row.len());
        if rows == cols && nested_rows.iter().all(|row| row.len() == cols) {
            let mut _data = Box::<[T]>::new_uninit_slice(rows * cols);
            for (i, row) in nested_rows.iter().enumerate() {
                for (j, elem) in row.iter().enumerate() {
                    unsafe { _data[j * rows + i].as_mut_ptr().write(elem.clone()) }
                }
            }
            Ok(Self {
                _data: unsafe { _data.assume_init() },
                rows,
                cols,
            })
        } else {
            Err(GridParseErr::JaggedInput)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum CellTag {
    Variable,
    Digit,
}

/// A Puzzle struct represets a partially completed solution to the
/// equation `sum(args + carry) == ret`, where addition is performed
/// using the column addition method.
#[derive(PartialEq, Eq)]
struct Puzzle {
    /// `args` encoded as a `Grid` of cells. After the puzzle is
    /// completed, each argument should be a row of the `Grid`,
    /// encoded as a _little-endian_ base-10 numeral.
    args: Grid<u8>,
    /// In order to distinguish between variable names and digits, we
    /// use a 2D array of [tags](CellTag) to distinguish which is which.  Each
    /// element of `tags` corresponds to an element of `data`, so that:
    ///
    /// 1. Whenever `args_tags[i] == Cell::Variable` we have `0 <= data[i]
    /// < 26` since there are `26` possible variable names for unknown
    /// digits, one for each letter of the English alphabet.
    /// 2. Whenever `args_[i] == Cell::Digit` we have `0 <= data[i] <
    /// 10`, since the only value for a square has to be a digit in
    /// `0..10`.
    args_tags: Grid<CellTag>,

    /// The desired return value for the sum is encoded in a similar way as `args`, but
    /// we use ordinary boxed slices instead of Grids to encode a 1D vector.
    /// We assume that `ret.len() == args.cols`.
    ret: Box<[u8]>,
    ret_tags: Box<[CellTag]>,

    /// The `carry` row has to be this big to make sure that we never overflow.
    carry: Box<[u16]>,

    env: Environment,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Remove repetition using helper function:
        // `fmt_cell(f: &mut Formatter<'_>, cell: &u8, tag: &CellTag, sep: Option<char>) -> fmt::Result`.
        for i in 0..self.args.rows {
            for j in (0..self.args.cols).rev() {
                let offset = match self.args_tags[(i, j)] {
                    CellTag::Variable => b'A',
                    CellTag::Digit => b'0',
                };
                if j >= 1 {
                    write!(f, " ")?;
                }
                write!(f, "{}", char::from(offset + self.args[(i, j)]))?;
            }
            write!(f, "\n")?;
        }
        writeln!(f, "{:->width$}", "", width = self.args.cols)?;
        let mut ret_iter = self.ret.iter().zip(self.ret_tags.iter()).rev();
        if let Some((cell, tag)) = ret_iter.next() {
            let offset = match tag {
                CellTag::Variable => b'A',
                CellTag::Digit => b'0',
            };
            f.write_char(char::from(offset + cell))?;
        }
        for (cell, tag) in ret_iter {
            let offset = match tag {
                CellTag::Variable => b'A',
                CellTag::Digit => b'0',
            };
            f.write_char(' ')?;
            f.write_char(char::from(offset + cell))?;
        }
        Ok(())
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
