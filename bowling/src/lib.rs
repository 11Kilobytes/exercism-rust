#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Frame {
    Open,
    Spare,
    Strike,
}
impl Frame {
    const fn bonus_throws(&self) -> u8 {
        *self as u8
    }
}
const FRAMES: usize = 10;
const FILL_OFFSET: usize = 2 * FRAMES;
const FILL_THROWS: usize = 3;
const PINS: u16 = 10;
#[derive(Debug)]
pub struct BowlingGame {
    frames: [Option<Frame>; FRAMES],
    throws: [Option<u16>; FILL_OFFSET + FILL_THROWS],
    index: usize,
    pins: u16,
}
impl BowlingGame {
    pub const fn new() -> Self {
        Self {
            frames: [None; FRAMES],
            throws: [None; FILL_OFFSET + FILL_THROWS],
            index: 0,
            pins: PINS,
        }
    }
    fn done(&self) -> bool {
        self.frames[FRAMES - 1]
            .is_some_and(|f| self.index >= FILL_OFFSET + usize::from(f.bonus_throws()))
    }
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.done() {
            Err(Error::GameComplete)
        } else if self.pins < pins {
            Err(Error::NotEnoughPinsLeft)
        } else {
            self.pins -= pins;
            self.throws[self.index] = Some(pins);
            if self.index >= FILL_OFFSET {
                self.index += 1;
                if self.frames[FRAMES - 1] == Some(Frame::Strike) && self.pins == 0 {
                    // Exploits that the only way to get more than one fill ball is
                    // to hit a strike at the end.
                    self.pins = PINS;
                }
            } else if self.pins == 0 {
                if self.index % 2 == 0 {
                    self.frames[self.index / 2] = Some(Frame::Strike);
                    self.index += 2;
                } else {
                    self.frames[self.index / 2] = Some(Frame::Spare);
                    self.index += 1;
                }
                self.pins = PINS;
            } else if self.index % 2 == 1 {
                self.frames[self.index / 2] = Some(Frame::Open);
                self.index += 1;
                self.pins = PINS;
            } else {
                self.index += 1;
            }
            Ok(())
        }
    }
    pub fn score(&self) -> Option<u16> {
        self.done().then(|| {
            (0..FRAMES)
                .map(|f| {
                    self.throws[(2 * f)..(2 * f + 2)]
                        .iter()
                        .flatten()
                        .sum::<u16>()
                        + {
                            let next_frame = self.frames[f].expect(
                                "Since the puzzle is complete all frames should satisfy is_some()",
                            );
                            self.throws[2 * (f + 1)..]
                                .iter()
                                .flatten()
                                .take(usize::from(next_frame.bonus_throws()))
                                .sum::<u16>()
                        }
                })
                .sum()
        })
    }
}
