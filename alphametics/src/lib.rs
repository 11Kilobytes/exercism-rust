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
/// equation `sum(nums + carry) == ret`, where addition is performed
/// using the column addition method.
#[derive(PartialEq, Eq)]
struct Puzzle {
    /// The arguments `nums` and `ret` are encoded as a `Grid` of
    /// cells. After the puzzle is completed, each number in the
    /// arguments should correspond to a row of the `Grid`, encoded as
    /// a _little-endian_ base-10 numeral.  The last row of `args` encodes
    /// `ret` and each of the other rows encodes a number in `nums`.
    args: Grid<u8>,
    /// In order to distinguish between variable names and digits, we
    /// use a 2D array of boolean [tags](CellTag) to distinguish which
    /// is which.  Each place in `tags` corresponds to a place in
    /// `data` hence `args_tags` and `args` have the same .row and
    /// .col fields and:
    ///
    /// 1. Whenever `self.args_tags[(i, j)] == CellTag::Variable` we
    /// have `0 <= self.args[(i, j)] < 26` since there are `26`
    /// possible variable names for unknown digits, one for each
    /// letter of the English alphabet.
    ///
    /// 2. Whenever `self.args_tags[(i, j)] == CellTag::Digit` we have
    /// `0 <= self.args[(i, j)] < 10`, since the only value for a
    /// square has to be a digit in `0..10`.
    args_tags: Grid<CellTag>,

    /// The cells of the `carry` row have to be this big to make sure
    /// that we never overflow. We assume that `self.carry.len() == self.args.rows`
    carry: Box<[u16]>,

    env: Environment,

    /// We assume that the puzzle entries before `col` have been
    /// filled in to form a partially valid solution, that is:

    /// 1. For every `i, j: usize`: `i < self.rows ∧ j < self.cols ⇒
    /// self.args_tags[(i, j)] == CellTag::Digit`.

    /// 2. For every `j: usize`: `j < self.cols ⇒ self.args[j][..rows -
    /// 2].sum() + carry[j] == self.args[(rows - 1, j)] + 10 *
    /// self.carry.get(j + 1).unwrap_or_else(0)`.
    col: usize,
    row: usize,

    /// We keep a running tally of the current column as we try to
    /// fill it in.  That is: if `self.col < self.cols` so that the
    /// puzzle is incompete, then `self.sum ==
    /// self.args[self.col][..self.row].sum()`.  This formulation of the
    /// invariant implies that we should never have `self.row >
    /// self.rows` for an incomplete puzzle since this might double
    /// count the return value.
    sum: u16,
}

fn fmt_cell(f: &mut fmt::Formatter<'_>, cell: u8, tag: CellTag) -> fmt::Result {
    let offset = match tag {
        CellTag::Variable => b'A',
        CellTag::Digit => b'0',
    };
    f.write_char(char::from(offset + cell))?;
    Ok(())
}

impl fmt::Display for Puzzle {
    // TODO: format the carry row as well.
    // Instead of `fmt_cell` I might instead go for:
    // `fn write_row(f: &mut fmt:Formatter<'_>,
    // rows: impl IntoIterator<Item = u8>,
    // tags: impl IntoIterator<Item = CellTag>,
    // widths: impl IntoIterator<Item = usize>) -> fmt::Result`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.args.rows {
            for j in (0..self.args.cols).rev() {
                if i > 0 {
                    f.write_char(' ')?;
                };
                fmt_cell(f, self.args[(i, j)], self.args_tags[(i, j)])?;
            }
            write!(f, "\n")?;
            if i + 2 == self.args.rows && self.args.cols != 0 {
                write!(f, "{:-<width$}", "", width = 2 * self.args.cols - 1)?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Move {
    NextCol,
    Bind { var: u8, digit: u8 },
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::NextCol => write!(f, "Next column"),
            Move::Bind { var, digit } => {
                fmt_cell(f, *var, CellTag::Variable)?;
                write!(f, " => ")?;
                fmt_cell(f, *digit, CellTag::Digit)?;
                Ok(())
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
    NonZeroCarry,
}
impl From<BindError> for MoveErr {
    fn from(value: BindError) -> Self {
        MoveErr::BindError(value)
    }
}

impl Puzzle {
    fn try_move(&mut self, m: Move) -> Result<(), MoveErr> {
        match m {
            Move::NextCol => {
                todo!()
            }
            Move::Bind { var, digit } => {
                self.env.try_subst(var, digit)?;
                loop {
                    if let Some(CellTag::Variable) = self.args_tags.get((self.row, self.col)) {
                        if let Some(d) = self.env.get(self.args[(self.row, self.col)]) {
                            self.args_tags[(self.row, self.col)] = CellTag::Digit;
                            self.args[(self.row, self.col)] = d;
                            if self.row + 1 < self.args.rows {
                                self.sum += u16::from(d);
                                self.row += 1;
                                continue;
                            } else {
                                break Ok(());
                            }
                        }
                    }
                    break Ok(());
                }
            }
        }
    }
    fn undo_move(&mut self, m: Move) {
        match m {
            Move::NextCol => todo!(),
            Move::Bind { var, digit } => todo!(),
        }
    }
    fn frontier(&self) -> impl Iterator<Item = Move> {}
    fn solve(&mut self) -> Option<HashMap<char, u8>> {}
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let words: Vec<Vec<u8>> = input
        .split(|c: char| !c.is_ascii_uppercase())
        .map(|word| word.bytes().map(|b| b - b'A').collect::<Vec<u8>>())
        .filter(|word| word.len() > 0)
        .collect();
    Puzzle::parse(words).ok()?.solve()
}
