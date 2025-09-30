use std::{
    collections::HashMap,
    fmt::{self, Write},
    ops::{Index, IndexMut},
};

/// Solutions to Alphametics puzzles must give a one-to-one mapping
/// from variable names to their values.  Environments represent such a
/// mapping using their `var_to_digit` field, and the inverse mapping
/// using their `digit_to_var` field.
#[derive(Debug, Eq, PartialEq)]
struct Environment {
    var_to_digit: [Option<u8>; 26],
    digit_to_var: [Option<u8>; 10],
}

#[derive(Debug, Eq, PartialEq)]
enum Entry {
    Old,
    Fresh { var: u8, digit: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BindError {
    VariableBound,
    DigitBound,
    InvalidVariable,
    InvalidDigit,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            var_to_digit: [None; 26],
            digit_to_var: [None; 10],
        }
    }
    pub fn get(&self, var: u8) -> Option<u8> {
        *self.var_to_digit.get(var as usize)?
    }
    pub fn try_bind(&mut self, var: u8, digit: u8) -> Result<Entry, BindError> {
        if !(0..26).contains(&var) {
            Err(BindError::InvalidVariable)
        } else if !(0..10).contains(&digit) {
            Err(BindError::InvalidDigit)
        } else if self.var_to_digit[usize::from(var)] == Some(digit) {
            Ok(Entry::Old)
        } else if self.var_to_digit[usize::from(var)].is_some() {
            Err(BindError::VariableBound)
        } else if self.digit_to_var[usize::from(digit)].is_some() {
            Err(BindError::DigitBound)
        } else {
            self.var_to_digit[usize::from(var)] = Some(digit);
            self.digit_to_var[usize::from(digit)] = Some(var);
            Ok(Entry::Fresh { var, digit })
        }
    }
    // pub fn free_digits(&self) -> impl Iterator<Item = u8> + use<'_> {
    //     (0u8..10).filter(|&d| self.digit_to_var[usize::from(d)].is_none())
    // }
    pub fn unbind(&mut self, entry: Entry) {
        if let Entry::Fresh { var, digit } = entry {
            self.digit_to_var[usize::from(digit)] = None;
            self.var_to_digit[usize::from(var)] = None;
        }
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

impl<T: Clone> Grid<T> {
    fn replicate(elem: T, rows: usize, cols: usize) -> Self {
        let mut _data = vec![elem; rows * cols].into_boxed_slice();
        Self { _data, rows, cols }
    }
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
        &self._data[(col * self.rows)..((col + 1) * self.rows)]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    /// Extracts an exclusive reference to the `col`-th column.
    fn index_mut(&mut self, col: usize) -> &mut Self::Output {
        if col >= self.cols {
            panic!(
                "Expected that the row {col} is less than the number of rows {}",
                self.rows
            );
        }
        &mut self._data[(col * self.rows)..((col + 1) * self.rows)]
    }
}

impl<T: Clone + Default> From<&[Vec<T>]> for Grid<T> {
    fn from(nested_rows: &[Vec<T>]) -> Grid<T> {
        let rows = nested_rows.len();
        let cols = nested_rows.iter().map(|row| row.len()).max().unwrap_or(0);
        let mut _data = vec![Default::default(); rows * cols].into_boxed_slice();
        for (i, row) in nested_rows.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                _data[j * rows + i] = elem.clone();
            }
        }
        Self { _data, rows, cols }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum CellTag {
    Variable,
    Digit,
}

impl Default for CellTag {
    fn default() -> Self {
        CellTag::Digit
    }
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
    /// `data` hence `args_tags` and `args` have the same `.row` and
    /// `.col` fields and:
    ///
    /// 1. Whenever `self.args_tags[i, j] == CellTag::Variable` we
    /// have `0 <= self.args[i, j] < 26` since there are `26`
    /// possible variable names for unknown digits, one for each
    /// letter of the English alphabet.
    ///
    /// 2. Whenever `self.args_tags[i, j] == CellTag::Digit` we have
    /// `0 <= self.args[i, j] < 10`, since the only value for a
    /// square has to be a digit in `0..10`.
    args_tags: Grid<CellTag>,

    /// We need to remember the sum of a column after moving on to the
    /// next one in order to be able to backtrack after a failing
    /// move.  For every `j : usize`, `j < self.col ⇒
    /// self.args[j,..-1].sum() == self.sums[j]`.
    sums: Box<[u16]>,

    env: Environment,

    /// We assume that the puzzle entries before `col` have been
    /// filled in to form a partially valid solution, that is:

    /// For every `i, j: usize`: `(j, i) < (self.col, self.row) ⇒
    /// self.args_tags[i, j] == CellTag::Digit`.
    col: usize,
    row: usize,

    /// We keep a running tally of the current column as we try to
    /// fill it in.  That is: if `self.col < self.cols` so that the
    /// puzzle is incompete, then `self.sum == self.args[0..self.row,
    /// self.col].sum()`.
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
        writeln!(f)?;
        for i in 0..self.args.rows {
            for j in (0..self.args.cols).rev() {
                if j + 1 < self.args.cols {
                    f.write_char(' ')?;
                };
                fmt_cell(f, self.args[(i, j)], self.args_tags[(i, j)])?;
            }
            write!(f, "\n")?;
            if i + 2 == self.args.rows && self.args.cols != 0 {
                writeln!(f, "{:-<width$}", "", width = 2 * self.args.cols - 1)?;
            }
        }
        writeln!(f, "Environment: {}", self.env)?;
        writeln!(f, "Cursor: ({}, {})", self.row, self.col)?;
        writeln!(f, "Sum: {}", self.sum)?;
        write!(f, "Sums: ")?;
        let mut iter = self.sums.iter().rev();
        if let Some(s) = iter.next() {
            write!(f, "{s}")?;
        }
        for s in iter {
            write!(f, " {s}")?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Move {
    Carry { digit: u8, carry: u16 },
    Bind { row: usize, digit: u8 },
    FindVar { row: usize },
}

enum PuzzleParseErr<'a> {
    InvalidWord(&'a str, char),
    InvalidSuffix(&'a str),
    InvalidSep(&'a str),
    MightOverflow,
}

enum MoveErr {
    BindError(BindError),
    NonZeroTerminalCarry,
}
impl From<BindError> for MoveErr {
    fn from(value: BindError) -> Self {
        MoveErr::BindError(value)
    }
}

impl Puzzle {
    fn try_move(&mut self, m: &Move) -> Result<Entry, MoveErr> {
        // TODO: Doesn't deal with empty rows correctly, e.g. I'd claim that " + == A" is a valid puzzle,
        // with the solution A = 0 since the sum of an empty series is zero.
        match *m {
            Move::Carry { digit, carry } => {
                debug_assert!(self.row + 1 == self.args.rows);
                debug_assert!(self.args_tags[(self.row, self.col)] == CellTag::Variable);
                let binding = self.env.try_bind(self.args[(self.row, self.col)], digit)?;
                self.args_tags[(self.row, self.col)] = CellTag::Digit;
                self.args[(self.row, self.col)] = digit;
                self.sums[self.col] = self.sum;
                if carry != 0 && self.col + 1 == self.args.cols {
                    Err(MoveErr::NonZeroTerminalCarry)
                } else {
                    self.col += 1;
                    self.sum = carry;
                    self.row = 0;
                    Ok(binding)
                }
            }
            Move::Bind { row, digit } => {
                debug_assert!(self.row == row);
                debug_assert!(self.args_tags[(self.row, self.col)] == CellTag::Variable);
                let binding = self.env.try_bind(self.args[(self.row, self.col)], digit)?;
                loop {
                    if let Some(CellTag::Variable) = self.args_tags.get((self.row, self.col)) {
                        if let Some(d) = self.env.get(self.args[(self.row, self.col)]) {
                            if self.row + 1 < self.args.rows {
                                self.args_tags[(self.row, self.col)] = CellTag::Digit;
                                self.args[(self.row, self.col)] = d;
                                self.sum += u16::from(d);
                                self.row += 1;
                                continue;
                            }
                        }
                    }
                    break Ok(binding);
                }
            }

            Move::FindVar { row: _ } => {
                debug_assert!(self.col < self.args.cols);
                self.row = self.args_tags[self.col]
                    .iter()
                    .position(|&t| t == CellTag::Variable)
                    .unwrap_or(self.args.rows);
                Ok(Entry::Old)
            }
        }
    }
    fn undo_move(&mut self, m: &Move, e: Entry) {
        eprintln!("Undoing Move {:?}, Entry {:?}: ", m, e);
        match *m {
            Move::Carry { digit, carry: _ } => {
                self.sums[self.col] = 0;
                self.col -= 1;
                self.row = self.args.rows - 1;
                self.sum = self.sums[self.col];
                self.args_tags[(self.row, self.col)] = CellTag::Variable;
                self.args[(self.row, self.col)] =
                    self.env.digit_to_var[usize::from(digit)].expect(&format!(
                        "The digit {digit} should have been bound in the environment {}",
                        self.env
                    ));
            }
            Move::Bind { row, digit: _ } => {
                let row = usize::from(row);
                for (tag, cell) in &mut self.args_tags[self.col][row..self.row]
                    .iter_mut()
                    .zip(&mut self.args[self.col][row..self.row])
                {
                    self.row -= 1;
                    self.sum -= u16::from(*cell);
                    *tag = CellTag::Variable;
                    *cell = self.env.digit_to_var[usize::from(*cell)].expect(&format!(
                        "The digit {cell} should have been bound in the environment {}",
                        self.env
                    ));
                }
            }
            Move::FindVar { row } => self.row = row,
        }
        self.env.unbind(e);
    }
    fn frontier(&self) -> Box<dyn Iterator<Item = Move>> {
        let row = self.row;
        if self.args_tags[(self.row, self.col)] != CellTag::Variable {
            Box::new(std::iter::once(Move::FindVar { row }))
        } else if self.row + 1 < self.args.rows {
            Box::new((0..10u8).map(move |d| Move::Bind { row, digit: d }))
        } else {
            let (digit, carry) = (
                u8::try_from(self.sum % 10)
                    .expect("(% 10) has the range 0..10 each element of which fits in a u8"),
                self.sum / 10,
            );
            Box::new(std::iter::once(Move::Carry { digit, carry }))
        }
    }
    fn solve(&mut self) -> Option<HashMap<char, u8>> {
        eprintln!("Solving: {}", self);
        if self.col >= self.args.cols {
            return Some((&self.env).into());
        }
        for m in self.frontier() {
            eprintln!("Considering Move {:?}", m);
            if let Ok(binding) = self.try_move(&m) {
                eprintln!("Applied Move {:?}", m);
                if let Some(ret) = self.solve() {
                    return Some(ret);
                } else {
                    self.undo_move(&m, binding);
                    eprintln!("Undone {}", self);
                }
            }
        }
        return None;
    }

    fn parse(input: &str) -> Result<Self, PuzzleParseErr<'_>> {
        enum ParseState {
            Word,
            Sep,
            Ret,
            Trailing,
        }
        let mut state = ParseState::Word;
        let mut words: Vec<Vec<u8>> = Vec::new();
        for tok in input.split_ascii_whitespace() {
            match state {
                ParseState::Word => {
                    if let Some(c) = tok.chars().find(|c| !c.is_ascii_uppercase()) {
                        return Err(PuzzleParseErr::InvalidWord(tok, c));
                    } else {
                        state = ParseState::Sep;
                        words.push(tok.bytes().map(|c| c - b'A').rev().collect());
                    }
                }
                ParseState::Sep => {
                    if tok == "+" {
                        state = ParseState::Word;
                    } else if tok == "==" {
                        state = ParseState::Ret;
                    } else {
                        return Err(PuzzleParseErr::InvalidSep(tok));
                    }
                }
                ParseState::Ret => {
                    if let Some(c) = tok.chars().find(|c| !c.is_ascii_uppercase()) {
                        return Err(PuzzleParseErr::InvalidWord(tok, c));
                    } else {
                        state = ParseState::Trailing;
                        words.push(tok.bytes().map(|c| c - b'A').rev().collect());
                    }
                }
                ParseState::Trailing => return Err(PuzzleParseErr::InvalidSuffix(tok)),
            }
        }
        let args = Grid::<u8>::from(&words[..]);
        let args_tags = Grid::<CellTag>::from(
            &(words
                .iter()
                .map(|word| vec![CellTag::Variable; word.len()])
                .collect::<Vec<Vec<_>>>())[..],
        );
        let sums = vec![0; args.cols].into_boxed_slice();
        Ok(Self {
            args,
            args_tags,
            sums,
            env: Environment::new(),
            col: 0,
            row: 0,
            sum: 0,
        })
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    Puzzle::parse(input).ok()?.solve()
}
