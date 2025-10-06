use itertools::Itertools;
use core::fmt;
use std::collections::HashMap;

#[derive(Debug)]
struct Puzzle {
    weights: [isize; 26],
    zero_excluded: [bool; 26],
    letters: Box<[u8]>,
}

enum PuzzleParseErr<'a> {
    InvalidWord(&'a str, char),
    LongWord(&'a str),
    InvalidSuffix(&'a str),
    InvalidSep(&'a str),
    MightOverflow,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &w) in (0..26).zip(self.weights.iter()) {
            if w != 0 {
                writeln!(f, "{} => {}", char::from(b'A' + i), w)?;
            }
        }
        write!(f, "Zero Excluded:")?;
        for i in 0..26u8 {
            if self.zero_excluded[usize::from(i)] {
                write!(f, " {}", char::from(b'A' + i))?;
            }
        }
        writeln!(f)?;
        write!(f, "Letters:")?;
        for &l in self.letters.iter() {
            write!(f, " {}", char::from(b'A' + l))?;
        }
        Ok(())
    }
}
impl Puzzle {
    fn no_trailing_zero(&self, perm: &Vec<u8>) -> bool {
        if let Some(l) = perm.iter().position(|&d| d == 0) {
            !self.zero_excluded[usize::from(self.letters[l])]
        } else {
            true
        }
    }
    fn balances(&self, perm: &Vec<u8>) -> bool {
        self.letters
            .iter()
            .zip(perm.iter())
            .map(|(&l, &d)| self.weights[usize::from(l)] * isize::from(d))
            .sum::<isize>()
            == 0
    }
    fn solve(&self) -> Option<HashMap<char, u8>> {
        eprintln!("Solving {}", self);
        let solution = (0..10u8)
            .permutations(self.letters.len())
            .find(|perm| self.no_trailing_zero(perm) && self.balances(perm))?;
        Some(self.letters
            .iter()
            .zip(solution)
            .map(|(&l, d)| (char::from(b'A' + l), d))
            .collect())
    }
    fn parse(input: &str) -> Result<Self, PuzzleParseErr<'_>> {
        enum ParseState {
            Word,
            Sep,
            Ret,
            Trailing,
        }
        let mut state = ParseState::Word;
        let mut weights = [0isize; 26];
        let mut zero_excluded = [false; 26];
        let mut letter_mask = [false; 26];
        for tok in input.split_ascii_whitespace() {
            match state {
                ParseState::Word => {
                    if let Some(c) = tok.chars().find(|c| !c.is_ascii_uppercase()) {
                        return Err(PuzzleParseErr::InvalidWord(tok, c));
                    } else {
                        state = ParseState::Sep;
                        for (i, b) in tok.bytes().rev().enumerate() {
                            let exp =
                                u32::try_from(i).map_err(|_| PuzzleParseErr::LongWord(tok))?;
                            let letter = usize::from(b - b'A');
                            letter_mask[letter] = true;
                            weights[letter] += 10isize.pow(exp);
                            if i + 1 == tok.len() {
                                zero_excluded[letter] = true;
                            }
                        }
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
                        for (i, b) in tok.bytes().rev().enumerate() {
                            let exp =
                                u32::try_from(i).map_err(|_| PuzzleParseErr::LongWord(tok))?;
                            let letter = usize::from(b - b'A');
                            letter_mask[letter] = true;
                            weights[letter] -= 10isize.pow(exp);
                            if i + 1 == tok.len() {
                                zero_excluded[letter] = true;
                            }
                        }
                    }
                }
                ParseState::Trailing => return Err(PuzzleParseErr::InvalidSuffix(tok)),
            }
        }
        let letters = (0..26u8).filter(|&l| letter_mask[usize::from(l)]).collect();
        Ok(Puzzle {
            weights,
            zero_excluded,
            letters,
        })
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    Puzzle::parse(input).ok()?.solve()
}
